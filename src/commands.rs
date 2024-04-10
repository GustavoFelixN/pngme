use std::error;
use std::{path::PathBuf, str::FromStr};

use crate::{chunk::Chunk, chunk_type::ChunkType, png::Png};

fn read_png(path: &PathBuf) -> Result<Png, Box<dyn std::error::Error>> {
    let png_file = std::fs::read(path)?;
    let png = Png::try_from(png_file.as_ref())?;
    Ok(png)
}

pub fn save_to_file(path: PathBuf, png: Png) -> Result<(), std::io::Error> {
    std::fs::write(path, png.as_bytes())?;
    Ok(())
}

pub fn encode_message(
    path: &PathBuf,
    c_type: String,
    message: String,
) -> Result<Png, Box<dyn error::Error>> {
    let mut file: Png = read_png(path)?;
    let type_chunk: ChunkType = ChunkType::from_str(c_type.as_str())?;
    let new_chunk: Chunk = Chunk::new(type_chunk, message.as_bytes().to_vec());

    file.append_chunk(new_chunk);

    Ok(file)
}

pub fn decode_message(path: &PathBuf, c_type: String) -> Result<String, Box<dyn error::Error>> {
    let file: Png = read_png(&path)?;
    if let Some(chunk) = file.chunk_by_type(c_type.as_str()) {
        Ok(chunk.to_string())
    } else {
        Err("Chunk not found".into())
    }
}

pub fn remove_chunk(path: PathBuf, c_type: String) -> Result<String, Box<dyn error::Error>> {
    let mut file: Png = read_png(&path)?;
    let chunk = file.remove_chunk(c_type.as_str())?;
    save_to_file(path, file)?;
    Ok(chunk.to_string())
}

#[cfg(test)]
#[test]
fn test_read_file_valid() {
    let path = PathBuf::from("./ferris.png");
    let png = read_png(&path);
    assert!(png.is_ok());
}

#[test]
fn test_read_file_invalid() {
    let path = PathBuf::from("./no_such_file.png");
    let png = read_png(&path);
    assert!(png.is_err());
}
