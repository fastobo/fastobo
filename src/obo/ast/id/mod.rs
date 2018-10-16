//! Syntax leaves for identifiers.
//!
//! `RelationId`, `ClassId`, and `InstanceId` are declared as separated tuple
//! structs wrapping the same type so that `PartialEq` is not implemented
//! between them, e.g. the compiler will raise a typing error when trying to
//! compare a `RelationId` to a `ClassId`.

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write;
use std::str::FromStr;

#[macro_use]
mod _macros;

// --- ID Types ----------------------------------------------------------------

/// The prefix of a prefixed identifier.
pub type IdPrefix = String;

/// An OBO namespace (not an ID space).
pub type OboNamespace = String;

// --- ID ----------------------------------------------------------------------

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

// --- Traits ------------------------------------------------------------------

impl Display for Id {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        match self {
            Id::Unprefixed(s) => write_escaped!(f, s, + ':' => "\\:"),
            Id::Url(s) => write_escaped!(f, s),
            Id::Prefixed(p, s) => {
                write_escaped!(f, p, + ':' => "\\:")?;
                f.write_char(':')?;
                write_escaped!(f, s)
            }
        }
    }
}

impl FromStr for Id {
    type Err = crate::errors::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match super::super::parser::id::id(s) {
            Ok(("", id)) => Ok(id),
            Ok((r, _)) => Err(crate::errors::ParseError::RemainingInput {
                remainer: r.to_string(),
            }),
            Err(e) => Err(e.into_error_kind().into()),
        }
    }
}

// --- Independent ID classes --------------------------------------------------

id_subclass!(
    ///The identifier of a typedef.
    RelationId
);

id_subclass!(
    ///The identifier of a person.
    PersonId
);

id_subclass!(
    /// The identifier of a term.
    ClassId
);

id_subclass!(
    /// The identifier of an instance.
    InstanceId
);

// --- Tests -------------------------------------------------------------------

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

    mod from_str {
        use super::*;

        #[test]
        fn url() {
            let id = "http://purl.obolibrary.org/obo/MS_1000031";
            assert_eq!(Id::from_str(id), Ok(Id::Url(id.to_string())))
        }

        #[test]
        fn prefixed() {
            let id = "MS:1000031";
            assert_eq!(
                Id::from_str(id),
                Ok(Id::Prefixed("MS".to_string(), "1000031".to_string()))
            )
        }

        #[test]
        fn unprefixed() {
            let id = "has_subclass";
            assert_eq!(Id::from_str(id), Ok(Id::Unprefixed(id.to_string())));
        }
    }
}
