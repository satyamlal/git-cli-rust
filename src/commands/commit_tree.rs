use crate::utils::hash_and_write;

pub fn execute(tree_sha: String, parent_sha: Option<String>, message: String) {
    let mut commit_content = String::new();
    commit_content.push_str(&format!("tree {}\n", tree_sha));

    if let Some(parent) = parent_sha {
        commit_content.push_str(&format!("parent {}\n", parent));
    }

    let time_now = std::time::SystemTime::now();
    let unix_time = time_now
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time Error!")
        .as_secs();

    let author_line = format!("Satyam Lal <satyam@github.com> {} +0000\n", unix_time);

    commit_content.push_str(&format!("author {}", author_line));
    commit_content.push_str(&format!("commiter {}", author_line));
    commit_content.push_str("\n");
    commit_content.push_str(&message);
    commit_content.push_str("\n");

    let header = format!("commit {}\0", commit_content.len());
    let mut payload = header.into_bytes();

    payload.extend(commit_content.as_bytes());

    let final_hash = hash_and_write(&payload);
    let final_hash_hex = hex::encode(final_hash);

    println!("{}", final_hash_hex);
}
