use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::ops::Deref;

use opaque_typedef::OpaqueTypedefUnsized;
use pest::iterators::Pair;

use crate::borrow::Borrow;
use crate::borrow::Cow;
use crate::borrow::ToOwned;
use crate::error::Error;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

/// An identifier without a prefix.
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct UnprefixedIdent {
    value: String,
}

impl UnprefixedIdent {
    /// Create a new unprefixed identifier.
    pub fn new(id: String) -> Self {
        Self { value: id }
    }
}

impl AsRef<str> for UnprefixedIdent {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl AsRef<UnprefixedId> for UnprefixedIdent {
    fn as_ref(&self) -> &UnprefixedId {
        UnprefixedId::new(&self.as_ref())
    }
}

impl<'a> Borrow<'a, &'a UnprefixedId> for UnprefixedIdent {
    fn borrow(&'a self) -> &'a UnprefixedId {
        self.as_ref()
    }
}

impl Deref for UnprefixedIdent {
    type Target = UnprefixedId;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl Display for UnprefixedIdent {
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

impl<'i> FromPair<'i> for UnprefixedIdent {
    const RULE: Rule = Rule::UnprefixedId;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
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

        Ok(Self::new(local))
    }
}
impl_fromstr!(UnprefixedIdent);

/// A borrowed `UnprefixedIdentifier`.
#[derive(Debug, Eq, Hash, PartialEq, OpaqueTypedefUnsized)]
#[repr(transparent)]
pub struct UnprefixedId(str);

impl UnprefixedId {
    /// Create a new `UnprefixedId`.
    pub fn new(s: &str) -> &Self {
        unsafe { UnprefixedId::from_inner_unchecked(s.as_ref()) }
    }
}

impl AsRef<str> for UnprefixedId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// TODO(@althonos)
// impl Display for UnprefixedId {}
impl<'i> FromPair<'i> for Cow<'i, &'i UnprefixedId> {
    const RULE: Rule = Rule::UnprefixedId;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        if pair.as_str().find('\\').is_some() {
            UnprefixedIdent::from_pair_unchecked(pair).map(Cow::Owned)
        } else {
            Ok(Cow::Borrowed(UnprefixedId::new(pair.as_str())))
        }
    }
}
impl_fromslice!('i, Cow<'i, &'i UnprefixedId>);

impl<'a> ToOwned<'a> for &'a UnprefixedId {
    type Owned = UnprefixedIdent;
    fn to_owned(&'a self) -> UnprefixedIdent {
        UnprefixedIdent::new(self.0.to_string())
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use std::str::FromStr;
    use std::string::ToString;

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
