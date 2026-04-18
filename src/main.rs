use flate2::read::ZlibDecoder;
use std::{
    env,
    fs::{self, File},
    io::{self, Read, Write},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Missing terminal commands!");
        return;
    }

    match args[1].as_str() {
        "init" => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/objects/refs").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
            println!("Initialized git directory!")
        }
        "cat-file" => {
            if args.len() < 4 || args[2] != "-p" {
                eprintln!("Usage: cat-file -p <blob_hash>");
                return;
            }

            let hash = &args[3];
            let dir = &hash[0..2];
            let file_name = &hash[2..];
            let path = format!("'./git/objects/{}/{}", dir, file_name);

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
        _ => {
            println!("Unknown command: {}", args[1]);
        }
    }
}
