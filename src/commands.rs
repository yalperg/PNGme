use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use url::Url;

use crate::args::Input;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

fn get_png_bytes(input: &Input) -> Result<Vec<u8>> {
    match input {
        Input::FilePath(path) => {
            fs::read(path).with_context(|| format!("Failed to read file: {:?}", path))
        }
        Input::Url(url_str) => {
            let url = Url::parse(url_str).context("Invalid URL")?;
            let resp = reqwest::blocking::get(url.as_str()).context("Failed to download file")?;
            let bytes = resp.bytes().context("Failed to read response body")?;
            Ok(bytes.to_vec())
        }
    }
}

pub fn encode(
    input: Input,
    chunk_type: String,
    message: String,
    output_file: Option<PathBuf>,
) -> Result<()> {
    let file_bytes = get_png_bytes(&input)?;

    let mut png = Png::try_from(file_bytes.as_slice())
        .map_err(anyhow::Error::msg)
        .context("Failed to parse PNG data")?;

    let chunk_type = ChunkType::from_str(&chunk_type)
        .map_err(anyhow::Error::msg)
        .context("Invalid chunk type")?;

    let chunk = Chunk::new(chunk_type, message.into_bytes());

    png.append_chunk(chunk);

    let output_path = output_file.unwrap_or_else(|| {
        match &input {
            Input::FilePath(path) => path.with_extension("png"),
            Input::Url(_) => PathBuf::from("output.png"),
        }
    });

    fs::write(&output_path, png.as_bytes())
        .with_context(|| format!("Failed to write output file: {:?}", output_path))?;

    println!("Message encoded successfully to {:?}", output_path);
    Ok(())
}

pub fn decode(input: Input, chunk_type: String) -> Result<()> {
    let file_bytes = get_png_bytes(&input)?;

    let png = Png::try_from(file_bytes.as_slice())
        .map_err(anyhow::Error::msg)
        .context("Failed to parse PNG data")?;

    if let Some(chunk) = png.chunk_by_type(&chunk_type) {
        let message = chunk
            .data_as_string()
            .with_context(|| format!("Failed to decode chunk data as string: {}", chunk_type))?;
        println!("Decoded message from chunk '{}': {}", chunk_type, message);
    } else {
        println!("No chunk of type '{}' found in the PNG file.", chunk_type);
    }

    Ok(())
}

pub fn remove(input: PathBuf, chunk_type: String) -> Result<()> {
    let file_bytes =
        fs::read(&input).with_context(|| format!("Failed to read file: {:?}", input))?;

    let mut png = Png::try_from(file_bytes.as_slice())
        .map_err(anyhow::Error::msg)
        .with_context(|| format!("Failed to parse PNG file: {:?}", input))?;

    png.remove_first_chunk(&chunk_type)
        .map_err(anyhow::Error::msg)
        .with_context(|| format!("Failed to remove chunk of type: {}", chunk_type))?;

    fs::write(&input, png.as_bytes())
        .with_context(|| format!("Failed to write updated PNG file: {:?}", input))?;

    println!(
        "Chunk '{}' removed successfully from {:?}",
        chunk_type, input
    );
    Ok(())
}

pub fn print(input: Input) -> Result<()> {
    let file_bytes = get_png_bytes(&input)?;

    let png = Png::try_from(file_bytes.as_slice())
        .map_err(anyhow::Error::msg)
        .context("Failed to parse PNG data")?;

    if png.chunks.is_empty() {
        println!("No chunks found in the PNG file.");
    } else {
        println!("Chunks in the PNG file:");
        for chunk in &png.chunks {
            println!("{}", chunk);
        }
    }

    Ok(())
}
