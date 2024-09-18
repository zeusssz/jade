use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::process::{Command, exit};
use std::time::Instant;

pub fn simulate_run(file: &str) {
    let temp_dir = "temp";
    if let Err(e) = fs::create_dir_all(temp_dir) {
        eprintln!("Failed to create temp directory: {}", e);
        exit(1);
    }

    let temp_file = format!("{}/temp.rs", temp_dir);
    let code = match fs::read_to_string(file) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Unable to read file {}: {}", file, e);
            exit(1);
        }
    };

    if let Err(e) = fs::write(&temp_file, &code) {
        eprintln!("Unable to write temp file: {}", e);
        cleanup(temp_dir);
        exit(1);
    }

    if let Err(_) = compile_rust_code(&temp_file, temp_dir) {
        cleanup(temp_dir);
        exit(1);
    }

    run_executable(temp_dir);
    cleanup(temp_dir);
}

// Compile the Rust code
fn compile_rust_code(temp_file: &str, temp_dir: &str) -> Result<(), ()> {
    let output = Command::new("rustc")
        .arg(temp_file)
        .arg("--out-dir")
        .arg(temp_dir)
        .output()
        .expect("Failed to compile Rust code");

    if !output.status.success() {
        eprintln!(
            "Compilation failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        return Err(());
    }
    Ok(())
}

// Run the compiled executable
fn run_executable(temp_dir: &str) {
    let exe_path = format!("{}/temp", temp_dir);
    let start = Instant::now();
    let output = Command::new(&exe_path)
        .output()
        .expect("Failed to execute compiled code");
    let duration = start.elapsed();

    if !output.status.success() {
        eprintln!(
            "Runtime error:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    } else {
        println!(
            "Program output:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }

    println!("Execution time: {:?}", duration);
}

// Cleanup temporary files
fn cleanup(temp_dir: &str) {
    if let Err(e) = fs::remove_dir_all(temp_dir) {
        eprintln!("Failed to clean up temp directory: {}", e);
    }
}

// Code analysis functionality
use syn::{parse_file, File, Item};

pub fn analyze_code(code: &str) {
    let parsed: File = match parse_file(code) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
            return;
        }
    };

    let mut analysis = CodeAnalysis::default();

    for item in parsed.items {
        match item {
            Item::Fn(_) => analysis.num_functions += 1,
            Item::Struct(_) => analysis.num_structs += 1,
            Item::Enum(_) => analysis.num_enums += 1,
            Item::Impl(_) => analysis.num_impls += 1,
            _ => (),
        }
    }

    analysis.print_summary();
}

#[derive(Default)]
struct CodeAnalysis {
    num_functions: usize,
    num_structs: usize,
    num_enums: usize,
    num_impls: usize,
}

impl CodeAnalysis {
    fn print_summary(&self) {
        println!("Code Analysis:");
        println!("Number of functions: {}", self.num_functions);
        println!("Number of structs: {}", self.num_structs);
        println!("Number of enums: {}", self.num_enums);
        println!("Number of impl blocks: {}", self.num_impls);
    }
}
