use std::convert::TryFrom;
use std::convert::TryInto;
use std::io;
use std::io::Read;
use std::net::{SocketAddr, TcpListener, TcpStream};

use crate::http::HttpRequest;

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
                Err(err) => eprintln!("Failed to establish connection: {}", err),
                Ok((mut stream, addr)) => {
                    println!("Request received from {}", addr);
                    let mut buf = Vec::new();
                    match stream.read_to_end(&mut buf) {
                        Err(err) => eprintln!("Failed to read from stream: {}", err),
                        Ok(size) => {
                            if size == 0 {
                                eprintln!("Empty request (?)");
                            } else {
                                match HttpRequest::try_from(buf.as_slice()) {
                                    Err(err) => println!("Failed to parse request: {}", err),
                                    Ok(req) => println!("Request received: {:?}", req),
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
