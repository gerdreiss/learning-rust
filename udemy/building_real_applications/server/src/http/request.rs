use std::convert::TryFrom;
use std::str;

use crate::http::method::HttpMethod;
use crate::parse::error::ParseError;

#[derive(Debug)]
pub struct HttpRequest {
    path: String,
    query: Option<String>,
    method: HttpMethod,
}

impl HttpRequest {
    fn from(request_string: &str) -> Result<Self, ParseError> {
        let mut split = request_string.split_whitespace();

        let method = split
            .next()
            .ok_or(ParseError::InvalidRequest)
            .and_then(|m| m.parse::<HttpMethod>())?;

        let (path, query) = split
            .next()
            .filter(|p| p.starts_with("/")) // the path must start with "/"
            .map(|p| p.split('?'))
            .map(|mut s| (s.next(), s.next()))
            .and_then(|(maybe_path, maybe_query)| {
                maybe_path.map(|p| (p.to_string(), maybe_query.map(|q| q.to_string())))
            })
            .ok_or(ParseError::InvalidRequest)?;

        let protocol = split
            .next()
            .filter(|s| s.starts_with("HTTP/1.1")) // we accept only HTTP requests
            .map(|s| s[..8].to_string())
            .ok_or(ParseError::InvalidProtocol)?;

        Ok(HttpRequest {
            path: path,
            query: query,
            method: method,
        })
    }
}

impl TryFrom<&[u8]> for HttpRequest {
    type Error = ParseError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let s_req = str::from_utf8(value)?;
        HttpRequest::from(s_req)
    }
}
