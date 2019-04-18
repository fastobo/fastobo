//! `Error` and `Result` types for this crate.

use std::io::Error as IOError;

use pest::error::Error as PestError;
use pest::error::InputLocation;
use pest::error::LineColLocation;

use crate::parser::Rule;

/// The error type for this crate.
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "unexpected rule: {:?} (expected {:?})", actual, expected)]
    UnexpectedRule { expected: Rule, actual: Rule },
    #[fail(display = "parser error: {}", error)]
    ParserError { error: PestError<Rule> },
    #[fail(display = "IO error: {}", error)]
    IOError { error: IOError },
}

impl Error {
    /// Update the line of the error, if needed.
    pub(crate) fn with_offsets(self, line_offset: usize, offset: usize) -> Self {
        use self::Error::*;
        use pest::error::InputLocation;
        use pest::error::LineColLocation;
        match self {
            IOError { error } => IOError { error },
            UnexpectedRule { expected, actual } => UnexpectedRule { expected, actual },
            ParserError { mut error } => {
                error.location = match error.location {
                    InputLocation::Pos(s) =>
                        InputLocation::Pos(s + offset),
                    InputLocation::Span((s, e)) =>
                        InputLocation::Span((s + offset, e + offset))
                };
                println!("{:?}", error.location);
                error.line_col = match error.line_col {
                    LineColLocation::Pos((l, c)) =>
                        LineColLocation::Pos((l + line_offset, c)),
                    LineColLocation::Span((ls, cs), (le, ce)) =>
                        LineColLocation::Span((ls + line_offset, cs), (le + line_offset, ce))
                };
                ParserError { error }
            }
        }
    }

    /// Update the path of the error, if needed.
    pub(crate) fn with_path(self, path: &str) -> Self {
        use self::Error::*;
        match self {
            IOError { error } => IOError { error },
            UnexpectedRule { expected, actual } => UnexpectedRule { expected, actual },
            ParserError { error } => ParserError { error: error.with_path(path) },
        }
    }
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
