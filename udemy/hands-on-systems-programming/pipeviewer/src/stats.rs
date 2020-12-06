use crossbeam::channel::Receiver;
use std::io::Result;

pub fn stats(verbose: bool, total_bytes: usize, last: bool) {
    if verbose {
        eprint!("\r{}", total_bytes);
        if last {
            eprintln!();
        }
    }
}

pub fn stats_loop(verbose: bool, stats_rx: Receiver<usize>) -> Result<()> {
    let mut total_bytes = 0;
    loop {
        let num_bytes = stats_rx.recv().unwrap();
        total_bytes += num_bytes;
        if verbose {
            eprint!("\r{}", total_bytes);
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
