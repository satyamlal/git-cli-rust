use clap::{Parser, Subcommand};
use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use sha1::{Digest, Sha1};
use std::{
    env,
    fs::{self, File},
    io::{self, Read, Write},
};

#[derive(Parser)]
#[command(name = "mygit", about = "Git clone CLI app")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[command(subcommand)]
enum Commands {
    Init,
    CatFile {
        #[arg(short = "-p")]
        hash: String,
    },
    HashObject {
        #[arg(short = "-w")]
        file_path: String,
    },
}

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
        "hash-object" => {
            if args.len() < 4 || args[2] != "-w" {
                eprintln!("Usage: hash-object -w <file>");
                return;
            }

            let file_path = &args[3];
            let content = fs::read(&file_path).expect("Failed to read!");
            let header = format!("blob {}\0", content.len());

            let mut payload = header.into_bytes();
            payload.extend(&content);

            let mut hasher = Sha1::new();
            hasher.update(&payload);
            let hash_result = hasher.finalize();

            let hash_hex = hex::encode(hash_result);
            let dir = &hash_hex[0..2];
            let file_name = &hash_hex[2..];
            let dir_path = format!(".git/objects/{}", dir);

            fs::create_dir_all(&dir_path).expect("Failed to create object directory!");

            let object_path = format!("{}/{}", dir_path, file_name);

            let file = File::create(&object_path).expect("Failed to create object file!");
            let mut encoder = ZlibEncoder::new(file, Compression::default());

            encoder
                .write_all(&payload)
                .expect("Failed to write compressed data!");
            encoder
                .finish()
                .expect("Failed to finish compression stream!");

            println!("{}", hash_hex);
        }
        _ => {
            println!("Unknown command: {}", args[1]);
        }
    }
}
