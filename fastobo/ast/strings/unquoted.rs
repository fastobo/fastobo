use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::ops::Deref;

use opaque_typedef::OpaqueTypedefUnsized;
use pest::iterators::Pair;

use crate::error::Error;
use crate::parser::FromPair;
use crate::parser::Rule;
use crate::borrow::Borrow;
use crate::borrow::Cow;
use crate::borrow::ToOwned;
use super::escape;
use super::unescape;

/// A string without delimiters, used as values in different clauses.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UnquotedString {
    value: Cow<'static, &'static str>,
}

impl UnquotedString {
    /// Create a new `UnquotedString`.
    pub fn new(s: String) -> Self {
        UnquotedString { value: s.into() }
    }

    /// Extracts a string slice containing the `UnquotedString` value.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl AsRef<str> for UnquotedString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<UnquotedStr> for UnquotedString {
    fn as_ref(&self) -> &UnquotedStr {
        UnquotedStr::new(self.as_ref())
    }
}

impl Deref for UnquotedString {
    type Target = UnquotedStr;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl std::borrow::Borrow<UnquotedStr> for UnquotedString {
    fn borrow(&self) -> &UnquotedStr {
        UnquotedStr::new(self.as_ref())
    }
}

impl<'a> Borrow<'a, &'a UnquotedStr> for UnquotedString {
    fn borrow(&'a self) -> &'a UnquotedStr {
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
        unescape(&mut local, s).expect("String as fmt::Write cannot fail");
        Ok(UnquotedString::new(local))
    }
}
impl_fromstr!(UnquotedString);

/// A borrowed `UnquotedString`.
#[derive(Debug, Eq, Hash, PartialEq, OpaqueTypedefUnsized)]
#[opaque_typedef(derive(Deref, AsRef(Inner, Self)))]
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

impl<'i> FromPair<'i> for Cow<'i, &'i UnquotedStr> {
    const RULE: Rule = Rule::UnquotedString;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, Error> {
        if pair.as_str().find('\\').is_some() {
            UnquotedString::from_pair_unchecked(pair).map(|s| Cow::Owned(s))
        } else {
            Ok(Cow::Borrowed(UnquotedStr::new(pair.as_str())))
        }
    }
}
impl_fromslice!('i, Cow<'i, &'i UnquotedStr>);

impl std::borrow::ToOwned for UnquotedStr {
    type Owned = UnquotedString;
    fn to_owned(&self) -> UnquotedString {
        UnquotedString::new(self.0.to_owned())
    }
}

impl<'a> ToOwned<'a> for &'a UnquotedStr {
    type Owned = UnquotedString;
    fn to_owned(&'a self) -> UnquotedString {
        UnquotedString::new(self.0.to_owned())
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
