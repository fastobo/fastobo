//! `Error` and `Result` types for this crate.

use std::io::Error as IOError;

use pest::error::Error as PestError;

use crate::parser::Rule;

/// The error type for this crate.
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "invalid character: {}", c)]
    InvalidCharacter { c: char },
    #[fail(display = "unexpected rule: {:?} (expected {:?})", actual, expected)]
    UnexpectedRule { expected: Rule, actual: Rule },
    #[fail(display = "parser error: {}", error)]
    ParserError { error: PestError<Rule> },
    #[fail(display = "IO error: {}", error)]
    IOError { error: IOError },
}

impl From<PestError<Rule>> for Error {
    fn from(error: PestError<Rule>) -> Self {
        Error::ParserError { error }
    }
}

impl From<IOError> for Error {
    fn from(error: IOError) -> Self {
        Error::IOError { error }
    }
}

/// The result type for this crate.
pub type Result<T> = std::result::Result<T, Error>;
