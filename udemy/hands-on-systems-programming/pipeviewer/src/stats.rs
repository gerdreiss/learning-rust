use crossbeam::channel::Receiver;
use std::io::Result;
use std::time::Instant;

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
    let start = Instant::now();
    let mut last_instant = Instant::now();
    loop {
        let num_bytes = stats_rx.recv().unwrap();
        let now = Instant::now();
        let elapsed = start.elapsed().as_secs();
        let rate_per_second = num_bytes as f64 / (now - last_instant).as_secs_f64();
        last_instant = now;
        total_bytes += num_bytes;
        if verbose {
            eprint!(
                "\r{} bytes in {} seconds [{:.0} b/s]",
                total_bytes, elapsed, rate_per_second
            );
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
