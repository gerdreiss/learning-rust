use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;

struct Database {
    inner: HashMap<String, String>,
}

impl Database {
    fn new() -> Database {
        let contents: String = std::fs::read_to_string("kv.db").unwrap_or_default();

        let entries: HashMap<String, String> = contents
            .lines()
            .map(|line| line.split('\t').collect::<Vec<&str>>()) // do I have to collect here?
            .filter(|split| !split.is_empty())
            .map(|split| (split[0], split[1]))
            .fold(HashMap::new(), |mut hmap, (k, v)| {
                hmap.insert(k.to_owned(), v.to_owned());
                hmap
            });

        Database { inner: entries }
    }

    fn write(&self, key: String, value: String) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("kv.db")?;
        writeln!(file, "{}\t{}", key, value)
    }

    #[allow(dead_code)]
    fn write_al(&self) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("kv.db")?;
        self.inner
            .iter()
            .map(|(key, value)| writeln!(file, "{}\t{}", key, value))
            .find(|r| r.is_err())
            .unwrap_or_else(|| Ok(()))
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next();
    let value = args.next();

    let database = Database::new();

    match (key, value) {
        (Some(k), Some(v)) => match database.write(k.clone(), v.clone()) {
            Err(e) => eprintln!("Couldn't write to file: {}", e),
            Ok(_) => println!("{}: {} added to database", k, v),
        },

        _ => println!("Both key and value must be defined: kvstore <key> <value>"),
    }
}
