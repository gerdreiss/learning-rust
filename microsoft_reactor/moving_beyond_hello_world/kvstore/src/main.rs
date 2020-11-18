use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use std::io::Error;
struct Database {
    inner: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, Error> {
        let contents: String = std::fs::read_to_string("kv.db").expect("Failed to read database");
        let db: HashMap<String, String> = contents
            .lines()
            .map(|line| line.split('\t').collect::<Vec<&str>>()) // do I have to collect here?
            .filter(|split| !split.is_empty())
            .map(|split| (split[0], split[1]))
            .fold(HashMap::new(), |mut hmap, (k, v)| {
                hmap.insert(k.to_owned(), v.to_owned());
                hmap
            });

        Ok(Database { inner: db })
    }
    fn write(&self, key: String, value: String) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("kv.db")?;
        writeln!(file, "{}\t{}", key, value)
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next();
    let value = args.next();
    let database = Database::new();

    match (database, key, value) {
        (Ok(db), Some(k), Some(v)) => match db::write(k, v) {
            Err(e) => eprintln!("Couldn't write to file: {}", e),
            Ok(_) => println!("{}: {} added to database", k, v),
        },
        (Err(e), _, _) => eprintln!("Couldn't read database file: {}", e),

        _ => println!("Both key and value must be defined: kvstore <key> <value>"),
    }
}
