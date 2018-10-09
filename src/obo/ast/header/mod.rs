//! Syntax nodes for header clauses.

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write;

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
