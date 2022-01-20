use std::borrow::Borrow;
use std::borrow::Cow;
use std::borrow::ToOwned;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::ops::Deref;

use fastobo_derive_internal::FromStr;
use pest::iterators::Pair;

use crate::ast::StringType;
use crate::error::SyntaxError;
use crate::parser::Cache;
use crate::parser::FromPair;
use crate::parser::QuickFind;
use crate::syntax::Rule;

// ---------------------------------------------------------------------------

fn escape<W: Write>(f: &mut W, s: &str) -> FmtResult {
    s.chars().try_for_each(|char| match char {
        '\r' => f.write_str("\\r"),
        '\n' => f.write_str("\\n"),
        '\u{000c}' => f.write_str("\\f"),
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

// ---------------------------------------------------------------------------

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
/// To get the the unescaped `String`, use `QuotedString::into_string`, or
/// use `ToString::to_string` to obtained a serialized (escaped) version of
/// the quoted string.
///
/// # Example
/// ```rust
/// # extern crate fastobo;
/// # use fastobo::ast::QuotedString;
/// let s = QuotedString::new("Hello, world!");
/// assert_eq!(s.as_str(), "Hello, world!");
/// assert_eq!(s.to_string(), "\"Hello, world!\"");
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, FromStr, Ord, PartialEq, PartialOrd)]
pub struct QuotedString(StringType);

impl QuotedString {
    /// Create a new `QuotedString` from an unescaped string.
    pub fn new<S>(s: S) -> Self
    where
        S: Into<StringType>,
    {
        QuotedString(s.into())
    }

    /// Extracts a string slice containing the `QuotedString` value.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Retrieve the underlying unescaped string from the `QuotedString`.
    pub fn into_string(self) -> String {
        self.0.into()
    }

    /// Retrieve the underlying unescaped inner string from the `QuotedString`.
    pub fn into_inner(self) -> StringType {
        self.0
    }
}

impl AsRef<str> for QuotedString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<QuotedStr> for QuotedString {
    fn as_ref(&self) -> &QuotedStr {
        QuotedStr::new(&self.0)
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
        <Self as Borrow<QuotedStr>>::borrow(self).fmt(f)
    }
}

impl From<&str> for QuotedString {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl<'i> FromPair<'i> for QuotedString {
    const RULE: Rule = Rule::QuotedString;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>, _cache: &Cache) -> Result<Self, SyntaxError> {
        let s = pair.as_str();
        let escaped = s.quickcount(b'\\');
        let mut local = String::with_capacity(s.len() + escaped);
        unescape(&mut local, s.get_unchecked(1..s.len() - 1))
            .expect("String as fmt::Write cannot fail");
        Ok(QuotedString::new(local))
    }
}

impl PartialEq<str> for QuotedString {
    fn eq(&self, other: &str) -> bool {
        self.0.as_str() == other
    }
}

#[cfg(feature = "smartstring")]
impl PartialEq<String> for QuotedString {
    fn eq(&self, other: &String) -> bool {
        self.0.as_str() == other.as_str()
    }
}

impl PartialEq<StringType> for QuotedString {
    fn eq(&self, other: &StringType) -> bool {
        self.0.as_str() == other.as_str()
    }
}

/// A borrowed `QuotedString`.
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct QuotedStr(str);

impl QuotedStr {
    /// Create a new `QuotedStr`.
    pub fn new(s: &str) -> &Self {
        unsafe { &*(s as *const str as *const Self) }
    }
}

impl AsRef<str> for QuotedStr {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for QuotedStr {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Display for QuotedStr {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_char('"')
            .and(escape(f, &self.0))
            .and(f.write_char('"'))
    }
}

impl<'i> FromPair<'i> for Cow<'i, QuotedStr> {
    const RULE: Rule = Rule::QuotedString;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        if pair.as_str().quickfind(b'\\').is_some() {
            QuotedString::from_pair_unchecked(pair, cache).map(Cow::Owned)
        } else {
            Ok(Cow::Borrowed(QuotedStr::new(pair.as_str())))
        }
    }
}

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

impl ToOwned for QuotedStr {
    type Owned = QuotedString;
    fn to_owned(&self) -> QuotedString {
        QuotedString::new(self.0.to_owned())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

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
