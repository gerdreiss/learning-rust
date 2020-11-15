use std::fmt;
use std::io;

pub enum TgitError {
    IoError(io::Error),
    NoDirectory,
    InvalidCommit,
    InvalidIndex,
}

impl fmt::Display for TgitError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &TgitError::IoError(ref e) => e.fmt(formatter),
            &TgitError::NoDirectory => formatter.write_str("No Directory Found"),
            &TgitError::InvalidCommit => formatter.write_str("Invalid Commit"),
            &TgitError::InvalidIndex => formatter.write_str("Corrupt index")
        }
    }
}

impl From<io::Error> for TgitError {
    fn from(err: io::Error) -> TgitError {
        TgitError::IoError(err)
    }
}
