use syn::{File, Item, Ident, Stmt, Expr, BinOp, visit_mut::Visit};
use std::collections::{HashMap, HashSet};
use crate::utils::{read_code, parse_code};

pub fn refactor_code(file_path: &str) -> Result<String, &'static str> {
    let code = read_code(file_path).ok_or("Failed to read code")?;
    let mut file = parse_code(&code).ok_or("Failed to parse code")?;

    let mut renamer = IdentifierRenamer::new();
    renamer.visit_file_mut(&mut file);

    let mut extractor = FunctionExtractor::default();
    extractor.visit_file_mut(&mut file);

    let mut dead_code_remover = DeadCodeRemover::default();
    dead_code_remover.mark_used_functions(&file);
    dead_code_remover.visit_file_mut(&mut file);
    dead_code_remover.remove_unused_functions(&mut file);

    simplify_expressions(&mut file);
    reorder_functions(&mut file);

    let refactored_code = quote::quote!(#file).to_string();
    Ok(refactored_code)
}

// --- Identifier Renamer ---
struct IdentifierRenamer {
    rename_map: HashMap<String, String>,
    used_identifiers: HashSet<String>,
}

impl IdentifierRenamer {
    fn new() -> Self {
        Self {
            rename_map: HashMap::new(),
            used_identifiers: HashSet::new(),
        }
    }
    
    fn generate_unique_name(&mut self, base_name: &str) -> String {
        let mut new_name = base_name.to_string();
        let mut counter = 1;
        while self.used_identifiers.contains(&new_name) {
            new_name = format!("{}_{}", base_name, counter);
            counter += 1;
        }
        self.used_identifiers.insert(new_name.clone());
        new_name
    }
}

impl VisitMut for IdentifierRenamer {
    fn visit_ident_mut(&mut self, ident: &mut Ident) {
        let ident_str = ident.to_string();
        let new_name = self.rename_map
            .entry(ident_str.clone())
            .or_insert_with(|| self.generate_unique_name(&ident_str));
        *ident = Ident::new(new_name, ident.span());
    }
}

// --- Function Extractor ---
#[derive(Default)]
struct FunctionExtractor {
    extracted_functions: Vec<Item>,
}

impl VisitMut for FunctionExtractor {
    fn visit_item_fn_mut(&mut self, func: &mut syn::ItemFn) {
        if func.block.stmts.len() > 10 {
            let name = func.sig.ident.to_string();
            let new_func_name = format!("{}_extracted", name);
            let new_func = syn::ItemFn {
                attrs: func.attrs.clone(),
                vis: func.vis.clone(),
                sig: syn::Signature {
                    ident: Ident::new(&new_func_name, func.sig.ident.span()),
                    ..func.sig.clone()
                },
                block: Box::new(func.block.clone()),
            };
            func.block.stmts.clear();
            let call_expr: Expr = syn::parse_quote! {
                #new_func_name();
            };
            func.block.stmts.push(Stmt::Expr(call_expr));
            self.extracted_functions.push(Item::Fn(new_func));
        }
    }

    fn visit_file_mut(&mut self, file: &mut File) {
        syn::visit_mut::visit_file_mut(self, file);
        file.items.append(&mut self.extracted_functions);
    }
}

// --- Dead Code Remover ---
#[derive(Default)]
struct DeadCodeRemover {
    used_functions: HashSet<String>,
}

impl DeadCodeRemover {
    fn mark_used_functions(&mut self, file: &File) {
        for item in &file.items {
            if let Item::Fn(func) = item {
                self.visit_block(&func.block);
            }
        }
    }

    fn visit_block(&mut self, block: &syn::Block) {
        for stmt in &block.stmts {
            if let Stmt::Expr(Expr::Call(call_expr)) = stmt {
                if let Expr::Path(expr_path) = &*call_expr.func {
                    if let Some(ident) = expr_path.path.get_ident() {
                        self.used_functions.insert(ident.to_string());
                    }
                }
            }
        }
    }

    fn remove_unused_functions(&mut self, file: &mut File) {
        file.items.retain(|item| {
            if let Item::Fn(func) = item {
                self.used_functions.contains(&func.sig.ident.to_string())
            } else {
                true
            }
        });
    }
}

impl VisitMut for DeadCodeRemover {
    fn visit_item_fn_mut(&mut self, func: &mut syn::ItemFn) {
        self.used_functions.insert(func.sig.ident.to_string());
        syn::visit_mut::visit_item_fn_mut(self, func);
    }
}

// --- Expression Simplification ---
fn simplify_expressions(file: &mut File) {
    for item in &mut file.items {
        if let Item::Fn(func) = item {
            for stmt in &mut func.block.stmts {
                if let Stmt::Expr(Expr::Binary(expr)) = stmt {
                    simplify_binary_expression(expr);
                }
            }
        }
    }
}

fn simplify_binary_expression(expr: &mut syn::ExprBinary) {
    if let (Expr::Lit(left_lit), Expr::Lit(right_lit)) = (&*expr.left, &*expr.right) {
        if let (syn::Lit::Int(left_int), syn::Lit::Int(right_int)) = (&left_lit.lit, &right_lit.lit) {
            let left_value = left_int.base10_parse::<i64>().unwrap_or_default();
            let right_value = right_int.base10_parse::<i64>().unwrap_or_default();
            let result = match expr.op {
                BinOp::Add(_) => Some(left_value + right_value),
                BinOp::Sub(_) => Some(left_value - right_value),
                BinOp::Mul(_) => Some(left_value * right_value),
                BinOp::Div(_) => if right_value != 0 { Some(left_value / right_value) } else { None },
                _ => None,
            };
            if let Some(result) = result {
                *expr.left = Box::new(Expr::Lit(syn::ExprLit {
                    attrs: vec![],
                    lit: syn::Lit::Int(syn::LitInt::new(&result.to_string(), expr.span())),
                }));
                *expr.right = Box::new(Expr::Lit(syn::LitInt::new("0", expr.span()).into()));
            }
        }
    }
}

// --- Function Reordering ---
fn reorder_functions(file: &mut File) {
    file.items.sort_by(|a, b| {
        let a_is_public = if let Item::Fn(func) = a {
            !func.sig.ident.to_string().starts_with('_')
        } else {
            false
        };
        let b_is_public = if let Item::Fn(func) = b {
            !func.sig.ident.to_string().starts_with('_')
        } else {
            false
        };
        a_is_public.cmp(&b_is_public)
    });
}
