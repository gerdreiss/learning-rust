use std::convert::From;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::Utf8Error;

#[derive(Debug)]
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
    InvalidQuery,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            ParseError::InvalidRequest => "Invalid request",
            ParseError::InvalidEncoding => "Invalid encoding",
            ParseError::InvalidProtocol => "Invalid protocol",
            ParseError::InvalidMethod => "Invalid method",
            ParseError::InvalidQuery => "Invalid query",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "{}", self.message())
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Error for ParseError {}
