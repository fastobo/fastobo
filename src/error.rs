//! `Error` and `Result` types for this crate.

use std::error::Error as StdError;
use std::io::Error as IOError;

use pest::error::Error as PestError;
use pest::error::InputLocation;
use pest::error::LineColLocation;
use pest::Position;
use pest::Span;

use crate::ast::*;
use crate::parser::Rule;

/// An error for cardinality violation.
///
/// This error is highly dependent on the function that returns it: the `name`
/// field can provide more information about the specific clause that errored
/// to the end-user.
#[derive(Debug, Eq, Fail, PartialEq)]
pub enum CardinalityError {
    #[fail(display = "missing {:?} clause", name)]
    MissingClause { name: String },
    #[fail(display = "duplicate {:?} clauses", name)]
    DuplicateClauses { name: String },
    #[fail(display = "invalid single {:?} clause", name)]
    SingleClause { name: String },
}

impl CardinalityError {
    pub(crate) fn missing<S: Into<String>>(name: S) -> Self {
        CardinalityError::MissingClause { name: name.into() }
    }

    pub(crate) fn duplicate<S: Into<String>>(name: S) -> Self {
        CardinalityError::DuplicateClauses { name: name.into() }
    }

    pub(crate) fn single<S: Into<String>>(name: S) -> Self {
        CardinalityError::DuplicateClauses { name: name.into() }
    }
}

/// The error type for this crate.
#[derive(Debug, Fail)]
pub enum Error {
    /// An unexpected rule was used in `FromPair::from_pair`.
    ///
    /// # Example
    /// ```rust
    /// # extern crate fastobo;
    /// # use fastobo::ast::*;
    /// # use fastobo::parser::*;
    /// let pairs = OboParser::parse(Rule::UnquotedString, "hello, world!");
    /// # let err =
    /// QuotedString::from_pair(pairs.unwrap().next().unwrap()).unwrap_err();
    /// # match err {
    /// #   fastobo::error::Error::UnexpectedRule { expected, actual } => {
    /// #       assert_eq!(expected, Rule::QuotedString);
    /// #       assert_eq!(actual, Rule::UnquotedString);
    /// #   }
    /// #   e => panic!("unexpected error: {:?}", e),
    /// # };
    /// ```
    #[fail(display = "unexpected rule: {:?} (expected {:?})", actual, expected)]
    UnexpectedRule { expected: Rule, actual: Rule },

    /// The underlying parser encountered an error.
    ///
    /// # Example
    /// ```rust
    /// # extern crate fastobo;
    /// # use std::str::FromStr;
    /// # use fastobo::ast::*;
    /// # let err =
    /// QuotedString::from_str("definitely not a quoted string").unwrap_err();
    /// # match err {
    /// #   fastobo::error::Error::ParserError { .. } => (),
    /// #   e => panic!("unexpected error: {:?}", e),
    /// # };
    /// ```
    #[fail(display = "parser error: {}", error)]
    ParserError {
        #[cause]
        error: PestError<Rule>,
    },

    /// An IO error occurred.
    ///
    /// # Example
    /// ```rust
    /// # extern crate fastobo;
    /// # use fastobo::ast::*;
    /// # let err =
    /// OboDoc::from_file("some/non-existing/path").unwrap_err();
    /// # match err {
    /// #   fastobo::error::Error::IOError { .. } => (),
    /// #   e => panic!("unexpected error: {:?}", e),
    /// # };
    #[fail(display = "IO error: {}", error)]
    IOError {
        #[cause]
        error: IOError,
    },

    /// A cardinality-related error occurred.
    #[fail(display = "cardinality error: {}", inner)]
    CardinalityError {
        id: Option<Ident>,
        #[cause]
        inner: CardinalityError,
    },
}

impl Error {
    /// Update the line of the error, if needed.
    pub(crate) fn with_offsets(self, line_offset: usize, offset: usize) -> Self {
        use self::Error::*;
        use pest::error::InputLocation;
        use pest::error::LineColLocation;
        match self {
            e @ IOError { .. } => e,
            e @ CardinalityError { .. } => e,
            e @ UnexpectedRule { .. } => e,
            ParserError { mut error } => {
                error.location = match error.location {
                    InputLocation::Pos(s) => InputLocation::Pos(s + offset),
                    InputLocation::Span((s, e)) => InputLocation::Span((s + offset, e + offset)),
                };
                error.line_col = match error.line_col {
                    LineColLocation::Pos((l, c)) => LineColLocation::Pos((l + line_offset, c)),
                    LineColLocation::Span((ls, cs), (le, ce)) => {
                        LineColLocation::Span((ls + line_offset, cs), (le + line_offset, ce))
                    }
                };
                ParserError { error }
            }
        }
    }

    /// Update the path of the error, if needed.
    pub(crate) fn with_path(self, path: &str) -> Self {
        use self::Error::*;
        match self {
            e @ IOError { .. } => e,
            e @ UnexpectedRule { .. } => e,
            e @ CardinalityError { .. } => e,
            ParserError { error } => ParserError {
                error: error.with_path(path),
            },
        }
    }

    /// Update the span of the error, if needed.
    pub(crate) fn with_span(self, span: Span) -> Self {
        use self::Error::*;
        match self {
            e @ IOError { .. } => e,
            e @ UnexpectedRule { .. } => e,
            e @ CardinalityError { .. } => e,
            ParserError { error } => {
                // FIXME(@althonos): the new error should be spanned only if
                //                   the original error is spanned, but there
                //                   is no clean way to create an error at
                //                   the right position with `pest::error`.
                ParserError {
                    error: PestError::new_from_span(error.variant, span),
                }
            }
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
