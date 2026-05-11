use std::{fs, io::Write};

pub fn execute(url: String, directory: String) {
    let client = reqwest::blocking::Client::new();

    let refs_url = format!("{}/info/refs?service=git-upload-pack", url);
    let resp = client
        .get(&refs_url)
        .send()
        .expect("Failed to send a GET request!");

    let body_bytes = resp.bytes().expect("Failed to read response!");
    let body_str = String::from_utf8_lossy(&body_bytes);

    let head_pos = body_str.find(" HEAD").expect("HEAD not found in refs!");
    let targets_hash = &body_str[head_pos - 40..head_pos];

    let pack_url = format!("{}/git-upload-pack", url);
    let request_body = format!("0032 want {}\n00000092done\n", targets_hash);

    let pack_resp = client
        .post(&pack_url)
        .header("Content-Type", "application/x-git-upload-pack-request")
        .body(request_body)
        .send()
        .expect("Failed to request packfile!");

    let pack_data = pack_resp.bytes().expect("Failed to download packfile!");

    fs::create_dir_all(&directory).expect("Failed to create target directory!");
    let git_dir = format!("{}/.git", directory);

    fs::create_dir_all(format!("{}/object", git_dir))
        .expect("FAILED to create objects folder inside .git directory!");
    fs::create_dir_all(format!("{}/refs", git_dir))
        .expect("FAILED to create refs folder inside .git directory!");
    fs::write(format!("{}/HEAD", git_dir), "ref: refs/heads/main\n").expect("HEAD Failed!");

    let pack_bytes = if pack_data.starts_with(b"0008NAK\n") {
        &pack_data[8..]
    } else {
        &pack_data[..]
    };

    let mut unpack_cmd = std::process::Command::new("git")
        .arg("unpack-objects")
        .current_dir(&directory)
        .env("GIT-DIR", ".git")
        .stdin(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn git unpack-objects");

    unpack_cmd
        .stdin
        .as_mut()
        .expect("Failed to open stdin!")
        .write_all(pack_bytes)
        .expect("Failed to write pack data!");

    unpack_cmd
        .wait()
        .expect("Failed to wait on git unpack-objects!");

    std::process::Command::new("git")
        .args(["--work=.", "--git-dir=.git", "checkout", targets_hash])
        .current_dir(&directory)
        .status()
        .expect("Failed to checkout files!");

    println!(
        "Download and checkout complete! Packfile size: {} bytes",
        pack_data.len()
    );
}
