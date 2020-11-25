use core::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, Clone, Copy)]
pub enum StatusCode {
    OK = 200,
    Created = 201,
    BadRequest = 400,
    NotFound = 404,
    ServerError = 500,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &'static str {
        match self {
            Self::OK => "OK",
            Self::Created => "Created",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
            Self::ServerError => "Server Error",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}
