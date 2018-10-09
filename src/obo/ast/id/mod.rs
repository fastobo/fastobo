//! Syntax leaves for identifiers.
//!
//! `RelationId`, `ClassId`, and `InstanceId` are declared as separated tuple
//! structs wrapping the same type so that `PartialEq` is not implemented
//! between them, e.g. the compiler will raise a typing error when trying to
//! compare a `RelationId` to a `ClassId`.

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write;

mod class;
mod instance;
mod person;
mod relation;

pub use self::class::*;
pub use self::instance::*;
pub use self::person::*;
pub use self::relation::*;

/// The prefix of a prefixed identifier.
pub type IdPrefix = String;

/// An OBO namespace (not an ID space).
pub type OboNamespace = String;

/// An entity identifier.
#[derive(Debug, PartialEq)]
pub enum Id {
    /// An URL identifier.
    Url(String),
    /// An unprefixed identifier, e.g. `has_part`.
    Unprefixed(String),
    /// A prefixed identifier, e.g. `GO:0005623`.
    Prefixed(IdPrefix, String),
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        match *self {
            Id::Unprefixed(ref s) => write_escaped!(f, s, + ':' => "\\:"),
            Id::Url(ref s) => write_escaped!(f, s),
            Id::Prefixed(ref p, ref s) => {
                write_escaped!(f, p, + ':' => "\\:")?;
                f.write_char(':')?;
                write_escaped!(f, s)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod display {
        use super::*;

        #[test]
        fn unprefixed() {
            let id = self::Id::Unprefixed("is_a".into());
            assert_eq!(id.to_string(), "is_a");
        }

        #[test]
        fn prefixed() {
            let id = self::Id::Prefixed("PSI".into(), "MS".into());
            assert_eq!(id.to_string(), "PSI:MS");
            let id = self::Id::Prefixed("PSI : thing".into(), "MS\n".into());
            assert_eq!(id.to_string(), "PSI\\W\\:\\Wthing:MS\\n");
        }

        #[test]
        fn url() {
            let id = self::Id::Url("https://dx.doi.org/".into());
            assert_eq!(id.to_string(), "https://dx.doi.org/");
        }
    }
}
