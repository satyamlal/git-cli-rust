use crate::utils::hash_and_write;
use std::{env, fs, path::Path};

pub fn execute() {
    let current_dir = env::current_dir().expect("Failed to get current directory!");
    let final_sha_bytes = write_tree_recursive(&current_dir);
    let final_sha_hex = hex::encode(final_sha_bytes);

    println!("{}", final_sha_hex);
}

pub fn write_tree_recursive(dir: &Path) -> [u8; 20] {
    let mut entries = Vec::new();
    let read_dir = fs::read_dir(dir).expect("Failed to read directory!");

    for entry_result in read_dir {
        let entry = entry_result.expect("Failed to read entry!");

        let file_name = entry
            .file_name()
            .into_string()
            .expect("Invalid UTF-8 filename!");

        if entry.file_name() == ".git" {
            continue;
        }

        let path = entry.path();
        let metadata = entry.metadata().expect("Failed to get metadata!");

        if metadata.is_dir() {
            let sha = write_tree_recursive(&path);
            entries.push((file_name, "40000".to_string(), sha));
        } else {
            let content = fs::read(&path).expect("Failed to read file!");
            let header = format!("blob {}\0", content.len());

            let mut payload = header.into_bytes();
            payload.extend(&content);

            let sha = hash_and_write(&payload);
            entries.push((file_name, "100644".to_string(), sha));
        }
    }
    entries.sort_by(|a, b| a.0.cmp(&b.0));

    let mut tree_content = Vec::new();
    for (name, mode, sha) in entries {
        tree_content.extend_from_slice(format!("{} {}\0", mode, name).as_bytes());
        tree_content.extend_from_slice(&sha);
    }

    let header = format!("tree {}\0", tree_content.len());
    let mut final_payload = header.into_bytes();
    final_payload.extend(tree_content);

    hash_and_write(&final_payload)
}
