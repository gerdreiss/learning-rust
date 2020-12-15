use failure_derive::*;

#[derive(Debug, Fail)]
pub enum TransactionError {
    #[fail(display = "Could not load file: {}", 0)]
    LoadError(std::io::Error),
    #[fail(display = "Could not parse transactions: {}", 0)]
    ParseError(serde_json::Error),
    #[fail(display = "Error: {}", 0)]
    Msg(&'static str),
}

impl From<std::io::Error> for TransactionError {
    fn from(err: std::io::Error) -> Self {
        TransactionError::LoadError(err)
    }
}

impl From<serde_json::Error> for TransactionError {
    fn from(err: serde_json::Error) -> Self {
        TransactionError::ParseError(err)
    }
}

impl From<&'static str> for TransactionError {
    fn from(e: &'static str) -> Self {
        TransactionError::Msg(e)
    }
}
