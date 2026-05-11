use flate2::read::ZlibDecoder;
use std::{
    fs::File,
    io::{self, Read, Write},
};

pub fn execute(hash: String) {
    let dir = &hash[0..2];
    let file_name = &hash[2..];
    let path = format!("./git/objects/{}/{}", dir, file_name);

    let file = File::open(&path).expect("Unable to find blob!");
    let mut decoder = ZlibDecoder::new(file);
    let mut decompressed_data = Vec::new();

    decoder
        .read_to_end(&mut decompressed_data)
        .expect("Decompression failed!");

    let null_pos = decompressed_data
        .iter()
        .position(|&b| b == 0)
        .expect("Null byte missing!");

    let content = &decompressed_data[null_pos + 1..];

    io::stdout()
        .write_all(content)
        .expect("Unable to write to file!");
}
