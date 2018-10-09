//! Syntax nodes for miscellaneous syntax nodes.

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write;

use super::Id;
use super::RelationId;

/// A property value attached to an entity.
#[derive(Debug, PartialEq)]
pub enum PropertyValue {
    Identified(RelationId, Id),
    // FIXME: maybe should be Typed(RelationId, String, DatatypeId)
    Typed(RelationId, String, String),
}

impl Display for PropertyValue {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        use self::PropertyValue::*;
        match self {
            Identified(rel, id) => write!(f, "{} {}", rel, id),
            Typed(rel, value, datatype) => write!(f, "{} \"", rel)
                .and(write_escaped!(f, value, '\n' => "\\n", '"' => "\\\""))
                .and(f.write_str("\" "))
                .and(write_escaped!(f, datatype, '\n' => "\\n")),
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
        fn identified() {
            use self::Id::Unprefixed;
            use self::PropertyValue::Identified;
            let pv = Identified(
                Unprefixed("married_to".to_string()).into(),
                Unprefixed("heather".to_string()),
            );
            assert_eq!(pv.to_string(), "married_to heather");
        }

        #[test]
        fn typed() {
            use self::Id::Unprefixed;
            use self::PropertyValue::Typed;
            let pv = Typed(
                Unprefixed("shoe_size".into()).into(),
                "8".to_string(),
                "xsd:positiveInteger".to_string(),
            );
            assert_eq!(pv.to_string(), "shoe_size \"8\" xsd:positiveInteger");
        }
    }
}
