use std::io::prelude::*;
use std::fs::File;

pub fn read_all_bytes(path: &str) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes);
    bytes
}