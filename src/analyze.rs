use syn::{parse_file, File, Item};

pub fn analyze_code(code: &str) {
    let parsed: File = match parse_file(code) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
            return;
        },
    };

    let mut num_functions = 0;
    let mut num_structs = 0;
    let mut num_enums = 0;
    let mut num_impls = 0;

    for item in parsed.items {
        match item {
            Item::Fn(_) => num_functions += 1,
            Item::Struct(_) => num_structs += 1,
            Item::Enum(_) => num_enums += 1,
            Item::Impl(_) => num_impls += 1,
            _ => (),
        }
    }

    println!("Number of functions: {}", num_functions);
    println!("Number of structs: {}", num_structs);
    println!("Number of enums: {}", num_enums);
    println!("Number of impl blocks: {}", num_impls);
}
