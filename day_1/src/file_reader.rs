use std::fs::read_to_string;
use std::io::Error;
pub fn read_input_file(filename: &str) -> Result<String, Error> {
    read_to_string(filename)
}
