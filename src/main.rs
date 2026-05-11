use clap::Parser;
use cli::Cli;

mod cli;
mod commands;
mod utils;

fn main() {
    let cli = Cli::parse();
    commands::execute(cli.command);
}
