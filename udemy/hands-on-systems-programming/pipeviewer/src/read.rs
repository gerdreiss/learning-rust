use crate::CHUNK_SIZE;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};
use std::sync::mpsc::Sender;

pub fn read(infile: &str) -> Result<Vec<u8>> {
    let mut reader: Box<dyn Read> = if infile.is_empty() {
        Box::new(BufReader::new(io::stdin()))
    } else {
        Box::new(BufReader::new(File::open(infile)?))
    };

    let mut buffer = [0; CHUNK_SIZE];

    let _ = reader.read(&mut buffer)?;

    Ok(Vec::from(buffer))
}

pub fn read_loop(infile: &str, stats_tx: Sender<Vec<u8>>) -> Result<()> {
    let mut reader: Box<dyn Read> = if infile.is_empty() {
        Box::new(BufReader::new(io::stdin()))
    } else {
        Box::new(BufReader::new(File::open(infile)?))
    };

    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };
        // todo: send this buffer to the stats thread
        if stats_tx.send(Vec::from(&buffer[..num_read])).is_err() {
            break;
        }
    }

    let _ = stats_tx.send(Vec::new());

    Ok(())
}
