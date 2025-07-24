use pngme::{args::{Args, Commands}, *};
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Encode {
            file_path,
            chunk_type,
            message,
            output_file,
        } => commands::encode(file_path, chunk_type, message, output_file),
        Commands::Decode {
            file_path,
            chunk_type,
        } => commands::decode(file_path, chunk_type),
        Commands::Remove {
            file_path,
            chunk_type,
        } => commands::remove(file_path, chunk_type),
        Commands::Print { file_path } => commands::print(file_path),
    }
}
