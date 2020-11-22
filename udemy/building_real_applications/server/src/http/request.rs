use super::method::HttpMethod;

pub struct HttpRequest {
    path: String,
    query: Option<String>,
    method: HttpMethod,
}
