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
            &TgitError::InvalidCommit => formatter.write_str("The commit is invalid"),
            &TgitError::InvalidIndex => formatter.write_str("The index is corrupt"),
        }
    }
}

impl From<io::Error> for TgitError {
    fn from(err: io::Error) -> TgitError {
        TgitError::IoError(err)
    }
}