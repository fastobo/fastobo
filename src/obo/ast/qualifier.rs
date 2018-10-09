//! Syntax nodes for miscellaneous syntax nodes.

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write;

use super::Id;
use super::RelationId;

/// A trailing qualifier.
#[derive(Debug, PartialEq)]
pub struct Qualifier {
    pub key: Id,
    pub value: String,
}

// QUESTION: identifier of Qualifier should escape '=' to "\\=",
//           or simply does not contain it ?
impl Display for Qualifier {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.key)
            .and(f.write_str("=\""))
            .and(write_escaped!(f, self.value, '\n' => "\\n", '"' => "\\\""))
            .and(f.write_char('"'))
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
            let qual = Qualifier {
                key: Id::Unprefixed("thing ".into()),
                value: "a \"quote\"".into(),
            };
            assert_eq!(qual.to_string(), "thing\\W=\"a \\\"quote\\\"\"");
        }
    }
}
