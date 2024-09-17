use clap::{Arg, Command};
use std::fs;
use std::path::Path;

mod analyze;
mod metrics;
mod refactor;
mod utils;

fn main() {
    let matches = Command::new("Jade")
        .version("0.1.0")
        .author("zeusssz")
        .about("A Rust code refactoring tool")
        .arg(
            Arg::new("refactor")
                .long("refactor")
                .takes_value(true)
                .help("Refactor the given file"),
        )
        .arg(
            Arg::new("analyze")
                .long("analyze")
                .takes_value(true)
                .help("Analyze the given file"),
        )
        .arg(
            Arg::new("metrics")
                .long("metrics")
                .takes_value(true)
                .help("Compute metrics for the given file by simulating a run"),
        )
        .get_matches();

    if let Some(file) = matches.value_of("refactor") {
        if Path::new(file).exists() {
            let refactored_code = refactor::refactor_code(file);
            let refactored_file = format!("{}_refactored.rs", file.trim_end_matches(".rs"));
            fs::write(refactored_file, refactored_code).expect("Unable to write file");
            println!("Refactored code written to: {}", refactored_file);
        } else {
            eprintln!("File does not exist: {}", file);
        }
    } else if let Some(file) = matches.value_of("analyze") {
        if Path::new(file).exists() {
            let code = fs::read_to_string(file).expect("Unable to read file");
            analyze::analyze_code(&code);
        } else {
            eprintln!("File does not exist: {}", file);
        }
    } else if let Some(file) = matches.value_of("metrics") {
        if Path::new(file).exists() {
            metrics::simulate_run(file);
        } else {
            eprintln!("File does not exist: {}", file);
        }
    } else {
        eprintln!("No valid command was provided. Use `jade --help` for usage information.");
    }
}
