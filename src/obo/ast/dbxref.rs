//! Syntax nodes for miscellaneous syntax nodes.

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write;

use super::Id;
use super::RelationId;

/// A database cross-reference.
#[derive(Debug, PartialEq)]
pub struct DbXref {
    pub name: Id,
    pub description: Option<String>,
}

// QUESTION: identifier of DbXref must also escape ',' to "\\," ?
//           or simply does not contain it ?
impl Display for DbXref {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        if let Some(ref desc) = self.description {
            write!(f, "{} \"", self.name)
                .and(write_escaped!(f, desc, '\n' => "\\n", '"' => "\\\""))
                .and(f.write_char('"'))
        } else {
            write!(f, "{}", self.name)
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {

    use super::Id;
    use super::*;

    mod display {

        use super::*;

        #[test]
        fn unprefixed() {
            let xref = super::DbXref {
                name: Id::Unprefixed("x".into()),
                description: None,
            };
            assert_eq!(xref.to_string(), "x");
        }

        #[test]
        fn prefixed() {
            let xref = super::DbXref {
                name: Id::Prefixed("PSI".to_string(), "MS".to_string()),
                description: Some("something".to_string()),
            };
            assert_eq!(xref.to_string(), "PSI:MS \"something\"");
        }
    }
}
