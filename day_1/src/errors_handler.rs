use std::io::{Error, ErrorKind};

pub fn handle_file_error(e: Error, filename: &str) -> ! {
    match e.kind() {
        ErrorKind::NotFound => {
            eprintln!("Error: File '{}' not found", filename);
            eprintln!("Make sure you're running from the correct directory");
        }
        ErrorKind::PermissionDenied => {
            eprintln!("Error: Permission denied reading '{}'", filename);
            eprintln!("Check file permissions");
        }
        _ => {
            eprintln!("Error reading file: {}", e);
        }
    }
    std::process::exit(1);
}
