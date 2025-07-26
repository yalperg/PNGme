use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Clone)]
pub enum Input {
    FilePath(PathBuf),
    Url(String),
}

impl Input {
    pub fn from_args(input: String, is_url: bool) -> Self {
        if is_url {
            Input::Url(input)
        } else {
            Input::FilePath(PathBuf::from(input))
        }
    }
}

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Encode {
        #[arg(short, long)]
        url: bool,
        input: String,
        chunk_type: String,
        message: String,
        #[arg(short, long)]
        output_file: Option<PathBuf>,
    },
    Decode {
        #[arg(short, long)]
        url: bool,
        input: String,
        chunk_type: String,
    },
    Remove {
        input: PathBuf,
        chunk_type: String,
    },
    Print {
        #[arg(short, long)]
        url: bool,
        input: String,
    }
}
