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
        // ':' => f.write_str("\\:"),
        '!' => f.write_str("\\!"),
        '{' => f.write_str("\\{"),
        '}' => f.write_str("\\}"),
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
/// assert_eq!(s.to_string(), "Hello, world\\!");
/// ```
#[derive(Clone, Debug, Default, Eq, FromStr, Hash, Ord, PartialEq, PartialOrd)]
pub struct UnquotedString(StringType);

impl UnquotedString {
    /// Create a new `UnquotedString` from an unescaped string.
    pub fn new<S>(s: S) -> Self
    where
        S: Into<StringType>,
    {
        UnquotedString(s.into())
    }

    /// Extracts a string slice containing the `UnquotedString` value.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Retrieve the underlying unescaped string from the `UnquotedString`.
    pub fn into_string(self) -> String {
        self.0.into()
    }

    /// Retrieve the underlying unescaped inner string from the `UnquotedString`.
    pub fn into_inner(self) -> StringType {
        self.0
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
        <Self as Borrow<UnquotedStr>>::borrow(self).fmt(f)
    }
}

impl From<&str> for UnquotedString {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl PartialEq<str> for UnquotedString {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<StringType> for UnquotedString {
    fn eq(&self, other: &StringType) -> bool {
        self.as_str() == other.as_str()
    }
}

#[cfg(feature = "smartstring")]
impl PartialEq<String> for UnquotedString {
    fn eq(&self, other: &String) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<'i> FromPair<'i> for UnquotedString {
    const RULE: Rule = Rule::UnquotedString;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        _cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        let s = pair.as_str();
        let escaped = s.quickcount(b'\\'); // number of escaped characters
        let mut local = String::with_capacity(s.len() + escaped);
        unescape(&mut local, s).expect("String as fmt::Write cannot fail");
        Ok(UnquotedString::new(local))
    }
}

/// A borrowed `UnquotedString`.
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct UnquotedStr(str);

impl UnquotedStr {
    /// Create a new `QuotedStr`.
    pub fn new(s: &str) -> &Self {
        unsafe { &*(s as *const str as *const Self) }
    }
}

impl AsRef<str> for UnquotedStr {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for UnquotedStr {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Display for UnquotedStr {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        escape(f, &self.0)
    }
}

impl<'i> FromPair<'i> for Cow<'i, UnquotedStr> {
    const RULE: Rule = Rule::UnquotedString;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        if pair.as_str().quickfind(b'\\').is_some() {
            UnquotedString::from_pair_unchecked(pair, cache).map(Cow::Owned)
        } else {
            Ok(Cow::Borrowed(UnquotedStr::new(pair.as_str())))
        }
    }
}

impl PartialEq<str> for UnquotedStr {
    fn eq(&self, other: &str) -> bool {
        &self.0 == other
    }
}

impl PartialEq<String> for UnquotedStr {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other.as_str()
    }
}

impl ToOwned for UnquotedStr {
    type Owned = UnquotedString;
    fn to_owned(&self) -> UnquotedString {
        UnquotedString::new(self.0.to_owned())
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
        let actual = UnquotedString::from_str("something\\ttabbed");
        let expected = UnquotedString::new(String::from("something\ttabbed"));
        assert_eq!(expected, actual.unwrap());

        let actual = UnquotedString::from_str("namespace-id-rule").unwrap();
        let expected = UnquotedString::new(String::from("namespace-id-rule"));
        assert_eq!(expected, actual);
    }

    #[test]
    fn to_string() {
        let actual = UnquotedString::new("(?<=[KR])(?!P)");
        let expected = "(?<=[KR])(?\\!P)";
        assert_eq!(&actual.to_string(), expected);
    }
}
