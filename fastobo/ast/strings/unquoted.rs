use std::borrow::Borrow;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use opaque_typedef::OpaqueTypedefUnsized;
use pest::iterators::Pair;

use crate::error::Error;
use crate::parser::FromPair;
use crate::parser::Rule;
use super::escape;
use super::unescape;

/// A string without delimiters, used as values in different clauses.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UnquotedString {
    value: String,
}

impl UnquotedString {
    /// Create a new `UnquotedString`.
    pub fn new<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        UnquotedString { value: s.into() }
    }
}

impl AsRef<str> for UnquotedString {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl Borrow<UnquotedStr> for UnquotedString {
    fn borrow(&self) -> &UnquotedStr {
        UnquotedStr::new(&self.value)
    }
}

impl Display for UnquotedString {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let s: &UnquotedStr = self.borrow();
        s.fmt(f)
    }
}

impl<'i> FromPair<'i> for UnquotedString {
    const RULE: Rule = Rule::UnquotedString;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, Error> {
        let s = pair.as_str();
        let mut local = String::with_capacity(s.len());
        let mut chars = s.chars();
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

        Ok(UnquotedString::new(local))
    }
}
impl_fromstr!(UnquotedString);

/// A borrowed `UnquotedString`.
#[derive(Debug, Eq, Hash, PartialEq, OpaqueTypedefUnsized)]
#[opaque_typedef(derive(
    AsRef(Deref, Self),
    FromInner,
    Deref,
    Into(Arc, Box, Rc, Inner),
    PartialEq(Inner, InnerRev, InnerCow, InnerCowRev, SelfCow, SelfCowRev),
    PartialOrd(Inner, InnerRev, InnerCow, InnerCowRev, SelfCow, SelfCowRev)
))]
#[repr(transparent)]
pub struct UnquotedStr(str);

impl UnquotedStr {
    /// Create a new `QuotedStr`.
    pub fn new(s: &str) -> &Self {
        unsafe { UnquotedStr::from_inner_unchecked(s.as_ref()) }
    }
}

impl<'a> Display for UnquotedStr {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        escape(f, &self.0)
    }
}

impl ToOwned for UnquotedStr {
    type Owned = UnquotedString;
    fn to_owned(&self) -> UnquotedString {
        UnquotedString::new(self.0.to_string())
    }
}






#[cfg(test)]
mod tests {

    use super::*;
    use std::str::FromStr;
    use std::string::ToString;

    #[test]
    fn from_str() {
        let actual = UnquotedString::from_str("something\\ttabbed");
        let expected = UnquotedString::new("something\ttabbed");
        assert_eq!(expected, actual.unwrap());

        let actual = UnquotedString::from_str("namespace-id-rule").unwrap();
        let expected = UnquotedString::new("namespace-id-rule");
        assert_eq!(expected, actual);
    }
}
