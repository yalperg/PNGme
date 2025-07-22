mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use clap::Parser;
use args::{Args, Commands};
use anyhow::Result;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Encode { file_path, chunk_type, message, output_file } => {
            todo!();
        }
        Commands::Decode { file_path, chunk_type } => {
            todo!();
        }
        Commands::Remove { file_path, chunk_type } => {
            todo!();
        }
        Commands::Print { file_path } => {
            todo!();
        }
    }
}