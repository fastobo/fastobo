use std::convert::TryFrom;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use iri_string::Url;
use pest::iterators::Pair;

use crate::obo14::parser::FromPair;
use crate::obo14::parser::Parser;
use crate::obo14::parser::Rule;
use crate::error::Error;
use crate::error::Result;

/// An identifier without a prefix.
#[derive(Debug, PartialEq, Hash, Eq)]
pub struct UnprefixedId {
    value: String,
}

impl UnprefixedId {
    /// Create a new unprefixed identifier.
    pub fn new<S: Into<String>>(id: S) -> Self {
        Self { value: id.into() }
    }
}

impl Display for UnprefixedId {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.value.chars().try_for_each(|char| match char {
            '\r' => f.write_str("\\r"),
            '\n' => f.write_str("\\n"),
            '\u{000c}' => f.write_str("\\f"),
            ' ' => f.write_str("\\ "),
            '\t' => f.write_str("\\t"),
            ':' => f.write_str("\\:"),
            _ => f.write_char(char),
        })
    }
}

impl FromPair for UnprefixedId {
    const RULE: Rule = Rule::UnprefixedId;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        let mut local = String::with_capacity(pair.as_str().len());
        let mut chars = pair.as_str().chars();
        while let Some(char) = chars.next() {
            if char == '\\' {
                match chars.next() {
                    Some('r') => local.push('\r'),
                    Some('n') => local.push('\n'),
                    Some('f') => local.push('\u{000c}'),
                    Some('t') => local.push('\t'),
                    Some(other) => local.push(other),
                    None => panic!("missing stuff"), // FIXME(@althonos)
                }
            } else {
                local.push(char);
            }
        }

        Ok(UnprefixedId::new(local))
    }
}
impl_fromstr!(UnprefixedId);

#[cfg(test)]
mod tests {

    use std::str::FromStr;
    use std::string::ToString;
    use super::*;

    #[test]
    fn from_str() {
        let actual = UnprefixedId::from_str("biological_process").unwrap();
        let expected = UnprefixedId::new("biological_process");
        assert_eq!(actual, expected);

        assert!(UnprefixedId::from_str("").is_err());
        assert!(UnprefixedId::from_str("Some\nthing:spanning").is_err());
        assert!(UnprefixedId::from_str("goslim_plant remaining").is_err());
    }

    #[test]
    fn to_string() {
        let id = UnprefixedId::new("something:with:colons");
        assert_eq!(id.to_string(), "something\\:with\\:colons");
    }
}