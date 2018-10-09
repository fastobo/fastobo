use nom::ErrorKind;
use std::convert::From;

use failure::Error;

/// Error type for `FromStr` result.
#[derive(Debug, PartialEq, Fail)]
pub enum ParseError {
    #[fail(display = "remaining input: {}", remainer)]
    RemainingInput { remainer: String },
    #[fail(display = "invalid character: '{}'", invalid)]
    InvalidCharacter { invalid: char },
    #[fail(display = "nom parser error: '{}'", description)]
    NomError { description: String },
}

impl From<ErrorKind> for ParseError {
    fn from(kind: ErrorKind) -> Self {
        ParseError::NomError {
            description: kind.description().to_string(),
        }
    }
}
