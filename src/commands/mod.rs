pub mod cat_file;
pub mod clone;
pub mod commit_tree;
pub mod hash_object;
pub mod init;
pub mod ls_tree;
pub mod write_tree;

use crate::cli::Commands;

pub fn execute(command: Commands) {
    match command {
        Commands::Init => init::execute(),
        Commands::CatFile { hash } => cat_file::execute(hash),
        Commands::HashObject { file_path } => hash_object::execute(file_path),
        Commands::LsTree { name_only, hash } => ls_tree::execute(name_only, hash),
        Commands::WriteTree => write_tree::execute(),
        Commands::CommitTree {
            tree_sha,
            parent_sha,
            message,
        } => commit_tree::execute(tree_sha, parent_sha, message),
        Commands::Clone { url, directory } => clone::execute(url, directory),
    }
}
