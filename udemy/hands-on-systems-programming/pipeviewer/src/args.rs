use clap::{App, Arg};
use std::env;

pub struct Args {
    pub infile: String,
    pub outfile: String,
    pub verbose: bool,
}

impl Args {
    pub fn parse() -> Self {
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

        let infile = matches.value_of("infile").unwrap_or_default().to_string();
        let outfile = matches.value_of("outfile").unwrap_or_default().to_string();
        let verbose =
            matches.is_present("verbose") || env::var("PV_VERBOSE").unwrap_or_default().is_empty();

        Self {
            infile,
            outfile,
            verbose,
        }
    }
}
