//! Syntax nodes for header clauses.

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write;
use std::str::FromStr;

use crate::errors;

mod clause;

pub use self::clause::HeaderClause;

/// A header frame, found at the beginning of an OBO file.
#[derive(Debug, PartialEq)]
pub struct HeaderFrame {
    pub clauses: Vec<HeaderClause>,
}

impl Display for HeaderFrame {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        match self
            .clauses
            .iter()
            .flat_map(|clause| write!(f, "{}\n", clause).err())
            .next()
        {
            Some(err) => Err(err),
            None => Ok(()),
        }
    }
}

impl FromStr for HeaderFrame {
    type Err = crate::errors::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match super::super::parser::header::header_frame(s) {
            Ok(("", frame)) => Ok(frame),
            Ok((r, _)) => Err(self::errors::ParseError::RemainingInput {
                remainer: r.to_string(),
            }),
            Err(e) => Err(e.into_error_kind().into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod display {
        use super::*;

        #[test]
        fn empty() {
            let f = HeaderFrame { clauses: vec![] };
            assert_eq!(f.to_string(), "");
        }

        #[test]
        fn single_clause() {
            let f = HeaderFrame {
                clauses: vec![HeaderClause::SavedBy("Martin Larralde".to_string())],
            };
            assert_eq!(f.to_string(), "saved-by: Martin Larralde\n");
        }

        #[test]
        fn multiple_clauses() {
            let f = HeaderFrame {
                clauses: vec![
                    HeaderClause::FormatVersion("1.2".to_string()),
                    HeaderClause::SavedBy("Martin Larralde".to_string()),
                ],
            };
            assert_eq!(
                f.to_string(),
                "format-version: 1.2\nsaved-by: Martin Larralde\n"
            );
        }
    }
}
