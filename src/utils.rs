use syn::{parse_file, File, Item};

pub fn read_code(file_path: &str) -> String {
    std::fs::read_to_string(file_path).expect("Unable to read file")
}

pub fn parse_code(code: &str) -> File {
    parse_file(code).expect("Failed to parse Rust code")
}
