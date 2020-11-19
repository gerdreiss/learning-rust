use anyhow::{Context, Result};
use std::fs::read_to_string;
use std::io;
use std::io::Error;
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), Error> {
    let stdout = io::stdout(); // get the global stdout entity
    let mut handle = stdout.lock(); // acquire a lock on it
    let pb = indicatif::ProgressBar::new(1000);
    let args = Cli::from_args();
    pb.inc(1);
    let result = read_to_string(&args.path)
        .with_context(|| format!("could not read file '{}'", &args.path.display()))
        .iter()
        .flat_map(|content| {
            content.lines().find(|line| {
                pb.inc(1);
                line.contains(&args.pattern)
            })
        })
        .map(|found| writeln!(handle, "{}", found))
        .find(|result| result.is_err())
        .unwrap_or(Ok(()));
    pb.finish_with_message("done");
    result
}

fn _main() -> Result<(), CustomError> {
    let args = Cli::from_args();
    read_to_string(&args.path)
        .map_err(|err| CustomError(format!("Error reading {}: {}", &args.path.display(), err)))
        .iter()
        .flat_map(|content| content.lines().find(|line| line.contains(&args.pattern)))
        .for_each(|found| println!("{}", found));

    Ok(())
}
