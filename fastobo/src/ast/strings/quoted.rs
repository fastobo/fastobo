use std::borrow::Borrow;
use std::borrow::ToOwned;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::ops::Deref;

use pest::iterators::Pair;
use opaque_typedef::OpaqueTypedefUnsized;

use crate::error::Error;
use crate::parser::FromPair;
use crate::parser::QuickFind;
use crate::parser::Rule;
use crate::share::Share;
use crate::share::Cow;
use crate::share::Redeem;
use super::escape;
use super::unescape;

/// A string enclosed by quotes, used for definitions.
///
/// This type is mostly just a wrapper for `String` that patches `FromStr` and
/// `Display` so that it can read and write quoted strings in OBO documents.
///
/// # Usage
/// Use `FromStr` to parse the serialized representation of a `QuotedString`,
/// and `QuotedString::new` to create a quoted string with its content set
/// from an `Into<String>` implementor.
///
/// # Example
/// ```rust
/// # extern crate fastobo;
/// # use fastobo::ast::QuotedString;
/// let s = QuotedString::new("Hello, world!");
/// assert_eq!(s.as_str(), "Hello, world!");
/// assert_eq!(s.to_string(), "\"Hello, world!\"");
/// ```
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct QuotedString {
    value: String,
}

impl QuotedString {
    /// Create a new `QuotedString` from an unescaped string.
    pub fn new<S>(s: S) -> Self
    where
        S: Into<String>
    {
        QuotedString { value: s.into() }
    }

    /// Extracts a string slice containing the `QuotedString` value.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl AsRef<str> for QuotedString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<QuotedStr> for QuotedString {
    fn as_ref(&self) -> &QuotedStr {
        self.share()
    }
}

impl Borrow<QuotedStr> for QuotedString {
    fn borrow(&self) -> &QuotedStr {
        QuotedStr::new(self.as_ref())
    }
}

impl Deref for QuotedString {
    type Target = QuotedStr;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl Display for QuotedString {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let s: &QuotedStr = self.borrow();
        s.fmt(f)
    }
}

impl<S> From<S> for QuotedString
where
    S: Into<String>
{
    fn from(s: S) -> Self {
        Self::new(s)
    }
}

impl<'i> FromPair<'i> for QuotedString {
    const RULE: Rule = Rule::QuotedString;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self, Error> {
        let s = pair.as_str();
        let escaped = s.quickcount(b'\\');
        let mut local = String::with_capacity(s.len() + escaped);
        unescape(&mut local, s.get_unchecked(1..s.len() - 1))
            .expect("String as fmt::Write cannot fail");
        Ok(QuotedString::new(local))
    }
}
impl_fromstr!(QuotedString);

impl PartialEq<str> for QuotedString {
    fn eq(&self, other: &str) -> bool {
        self.value == other
    }
}

impl PartialEq<String> for QuotedString {
    fn eq(&self, other: &String) -> bool {
        self.value == other.as_str()
    }
}

impl<'a> Share<'a, &'a QuotedStr> for QuotedString {
    fn share(&'a self) -> &'a QuotedStr {
        QuotedStr::new(&self.value)
    }
}

/// A borrowed `QuotedString`.
#[derive(Debug, Eq, Hash, PartialEq, OpaqueTypedefUnsized)]
#[opaque_typedef(derive(Deref, AsRef(Inner, Self)))]
#[repr(transparent)]
pub struct QuotedStr(str);

impl QuotedStr {
    /// Create a new `QuotedStr`.
    pub fn new(s: &str) -> &Self {
        // Using `unchecked` because there is no validation needed.
        unsafe { QuotedStr::from_inner_unchecked(s) }
    }
}

impl<'a> Display for QuotedStr {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_char('"')
            .and(escape(f, &self.0))
            .and(f.write_char('"'))
    }
}

impl<'i> FromPair<'i> for Cow<'i, &'i QuotedStr> {
    const RULE: Rule = Rule::QuotedString;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, Error> {
        if pair.as_str().quickfind(b'\\').is_some() {
            QuotedString::from_pair_unchecked(pair).map(Cow::Owned)
        } else {
            Ok(Cow::Borrowed(QuotedStr::new(pair.as_str())))
        }
    }
}
impl_fromslice!('i, Cow<'i, &'i QuotedStr>);

impl PartialEq<str> for QuotedStr {
    fn eq(&self, other: &str) -> bool {
        &self.0 == other
    }
}

impl PartialEq<String> for QuotedStr {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other.as_str()
    }
}

impl<'a> Redeem<'a> for &'a QuotedStr {
    type Owned = QuotedString;
    fn redeem(&self) -> QuotedString {
        QuotedString::new(self.0.to_owned())
    }
}

impl ToOwned for QuotedStr {
    type Owned = QuotedString;
    fn to_owned(&self) -> QuotedString {
        QuotedString::new(self.0.to_owned())
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;
    use std::string::ToString;
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn from_str() {
        let actual = QuotedString::from_str("\"something in quotes\"");
        let expected = QuotedString::new(String::from("something in quotes"));
        assert_eq!(expected, actual.unwrap());

        let actual = QuotedString::from_str("\"something in \\\"escaped\\\" quotes\"");
        let expected = QuotedString::new(String::from("something in \"escaped\" quotes"));
        assert_eq!(expected, actual.unwrap());
    }
}
