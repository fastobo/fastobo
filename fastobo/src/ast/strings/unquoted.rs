use std::borrow::Borrow;
use std::borrow::ToOwned;
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
use crate::parser::QuickFind;
use crate::share::Share;
use crate::share::Cow;
use crate::share::Redeem;
use super::escape;
use super::unescape;

/// A string without delimiters, used as values in different clauses.
///
/// This type is mostly just a wrapper for `String` that patches `FromStr` and
/// `Display` so that it can read and write unquoted strings in OBO documents.
///
/// # Usage
/// Use `FromStr` to parse the serialized representation of a `UnquotedString`,
/// and `UnquotedString::new` to create a quoted string with its content set
/// from a Rust `String` passed as argument.
///
/// To get the the unescaped `String`, use `UnquotedString::into_string`, or
/// use `ToString::to_string` to obtained a serialized (escaped) version of
/// the unquoted string.
///
/// # Example
/// ```rust
/// # extern crate fastobo;
/// # use fastobo::ast::UnquotedString;
/// let s = UnquotedString::new("Hello, world!");
/// assert_eq!(s.to_string(), "Hello, world!");
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, OpaqueTypedef, PartialEq, PartialOrd)]
#[opaque_typedef(derive(AsRef(Inner, Self), Into(Inner)))]
pub struct UnquotedString {
    value: String,
}

impl UnquotedString {
    /// Create a new `UnquotedString` from an unescaped string.
    pub fn new<S>(s: S) -> Self
    where
        S: Into<String>
    {
        UnquotedString { value: s.into() }
    }

    /// Extracts a string slice containing the `UnquotedString` value.
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Retrieve the underlying unescaped string from the `UnquotedString`.
    pub fn into_string(self) -> String {
        self.value
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

impl Borrow<UnquotedStr> for UnquotedString {
    fn borrow(&self) -> &UnquotedStr {
        UnquotedStr::new(self.as_ref())
    }
}

impl Deref for UnquotedString {
    type Target = UnquotedStr;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
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
        let escaped = s.quickcount(b'\\'); // number of escaped characters
        let mut local = String::with_capacity(s.len() + escaped);
        unescape(&mut local, s).expect("String as fmt::Write cannot fail");
        Ok(UnquotedString::new(local))
    }
}
impl_fromstr!(UnquotedString);

impl<'a> Share<'a, &'a UnquotedStr> for UnquotedString {
    fn share(&'a self) -> &'a UnquotedStr {
        UnquotedStr::new(&self.value)
    }
}

/// A borrowed `UnquotedString`.
#[derive(Debug, Eq, Hash, OpaqueTypedefUnsized, Ord, PartialEq, PartialOrd)]
#[opaque_typedef(derive(Deref, AsRef(Inner, Self)))]
#[repr(transparent)]
pub struct UnquotedStr(str);

impl UnquotedStr {
    /// Create a new `QuotedStr`.
    pub fn new(s: &str) -> &Self {
        unsafe { UnquotedStr::from_inner_unchecked(s) }
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
        if pair.as_str().quickfind(b'\\').is_some() {
            UnquotedString::from_pair_unchecked(pair).map(Cow::Owned)
        } else {
            Ok(Cow::Borrowed(UnquotedStr::new(pair.as_str())))
        }
    }
}
impl_fromslice!('i, Cow<'i, &'i UnquotedStr>);

impl ToOwned for UnquotedStr {
    type Owned = UnquotedString;
    fn to_owned(&self) -> UnquotedString {
        UnquotedString::new(self.0.to_owned())
    }
}

impl<'a> Redeem<'a> for &'a UnquotedStr {
    type Owned = UnquotedString;
    fn redeem(&'a self) -> UnquotedString {
        UnquotedString::new(self.0.to_owned())
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
        let actual = UnquotedString::from_str("something\\ttabbed");
        let expected = UnquotedString::new(String::from("something\ttabbed"));
        assert_eq!(expected, actual.unwrap());

        let actual = UnquotedString::from_str("namespace-id-rule").unwrap();
        let expected = UnquotedString::new(String::from("namespace-id-rule"));
        assert_eq!(expected, actual);
    }
}
