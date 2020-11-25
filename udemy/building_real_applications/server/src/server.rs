use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

use crate::http::status_code::StatusCode;
use crate::http::HttpRequest;
use crate::http::HttpResponse;
use crate::parse::ParseError;

pub trait Handler {
    fn handle_request(&mut self, request: &HttpRequest) -> HttpResponse;
    fn handle_bad_request(&mut self, e: &ParseError) -> HttpResponse {
        println!("Failed to parse request: {}", e);
        HttpResponse::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", &self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Err(err) => eprintln!("Failed to establish connection: {}", err),
                Ok((mut stream, addr)) => {
                    println!("Request received from {}", addr);
                    let mut buf = [0; 1024];
                    let response = match stream.read(&mut buf) {
                        Err(e) => {
                            eprintln!("Failed to read from stream: {}", e);
                            HttpResponse::new(StatusCode::ServerError, None)
                        }
                        // is this even possible?
                        Ok(0) => {
                            eprintln!("Empty request (?)");
                            HttpResponse::new(StatusCode::BadRequest, None)
                        }
                        Ok(_) => match HttpRequest::try_from(&buf[..]) {
                            Err(err) => handler.handle_bad_request(&err),
                            Ok(req) => handler.handle_request(&req),
                        },
                    };
                    if let Err(e) = response.send(&mut stream) {
                        println!("Failed to send response: {:?}", e);
                    }
                }
            }
        }
    }
}
