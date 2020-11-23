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
            listener
                .accept()
                .and_then(|(mut stream, addr)| self.process_request(addr, &mut stream));

            // match listener.accept() {
            //     Err(err) => eprintln!("Failed to establish connection: {}", err),
            //     Ok((mut stream, addr)) => process_request(addr, &mut stream)
            // }
        }
    }

    fn process_request(&self, addr: SocketAddr, stream: &mut TcpStream) -> io::Result<()> {
        println!("Request received from {}", addr);

        let mut buf = Vec::new();

        stream.read_to_end(&mut buf).and_then(|_| {
            // match buf.as_slice().try_into() as <Result<HttpRequest, String>> {
            //     Err(err) => println!("Failed to parse request: {}", err),
            //     Ok(req) => println!("Request received: {:?}", req),
            // }
            match HttpRequest::try_from(buf.as_slice()) {
                Err(err) => println!("Failed to parse request: {}", err),
                Ok(req) => println!("Request received: {:?}", req),
            }
            Ok(())
        })
    }
}
