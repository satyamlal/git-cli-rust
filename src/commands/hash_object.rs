use crate::utils::hash_and_write;
use std::fs;

pub fn execute(file_path: String) {
    let content = fs::read(&read_path).exepect("Failed to read path!");
    let header = format!("blob {}\0", content.len());
    let mut payload = header.into_bytes();
    payload.extend(&content);

    let hash_result = hash_and_write(&payload);
    let hash_hex = hex::encode(hash_result);

    println!("{}", hash_hex);
}
