use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

pub fn encode(
    file_path: PathBuf,
    chunk_type: String,
    message: String,
    output_file: Option<PathBuf>,
) -> Result<()> {
    let file_bytes =
        fs::read(&file_path).with_context(|| format!("Failed to read file: {:?}", file_path))?;

    let mut png = Png::try_from(file_bytes.as_slice())
        .map_err(anyhow::Error::msg)
        .with_context(|| format!("Failed to parse PNG file: {:?}", file_path))?;

    let chunk_type = ChunkType::from_str(&chunk_type)
        .map_err(anyhow::Error::msg)
        .with_context(|| format!("Invalid chunk type: {}", chunk_type))?;

    let chunk = Chunk::new(chunk_type, message.into_bytes());

    png.append_chunk(chunk);

    let output_path = output_file.unwrap_or_else(|| file_path.with_extension("png"));

    fs::write(&output_path, png.as_bytes())
        .with_context(|| format!("Failed to write output file: {:?}", output_path))?;

    println!("Message encoded successfully to {:?}", output_path);
    Ok(())
}

pub fn decode(file_path: PathBuf, chunk_type: String) -> Result<()> {
    let file_bytes =
        fs::read(&file_path).with_context(|| format!("Failed to read file: {:?}", file_path))?;

    let png = Png::try_from(file_bytes.as_slice())
        .map_err(anyhow::Error::msg)
        .with_context(|| format!("Failed to parse PNG file: {:?}", file_path))?;

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

pub fn remove(file_path: PathBuf, chunk_type: String) -> Result<()> {
    let file_bytes =
        fs::read(&file_path).with_context(|| format!("Failed to read file: {:?}", file_path))?;

    let mut png = Png::try_from(file_bytes.as_slice())
        .map_err(anyhow::Error::msg)
        .with_context(|| format!("Failed to parse PNG file: {:?}", file_path))?;

    png.remove_first_chunk(&chunk_type)
        .map_err(anyhow::Error::msg)
        .with_context(|| format!("Failed to remove chunk of type: {}", chunk_type))?;

    fs::write(&file_path, png.as_bytes())
        .with_context(|| format!("Failed to write updated PNG file: {:?}", file_path))?;

    println!(
        "Chunk '{}' removed successfully from {:?}",
        chunk_type, file_path
    );
    Ok(())
}

pub fn print(file_path: PathBuf) -> Result<()> {
    let file_bytes =
        fs::read(&file_path).with_context(|| format!("Failed to read file: {:?}", file_path))?;

    let png = Png::try_from(file_bytes.as_slice())
        .map_err(anyhow::Error::msg)
        .with_context(|| format!("Failed to parse PNG file: {:?}", file_path))?;

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
