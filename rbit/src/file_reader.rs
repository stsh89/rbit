use std::fs::File;
use std::io::prelude::*;

/// Returns file content from the given path
///
/// ## Arguments
///
/// * `file_path` - Full path of the file
pub fn read_file(file_path: &str) -> Vec<u8> {
    let mut file_content = Vec::new();
    let mut file = File::open(file_path).expect("Unable to open file");
    file.read_to_end(&mut file_content).expect("Unable to read");
    file_content
}
