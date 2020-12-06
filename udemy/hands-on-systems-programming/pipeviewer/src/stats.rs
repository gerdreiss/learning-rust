use std::io::Result;
use std::sync::mpsc::{Receiver, Sender};

pub fn stats(verbose: bool, total_bytes: usize, last: bool) {
    if verbose {
        eprint!("\r{}", total_bytes);
        if last {
            eprintln!();
        }
    }
}

pub fn stats_loop(
    verbose: bool,
    stats_rx: Receiver<Vec<u8>>,
    write_tx: Sender<Vec<u8>>,
) -> Result<()> {
    let mut total_bytes = 0;
    loop {
        let buffer: Vec<u8> = stats_rx.recv().unwrap();
        let num_bytes = buffer.len();
        total_bytes += num_bytes;
        if verbose {
            eprint!("\r{}", total_bytes);
        }
        if write_tx.send(buffer).is_err() {
            break;
        }
        if num_bytes == 0 {
            break;
        }
    }
    if verbose {
        eprintln!();
    }
    Ok(())
}
