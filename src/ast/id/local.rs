use std::cmp::Ordering;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::hash::Hash;
use std::hash::Hasher;

use opaque_typedef::OpaqueTypedefUnsized;
use pest::iterators::Pair;

use crate::error::Error;
use crate::error::SyntaxError;
use crate::parser::FromPair;
use crate::parser::QuickFind;
use crate::parser::Rule;
use crate::share::Cow;
use crate::share::Redeem;
use crate::share::Share;

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

fn is_canonical<S: AsRef<str>>(s: S) -> bool {
    s.as_ref().chars().all(|c| c.is_ascii_digit())
}

/// A local identifier, preceded by a prefix in prefixed IDs.
///
/// * A canonical local ID only contains digits (`[0-9]`).
/// * A non-canonical local ID can contain any character excepting
///   whitespaces and newlines.
///
/// # Example
/// ```rust
/// # extern crate fastobo;
/// # use fastobo::ast::PrefixedIdent;
/// # use std::str::FromStr;
/// let id = PrefixedIdent::from_str("GO:0046154").unwrap();
/// assert!(id.local().is_canonical());
/// assert_eq!(id.local(), "0046154");
/// ```
#[derive(Clone, Debug, Ord, Eq)]
pub struct IdentLocal {
    value: String,
    canonical: bool,
}

impl IdentLocal {
    /// Create a new local identifier.
    pub fn new<S>(local: S) -> Self
    where
        S: Into<String>,
    {
        let value = local.into();
        Self {
            canonical: is_canonical(&value),
            value,
        }
    }

    /// Check if the local identifier is canonical or not.
    pub fn is_canonical(&self) -> bool {
        self.canonical
    }

    /// Create a new `IdLocal` without checking if it is canonical.
    pub unsafe fn new_unchecked(s: String, canonical: bool) -> Self {
        Self {
            value: s,
            canonical,
        }
    }

    /// Get the local identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Extract the unescaped local identifier as a `String`.
    pub fn into_string(self) -> String {
        self.value
    }
}

impl AsRef<str> for IdentLocal {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl From<IdentLocal> for String {
    fn from(id: IdentLocal) -> Self {
        id.value
    }
}

impl From<String> for IdentLocal {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&str> for IdentLocal {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl Display for IdentLocal {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.share().fmt(f)
    }
}

impl<'i> FromPair<'i> for IdentLocal {
    const RULE: Rule = Rule::IdLocal;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, SyntaxError> {
        // Bail out if the local ID is canonical (digits only).
        let inner = pair.into_inner().next().unwrap();
        if inner.as_rule() == Rule::CanonicalIdLocal {
            return Ok(Self::new_unchecked(inner.as_str().to_string(), true));
        }

        // Unescape the local ID if it is non canonical.
        let s = inner.as_str();
        let escaped = s.quickcount(b'\\');
        let mut local = String::with_capacity(s.len() + escaped);
        unescape(&mut local, s).expect("fmt::Write cannot fail on a String");

        // FIXME(@althonos): possible syntax issue, which uses a non-canonical
        //                   rule on canonical local IDs (workaround is to check
        //                   one more time if the local ID is canonical)
        Ok(Self::new(local))
    }
}
impl_fromstr!(IdentLocal);

impl Hash for IdentLocal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}

impl PartialEq for IdentLocal {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialEq<str> for IdentLocal {
    fn eq(&self, other: &str) -> bool {
        self.value == other
    }
}

impl PartialOrd for IdentLocal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<'a> Share<'a, IdLocal<'a>> for IdentLocal {
    fn share(&'a self) -> IdLocal<'a> {
        unsafe { IdLocal::new_unchecked(&self.value, self.canonical) }
    }
}

/// A borrowed `IdLocal`.
#[derive(Clone, Debug, Ord, PartialEq, Hash, Eq)]
pub struct IdLocal<'a> {
    value: &'a str,
    canonical: bool,
}

impl<'a> IdLocal<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            canonical: is_canonical(s),
            value: s,
        }
    }

    pub unsafe fn new_unchecked(s: &'a str, canonical: bool) -> Self {
        Self {
            value: s,
            canonical,
        }
    }
}

impl<'a> AsRef<str> for IdLocal<'a> {
    fn as_ref(&self) -> &str {
        self.value
    }
}

impl<'a> Display for IdLocal<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if self.canonical {
            f.write_str(&self.value)
        } else {
            escape(f, &self.value)
        }
    }
}

impl<'i> FromPair<'i> for Cow<'i, IdLocal<'i>> {
    const RULE: Rule = Rule::IdLocal;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, SyntaxError> {
        let inner = pair.into_inner().next().unwrap();
        if inner.as_rule() == Rule::CanonicalIdLocal {
            Ok(Cow::Borrowed(IdLocal::new_unchecked(inner.as_str(), true)))
        } else if inner.as_str().find('\\').is_some() {
            IdentLocal::from_pair_unchecked(inner).map(Cow::Owned)
        } else {
            Ok(Cow::Borrowed(IdLocal::new_unchecked(inner.as_str(), false)))
        }
    }
}
impl_fromslice!('i, Cow<'i, IdLocal<'i>>);

impl<'a> PartialEq<str> for IdLocal<'a> {
    fn eq(&self, other: &str) -> bool {
        self.value == other
    }
}

impl<'a> PartialOrd for IdLocal<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<'a> Redeem<'a> for IdLocal<'a> {
    type Owned = IdentLocal;
    fn redeem(&self) -> Self::Owned {
        unsafe { IdentLocal::new_unchecked(self.value.to_owned(), self.canonical) }
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
        let local = IdentLocal::from_str("0001").unwrap();
        self::assert_eq!(local.as_ref(), "0001");
        assert!(local.is_canonical());

        let local = IdentLocal::from_str("\\0001").unwrap();
        self::assert_eq!(local.as_ref(), "0001");
        assert!(local.is_canonical());

        let local = IdentLocal::from_str("0F").unwrap();
        self::assert_eq!(local.as_ref(), "0F");
        assert!(!local.is_canonical());

        assert!(IdentLocal::from_str("ABC\nDEF").is_err());
    }

    #[test]
    fn to_string() {
        self::assert_eq!(IdentLocal::new("0001").to_string(), "0001");
        self::assert_eq!(IdentLocal::new(":001").to_string(), "\\:001");
        self::assert_eq!(IdentLocal::new("0 01").to_string(), "0\\ 01");
    }

}
