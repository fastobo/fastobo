use pest::error::Error as PestError;

use crate::parser::Rule;

#[derive(Debug, Fail, PartialEq, Eq, Hash)]
pub enum Error {
    #[fail(display = "invalid character: {}", c)]
    InvalidCharacter { c: char },
    #[fail(display = "remaining input: {}", i)]
    RemainingInput { i: String },
    #[fail(display = "parser error: {}", error)]
    ParserError { error: PestError<Rule> },
    #[fail(display = "unexpected rule: {:?} (expected {:?})", actual, expected)]
    UnexpectedRule { expected: Rule, actual: Rule },
}

impl From<PestError<Rule>> for Error {
    fn from(error: PestError<Rule>) -> Self {
        Error::ParserError { error }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
