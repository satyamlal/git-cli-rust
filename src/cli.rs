use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mygit", about = "Git clone CLI app")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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
    CommitTree {
        tree_sha: String,
        #[arg(short = 'p')]
        parent_sha: Option<String>,
        #[arg(short = 'm')]
        message: String,
    },
    Clone {
        url: String,
        directory: String,
    },
}
