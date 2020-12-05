use crate::CHUNK_SIZE;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};

pub fn read(infile: &str) -> Result<Vec<u8>> {
    let mut reader: Box<dyn Read> = if infile.is_empty() {
        Box::new(BufReader::new(io::stdin()))
    } else {
        Box::new(BufReader::new(File::open(infile)?))
    };

    let mut buffer = [0; CHUNK_SIZE];

    reader.read(&mut buffer)?;

    Ok(Vec::from(buffer))
}
