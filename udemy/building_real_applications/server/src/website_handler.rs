use super::server::Handler;
use crate::http::status_code::StatusCode;
use crate::http::HttpMethod;
use crate::http::HttpRequest;
use crate::http::HttpResponse;

use std::fs;

pub struct WebsiteHandler {
    public_path: String,
}
impl WebsiteHandler {
    pub fn new(path: String) -> Self {
        Self { public_path: path }
    }
    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &HttpRequest) -> HttpResponse {
        println!("Request object: {:?}", request);

        match request.method() {
            HttpMethod::GET => match request.path() {
                "/" => HttpResponse::new(StatusCode::OK, self.read_file("index.html")),
                "/hello" => HttpResponse::new(StatusCode::OK, self.read_file("hello.html")),
                path => self
                    .read_file(path)
                    .map(|content| HttpResponse::new(StatusCode::OK, Some(content)))
                    .unwrap_or(HttpResponse::new(StatusCode::NotFound, None)),
            },
            _ => HttpResponse::new(StatusCode::NotFound, None),
        }
    }
}
