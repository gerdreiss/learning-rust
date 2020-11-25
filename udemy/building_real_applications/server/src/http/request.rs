use crate::http::QueryString;
use std::convert::TryFrom;
use std::str;

use crate::http::method::HttpMethod;
use crate::parse::error::ParseError;

#[derive(Debug)]
pub struct HttpRequest<'buf> {
    path: &'buf str,
    query: Option<QueryString<'buf>>,
    method: HttpMethod,
}

impl<'buf> HttpRequest<'buf> {
    pub fn path(&self) -> &'buf str {
        &self.path
    }
    pub fn method(&self) -> &HttpMethod {
        &self.method
    }
    pub fn query(&self) -> Option<&QueryString> {
        self.query.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for HttpRequest<'buf> {
    type Error = ParseError;

    fn try_from(value: &'buf [u8]) -> Result<HttpRequest<'buf>, Self::Error> {
        let mut split = str::from_utf8(value).map(|s| s.split_whitespace())?;

        let method = split
            .next()
            .ok_or(ParseError::InvalidRequest)
            .and_then(|m| m.parse::<HttpMethod>())?;

        let (path, query) = split
            .next()
            .filter(|p| p.starts_with("/")) // the path must start with "/"
            .map(|p| p.splitn(2, '?')) // split into exactly two chunks: path and query (if any)
            .map(|mut s| (s.next(), s.next()))
            .map(|(maybe_path, maybe_query)| {
                (
                    maybe_path,
                    maybe_query.and_then(|q| QueryString::try_from(q).ok()),
                )
            })
            .and_then(|(maybe_path, maybe_query)| maybe_path.map(|p| (p, maybe_query)))
            .ok_or(ParseError::InvalidRequest)?;

        let _protocol = split
            .next()
            .and_then(|s| s.split('\r').next())
            .filter(|s| s.starts_with("HTTP/1.1")) // we accept only HTTP requests
            .ok_or(ParseError::InvalidProtocol)?;

        Ok(Self {
            path,
            query,
            method,
        })
    }
}
