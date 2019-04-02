use std::borrow::Borrow;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use pest::iterators::Pair;
use opaque_typedef::OpaqueTypedefUnsized;

use crate::error::Error;
use crate::parser::FromPair;
use crate::parser::Rule;
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
/// from a Rust `String` passed as argument.
///
/// # Example
/// ```rust
/// # extern crate fastobo;
/// # use fastobo::ast::QuotedString;
/// let s = QuotedString::new("Hello, world!");
/// assert_eq!(s.as_ref(), "Hello, world!");
/// assert_eq!(s.to_string(), "\"Hello, world!\"");
/// ```
#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub struct QuotedString {
    value: String,
}

impl QuotedString {
    /// Create a new `QuotedString`.
    pub fn new<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        QuotedString { value: s.into() }
    }
}

impl AsRef<str> for QuotedString {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl Borrow<QuotedStr> for QuotedString {
    fn borrow(&self) -> &QuotedStr {
        QuotedStr::new(&self.value)
    }
}

impl Display for QuotedString {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let s: &QuotedStr = self.borrow();
        s.fmt(f)
    }
}

impl<'i> FromPair<'i> for QuotedString {
    const RULE: Rule = Rule::QuotedString;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self, Error> {
        let s = pair.as_str();
        let mut local = String::with_capacity(s.len());
        unescape(&mut local, s.get_unchecked(1..s.len() - 1))
            .expect("String as fmt::Write cannot fail");
        Ok(QuotedString::new(local))
    }
}
impl_fromstr!(QuotedString);

/// A borrowed `QuotedString`.
#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, OpaqueTypedefUnsized)]
#[opaque_typedef(derive(
    AsRef(Deref, Self),
    FromInner,
    Deref,
    Into(Arc, Box, Rc, Inner),
    PartialEq(Inner, InnerRev, InnerCow, InnerCowRev, SelfCow, SelfCowRev),
    PartialOrd(Inner, InnerRev, InnerCow, InnerCowRev, SelfCow, SelfCowRev)
))]
#[repr(transparent)]
pub struct QuotedStr(str);

impl QuotedStr {
    /// Create a new `QuotedStr`.
    pub fn new(s: &str) -> &Self {
        // Using `unchecked` because there is no validation needed.
        unsafe { QuotedStr::from_inner_unchecked(s.as_ref()) }
    }
}

impl<'a> Display for QuotedStr {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_char('"')
            .and(escape(f, &self.0))
            .and(f.write_char('"'))
    }
}

impl ToOwned for QuotedStr {
    type Owned = QuotedString;
    fn to_owned(&self) -> QuotedString {
        QuotedString::new(self.0.to_string())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::str::FromStr;
    use std::string::ToString;

    #[test]
    fn from_str() {
        let actual = QuotedString::from_str("\"something in quotes\"");
        let expected = QuotedString::new("something in quotes");
        assert_eq!(expected, actual.unwrap());

        let actual = QuotedString::from_str("\"something in \\\"escaped\\\" quotes\"");
        let expected = QuotedString::new("something in \"escaped\" quotes");
        assert_eq!(expected, actual.unwrap());
    }
}
