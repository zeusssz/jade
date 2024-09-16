use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::process::{Command, exit};
use std::time::Instant;

pub fn simulate_run(file: &str) {
    let temp_dir = "temp";
    fs::create_dir_all(temp_dir).expect("Failed to create temp directory");

    let temp_file = format!("{}/temp.rs", temp_dir);
    let code = fs::read_to_string(file).expect("Unable to read file");
    fs::write(&temp_file, code).expect("Unable to write temp file");

    let output = Command::new("rustc")
        .arg(temp_file)
        .arg("--out-dir")
        .arg(temp_dir)
        .output()
        .expect("Failed to compile Rust code");

    if !output.status.success() {
        eprintln!("Compilation failed:\n{}", String::from_utf8_lossy(&output.stderr));
        exit(1);
    }

    let start = Instant::now();
    let exe_path = format!("{}/temp", temp_dir);
    let output = Command::new(exe_path)
        .output()
        .expect("Failed to execute the compiled code");
    let duration = start.elapsed();

    if !output.status.success() {
        eprintln!("Runtime error:\n{}", String::from_utf8_lossy(&output.stderr));
    } else {
        println!("Program output:\n{}", String::from_utf8_lossy(&output.stdout));
    }

    println!("Execution time: {:?}", duration);

    fs::remove_file(exe_path).expect("Failed to delete temp binary");
    fs::remove_file(temp_file).expect("Failed to delete temp source file");
    fs::remove_dir(temp_dir).expect("Failed to delete temp directory");
}
