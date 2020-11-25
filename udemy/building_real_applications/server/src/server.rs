use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

use crate::http::status_code::StatusCode;
use crate::http::HttpRequest;
use crate::http::HttpResponse;

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
                        Ok(_) => {
                            println!("Request content: {:?}", String::from_utf8_lossy(&buf));
                            match HttpRequest::try_from(&buf[..]) {
                                Err(err) => {
                                    println!("Failed to parse request: {}", err);
                                    HttpResponse::new(StatusCode::BadRequest, None)
                                }
                                Ok(req) => {
                                    println!("Request object: {:?}", req);
                                    HttpResponse::new(
                                        StatusCode::OK,
                                        Some(
                                            "<br/><br/><center><h1>IT WORKS!</h1></center>"
                                                .to_owned(),
                                        ),
                                    )
                                }
                            }
                        }
                    };
                    if let Err(e) = response.send(&mut stream) {
                        println!("Failed to send response: {:?}", e);
                    }
                }
            }
        }
    }
}
