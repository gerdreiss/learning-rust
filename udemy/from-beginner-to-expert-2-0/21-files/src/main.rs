use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

fn basic_file_handling() -> std::io::Result<()> {
    let path = Path::new("/Users/g/Downloads/abc.txt");

    let mut file = File::create(path)?;
    file.write(b"created with Rust!")?;

    let mut file = OpenOptions::new().append(true).open(path)?;
    file.write(b"\nAppended with Rust!")?;

    let mut contents = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut contents)?;

    println!("File contents: {}", contents);

    Ok(())
}

fn main() {
    match basic_file_handling() {
        Ok(_) => println!("File created and/or read successfully"),
        Err(e) => println!("Error: {}", e),
    }
}
