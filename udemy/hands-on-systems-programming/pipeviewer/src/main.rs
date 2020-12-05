use clap::{App, Arg};
use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, ErrorKind, Read, Write};

const CHUNK_SIZE: usize = 16 * 1024;

fn main() -> io::Result<()> {
    let matches = App::new("pipeviewer")
        .arg(Arg::with_name("infile").help("Read from a file instead of stdin."))
        .arg(
            Arg::with_name("outfile")
                .short("o")
                .long("outfile")
                .takes_value(true)
                .help("Write to a file instead of stdout."),
        )
        .arg(Arg::with_name("verbose").short("V").long("verbose"))
        .get_matches();

    let infile = matches.value_of("infile").unwrap_or_default();
    let outfile = matches.value_of("outfile").unwrap_or_default();
    let verbose =
        matches.is_present("verbose") || env::var("PV_VERBOSE").unwrap_or_default().is_empty();

    let mut reader: Box<dyn Read> = if infile.is_empty() {
        Box::new(BufReader::new(io::stdin()))
    } else {
        Box::new(BufReader::new(File::open(infile)?))
    };
    let mut writer: Box<dyn Write> = if outfile.is_empty() {
        Box::new(BufWriter::new(io::stdout()))
    } else {
        Box::new(BufWriter::new(File::create(outfile)?))
    };

    let mut total_bytes = 0;
    let mut buffer = [0; CHUNK_SIZE];

    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(x) => x,
            Err(_) => break,
        };

        total_bytes += num_read;
        if verbose {
            eprint!("\r{}", total_bytes);
        }

        if let Err(e) = writer.write_all(&buffer[..num_read]) {
            if e.kind() == ErrorKind::BrokenPipe {
                break;
            }
            return Err(e);
        }
    }

    Ok(())
}
