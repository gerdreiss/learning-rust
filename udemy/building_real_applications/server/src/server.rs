use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
    pub fn run(&self) {
        println!("Listening on {}", &self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Err(err) => {
                    eprintln!("Failed to establish connection: {}", err);
                    continue;
                }
                Ok((mut stream, addr)) => {
                    println!("Request received from {}", addr);
                    let mut buf = Vec::new();
                    match stream.read_to_end(&mut buf) {
                        Err(err) => {
                            eprintln!("Failed to read from connection: {}", err);
                            continue;
                        }
                        Ok(_) => {
                            let req = String::from_utf8_lossy(buf.as_mut_slice());
                            println!("Request received: {}", req)
                        }
                    }
                }
            }
        }
    }
}
