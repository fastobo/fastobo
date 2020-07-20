use std::borrow::Borrow;
use std::borrow::ToOwned;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::ops::Deref;

use opaque_typedef::OpaqueTypedef;
use opaque_typedef::OpaqueTypedefUnsized;
use pest::iterators::Pair;

use crate::ast::StringType;
use crate::error::Error;
use crate::error::SyntaxError;
use crate::parser::FromPair;
use crate::parser::QuickFind;
use crate::parser::Rule;

fn escape<W: Write>(f: &mut W, s: &str) -> FmtResult {
    s.chars().try_for_each(|char| match char {
        '\r' => f.write_str("\\r"),
        '\n' => f.write_str("\\n"),
        '\u{000c}' => f.write_str("\\f"),
        ' ' => f.write_str("\\ "),
        '\t' => f.write_str("\\t"),
        ':' => f.write_str("\\:"),
        '"' => f.write_str("\\\""),
        '\\' => f.write_str("\\\\"),
        _ => f.write_char(char),
    })
}

fn unescape<W: Write>(f: &mut W, s: &str) -> FmtResult {
    let mut chars = s.chars();
    while let Some(char) = chars.next() {
        if char == '\\' {
            match chars.next() {
                Some('r') => f.write_char('\r')?,
                Some('n') => f.write_char('\n')?,
                Some('f') => f.write_char('\u{000c}')?,
                Some('t') => f.write_char('\t')?,
                Some(other) => f.write_char(other)?,
                None => panic!("invalid escape"), // FIXME(@althonos)
            }
        } else {
            f.write_char(char)?;
        }
    }
    Ok(())
}

/// An identifier without a prefix.
#[derive(Clone, Debug, Hash, Eq, OpaqueTypedef, Ord, PartialEq, PartialOrd)]
#[opaque_typedef(derive(FromInner))]
pub struct UnprefixedIdent(StringType);

impl UnprefixedIdent {
    /// Create a new unprefixed identifier.
    pub fn new<S>(id: S) -> Self
    where
        S: Into<StringType>,
    {
        // FIXME: check the given string is a valid unprefixed identifier.
        Self(id.into())
    }

    /// Return a reference to the underlying `str`.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for UnprefixedIdent {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<UnprefixedId> for UnprefixedIdent {
    fn as_ref(&self) -> &UnprefixedId {
        UnprefixedId::new(&self.0)
    }
}

impl Borrow<UnprefixedId> for UnprefixedIdent {
    fn borrow(&self) -> &UnprefixedId {
        UnprefixedId::new(&self.0)
    }
}

impl Display for UnprefixedIdent {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        UnprefixedId::new(&self.0).fmt(f)
    }
}

impl From<UnprefixedIdent> for StringType {
    fn from(id: UnprefixedIdent) -> Self {
        id.0
    }
}

#[cfg(feature = "smartstring")]
impl From<UnprefixedIdent> for String {
    fn from(id: UnprefixedIdent) -> Self {
        id.0.into()
    }
}

impl From<&str> for UnprefixedIdent {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl<'i> FromPair<'i> for UnprefixedIdent {
    const RULE: Rule = Rule::UnprefixedId;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, SyntaxError> {
        let s = pair.as_str();
        let escaped = s.quickcount(b'\\');
        let mut local = String::with_capacity(s.len() + escaped);
        unescape(&mut local, s).expect("fmt::Write cannot fail on a String");
        Ok(Self::new(local))
    }
}
impl_fromstr!(UnprefixedIdent);

/// A borrowed `UnprefixedIdent`.
#[derive(Debug, Eq, Hash, OpaqueTypedefUnsized, Ord, PartialEq, PartialOrd)]
#[opaque_typedef(derive(Deref, AsRef(Inner, Self)))]
#[repr(transparent)]
pub struct UnprefixedId(str);

impl UnprefixedId {
    pub fn new(s: &str) -> &Self {
        unsafe { Self::from_inner_unchecked(s) }
    }
}

impl Display for UnprefixedId {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        escape(f, &self.0)
    }
}

impl ToOwned for UnprefixedId {
    type Owned = UnprefixedIdent;
    fn to_owned(&self) -> UnprefixedIdent {
        UnprefixedIdent::new(self.0.to_owned())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;
    use std::string::ToString;

    #[test]
    fn from_str() {
        let actual = UnprefixedIdent::from_str("biological_process").unwrap();
        let expected = UnprefixedIdent::new(String::from("biological_process"));
        self::assert_eq!(actual, expected);

        assert!(UnprefixedIdent::from_str("").is_err());
        assert!(UnprefixedIdent::from_str("Some\nthing:spanning").is_err());
        assert!(UnprefixedIdent::from_str("goslim_plant remaining").is_err());
    }

    #[test]
    fn to_string() {
        let id = UnprefixedIdent::new(String::from("something:with:colons"));
        self::assert_eq!(id.to_string(), "something\\:with\\:colons");
    }
}
