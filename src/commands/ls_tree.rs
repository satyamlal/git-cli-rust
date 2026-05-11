use flate2::read::ZlibDecoder;
use std::{fs::File, io::Read};

pub fn execute(name_only: bool, hash: String) {
    let dir = &hash[0..2];
    let file_name = &hash[2..];
    let path = format!(".git/objects/{}/{}", dir, file_name);

    let file = File::open(&path).expect("Unable to find object tree!");
    let mut decoder = ZlibDecoder::new(file);
    let mut decompressed_data = Vec::new();

    decoder
        .read_to_end(&mut decompressed_data)
        .expect("Decompression Failed!");

    let header_null_pos = decompressed_data
        .iter()
        .position(|&b| b == 0)
        .expect("Header null byte missing!");

    let mut i = header_null_pos + 1;

    while i < decompressed_data.len() {
        let space_pos = decompressed_data[i..]
            .iter()
            .position(|&b| b == b' ')
            .expect("Space delimiter missing!")
            + i;
        let null_pos = decompressed_data[space_pos..]
            .iter()
            .position(|&b| b == 0)
            .expect("Entry null byte missing!")
            + space_pos;

        if name_only {
            let name_bytes = &decompressed_data[space_pos + 1..null_pos];
            let name = std::str::from_utf8(name_bytes).expect("Invalid UTF-8 in name!");

            println!("{}", name);
        }
        i = null_pos + 1 + 20;
    }
}
