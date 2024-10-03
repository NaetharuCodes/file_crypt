// FILE READING AND WRITING OPS

use std::fs::{self, File};
use std::io::Write;

pub fn read_file(filepath: &str) -> Result<Vec<u8>, std::io::Error> {
    fs::read(filepath)
}

pub fn write_file(filepath: &str, data: &[u8]) -> Result<(), std::io::Error> {
    let mut file = File::create(filepath)?;
    file.write_all(data)
}
