use anyhow::Result;
use clap::Parser;
use pngme::{
    args::{Args, Commands, Input},
    *,
};

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Encode {
            input,
            url,
            chunk_type,
            message,
            output_file,
            password,
        } => {
            let input = Input::from_args(input, url);
            commands::encode(input, chunk_type, message, output_file, password)
        }
        Commands::Decode {
            input,
            url,
            chunk_type,
            password,
        } => {
            let input = Input::from_args(input, url);
            commands::decode(input, chunk_type, password)
        }
        Commands::Remove { input, chunk_type } => commands::remove(input, chunk_type),
        Commands::Print { input, url } => {
            let input = Input::from_args(input, url);
            commands::print(input)
        }
    }
}
