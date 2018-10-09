//! Syntax nodes for miscellaneous syntax nodes.

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write;

use super::Id;
use super::RelationId;

/// The scope of a synonym, e.g. `BROAD` or `RELATED`.
#[derive(Debug, PartialEq)]
pub enum SynonymScope {
    Exact,
    Broad,
    Narrow,
    Related,
}

impl Display for SynonymScope {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        use self::SynonymScope::*;
        match *self {
            Exact => f.write_str("EXACT"),
            Broad => f.write_str("BROAD"),
            Narrow => f.write_str("NARROW"),
            Related => f.write_str("RELATED"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod display {
        use super::*;

        #[test]
        fn exact() {
            assert_eq!(super::SynonymScope::Exact.to_string(), "EXACT");
        }
    }
}
