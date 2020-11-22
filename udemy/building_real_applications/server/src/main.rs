use http::HttpMethod;
use http::HttpRequest;
use server::Server;

mod http;
mod server;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    let result = server.run();
}
