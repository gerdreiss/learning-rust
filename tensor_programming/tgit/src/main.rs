use clap::{App, Arg, SubCommand};

mod error;
mod index;
mod init;
mod types;
mod file;
mod add;
mod commit;

fn main() {
    let m = App::new("tgit")
        .subcommand(SubCommand::with_name("init").about("Initialize the Repo"))
        .subcommand(SubCommand::with_name("add").about("Add a file")
                        .arg(Arg::with_name("file").help("File to add")
                                 .index(1)
                                 .multiple(true)
                                 .required(true),
                        ),
        )
        .get_matches();

    match m.subcommand() {
        ("init", Some(..)) => {
            match init::init() {
                Ok(()) => println!("Repo initialized"),
                Err(..) => println!("Already Initialized"),
            }
        }
        ("add", Some(submatches)) => {
            let file_names = &submatches.values_of("file").unwrap().collect();
            match add::add_all(file_names) {
                Ok(()) => (),
                Err(e) => println!("Error: {}", e),
            }
        }
        _ => println!("Command not recognized")
    }
}
