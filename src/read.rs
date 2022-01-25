use std::io::{self, BufReader, Read, Result};
use crate::CHUNK_SIZE_KB;
use std::fs::File;

pub fn read(infile: &str) -> Result<Vec<u8>> {

    let mut reader: Box<dyn Read> = if !infile.is_empty() {
        Box::new(BufReader::new(File::open(infile)?))
    } else {
        Box::new(BufReader::new(io::stdin()))
    };

    let mut buffer = [0; CHUNK_SIZE_KB];

    let num_read = reader.read(&mut buffer)?;
    Ok(Vec::from(&buffer[..num_read]))
}