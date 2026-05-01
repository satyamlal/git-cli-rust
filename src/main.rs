use clap::{Parser, Subcommand};
use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use sha1::{Digest, Sha1};
use std::{
    env,
    fs::{self, File},
    io::{self, Read, Write},
    path::Path,
};

#[derive(Parser)]
#[command(name = "mygit", about = "Git clone CLI app")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    CatFile {
        #[arg(short = 'p')]
        hash: String,
    },
    HashObject {
        #[arg(short = 'w')]
        file_path: String,
    },
    LsTree {
        #[arg(long = "name-only")]
        name_only: bool,
        hash: String,
    },
    WriteTree,
}

fn hash_and_write(payload: &[u8]) -> [u8; 20] {
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

fn write_tree(dir: &Path) -> [u8; 20] {
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
            let sha = write_tree(&path);
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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/objects/refs").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
            println!("Initialized git directory!")
        }
        Commands::CatFile { hash } => {
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
        Commands::HashObject { file_path } => {
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
        Commands::LsTree { name_only, hash } => {
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
        Commands::WriteTree => {
            let current_dir = env::current_dir().expect("Failed to get current directory path!");
            let final_sha_bytes = write_tree(&current_dir);
            let final_sha_hex = hex::encode(final_sha_bytes);

            println!("{}", final_sha_hex);
        }
    }
}
