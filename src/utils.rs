use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use sha1::{Digest, Sha1};
use std::{
    fs::{self, File},
    io::{self, Read, Write},
};

pub fn hash_and_write(payload: &[u8]) -> [u8; 20] {
    let mut hasher = Sha1::new();
    hasher.update(payload);
    let hash_result = hasher.finalize();

    let mut sha_bytes = [0u8; 20];
    sha_bytes.copy_from_slice(&hash_result);

    let hash_hex = hex::encode(hash_result);
    let dir = &hash_hex[0..2];
    let file_name = &hash_hex[2..];
    let dir_path = format!(".git/objects/{}", dir);

    fs::create_dir_all(&dir_path).expect("Failed to create object directory!");
    let object_path = format!("{}/{}", dir_path, file_name);
    let file = File::create(&object_path).expect("Failed to create object file!");

    let mut encoder = ZlibEncoder::new(file, Compression::default());

    encoder
        .write_all(payload)
        .expect("Failed to write compressed data!");
    encoder
        .finish()
        .expect("Failed to finish compression stream!");

    sha_bytes
}
