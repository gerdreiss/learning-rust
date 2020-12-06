use crossbeam::channel::{bounded, unbounded};
use pipeviewer::{args::Args, read, stats, write};
use std::io::Result;
use std::thread;

fn main() -> Result<()> {
    let args = Args::parse();
    let Args {
        infile,
        outfile,
        verbose,
    } = args;

    let (stats_tx, stats_rx) = unbounded(); // mpsc::channel();
    let (write_tx, write_rx) = bounded(1024); // mpsc::channel();

    let read_handle = thread::spawn(move || read::read_loop(&infile, stats_tx, write_tx));
    let stats_handle = thread::spawn(move || stats::stats_loop(verbose, stats_rx));
    let write_handle = thread::spawn(move || write::write_loop(&outfile, write_rx));

    // crash if any threads have crashed
    // .join() returns a thread::Result<io::Result<()>>
    let read_io_result = read_handle.join().unwrap();
    let stats_io_result = stats_handle.join().unwrap();
    let write_io_result = write_handle.join().unwrap();

    // Return an error if any thread returned an error
    read_io_result?;
    stats_io_result?;
    write_io_result?;

    //let mut total_bytes: usize = 0;
    // this has been implemented in the *_loop functions
    // loop {
    //     let buffer = match read::read(&args.infile) {
    //         Ok(x) if x.is_empty() => break,
    //         Ok(x) => x,
    //         Err(_) => break,
    //     };
    //     total_bytes += buffer.len();
    //     stats::stats(args.verbose, total_bytes, false);
    //     if !write::write(&args.outfile, &buffer)? {
    //         break;
    //     }
    // }
    // stats::stats(args.verbose, total_bytes, true);

    Ok(())
}
