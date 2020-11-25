use super::status_code::StatusCode;
use core::fmt::{Display, Formatter, Result as FmtResult};
use std::io::{Result as IoResult, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct HttpResponse {
    status_code: StatusCode,
    body: Option<String>,
}

impl HttpResponse {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        HttpResponse { status_code, body }
    }
    pub fn send(&self, stream: &mut TcpStream) -> IoResult<()> {
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            self.body.as_ref().unwrap_or(&String::new()),
        );
        stream.flush()
    }
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            self.body.as_ref().unwrap_or(&String::new()),
        )
    }
}
