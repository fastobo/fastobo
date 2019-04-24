use std::cmp::Ordering;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use opaque_typedef::OpaqueTypedefUnsized;
use pest::iterators::Pair;

use crate::share::Share;
use crate::share::Cow;
use crate::share::Redeem;
use crate::error::Error;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;
use crate::parser::QuickFind;


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
        _ => f.write_char(char)
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
    let string = s.as_ref();
    let mut chars = string.chars();
    if let Some(c) = chars.next() {
        c.is_ascii_alphabetic() && chars.all(|ref c| char::is_ascii_alphanumeric(c))
    } else {
        false
    }
}


/// An identifier prefix, either canonical or non-canonical.
///
/// * A canonical ID prefix only contains alphabetic characters (`[a-zA-Z]`)
///   followed by either an underscore or other alphabetic characters.
/// * A non-canonical ID prefix can contain any character besides `:`.
///
/// # Example
/// ```rust
/// # extern crate fastobo;
/// # use fastobo::ast::PrefixedIdent;
/// # use std::str::FromStr;
/// let id = PrefixedIdent::from_str("GO:0046154").unwrap();
/// assert!(id.prefix.is_canonical());
/// assert_eq!(&id.prefix, "GO");
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq)]
pub struct IdentPrefix {
    value: String,
    canonical: bool,
}

impl IdentPrefix {
    /// Create a new identifier prefix.
    pub fn new<S>(prefix: S) -> Self
    where
        S: Into<String>
    {
        let value = prefix.into();
        Self {
            canonical: is_canonical(&value),
            value,
        }
    }

    /// Check if the identifier prefix is canonical or not.
    ///
    /// # Example
    /// ```rust
    /// # extern crate fastobo;
    /// # use fastobo::ast::IdentPrefix;
    /// assert!(IdentPrefix::new(String::from("GO")).is_canonical());
    /// ```
    pub fn is_canonical(&self) -> bool {
        self.canonical
    }

    /// Create a new `IdPrefix` without checking if it is canonical or not.
    ///
    /// This is unsafe because the `canonical` flag will be used to determine
    /// if the prefix needs escaping. If not set right, the syntax of the
    /// produced serialization could be invalid.
    pub unsafe fn new_unchecked(s: String, canonical: bool) -> Self {
        Self { value: s, canonical }
    }

    /// Get the prefix as a string slice.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl AsRef<str> for IdentPrefix {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl Display for IdentPrefix {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.share().fmt(f)
    }
}

impl<S> From<S> for IdentPrefix
where
    S: Into<String>
{
    fn from(s: S) -> Self {
        Self::new(s)
    }
}

impl<'i> FromPair<'i> for IdentPrefix {
    const RULE: Rule = Rule::IdPrefix;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        // Bail out if the local prefix is canonical (alphanumeric only)
        let inner = pair.into_inner().next().unwrap();
        if inner.as_rule() == Rule::CanonicalIdPrefix {
            return Ok(Self::new_unchecked(inner.as_str().to_string(), true));
        }

        // Unescape the prefix if is non canonical.
        let s = inner.as_str();
        let escaped = s.quickcount(b'\\');
        let mut local = String::with_capacity(s.len() + escaped);
        unescape(&mut local, s).expect("fmt::Write cannot fail on a String");

        Ok(Self::new_unchecked(local, false))
    }
}
impl_fromstr!(IdentPrefix);

impl PartialEq<str> for IdentPrefix {
    fn eq(&self, other: &str) -> bool {
        self.value == other
    }
}

impl PartialOrd for IdentPrefix {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<'a> Share<'a, IdPrefix<'a>> for IdentPrefix {
    fn share(&'a self) -> IdPrefix<'a> {
        unsafe { IdPrefix::new_unchecked(&self.value, self.canonical) }
    }
}


/// A borrowed `IdentPrefix`
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq)]
pub struct IdPrefix<'a> {
    value: &'a str,
    canonical: bool,
}

impl<'a> IdPrefix<'a> {
    /// Create a new `IdPrf` from a borrowed string slice.
    pub fn new(s: &'a str) -> Self {
        Self {
            canonical: is_canonical(s),
            value: s,
        }
    }

    pub fn as_str(&self) -> &'a str {
        self.value
    }

    /// Create a new `IdPrf` without checking if it is canonical or not.
    pub unsafe fn new_unchecked(s: &'a str, canonical: bool) -> Self {
        Self { value: s, canonical }
    }
}

impl<'a> AsRef<str> for IdPrefix<'a> {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl<'a> Display for IdPrefix<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if self.canonical {
            f.write_str(&self.value)
        } else {
            escape(f, &self.value)
        }
    }
}

impl<'a> Into<&'a str> for IdPrefix<'a> {
    fn into(self) -> &'a str {
        self.value
    }
}

impl<'i> FromPair<'i> for Cow<'i, IdPrefix<'i>> {
    const RULE: Rule = Rule::IdPrefix;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let inner = pair.into_inner().next().unwrap();
        if inner.as_rule() == Rule::CanonicalIdPrefix {
            Ok(Cow::Borrowed(IdPrefix::new_unchecked(inner.as_str(), true)))
        } else if inner.as_str().find('\\').is_some() {
            IdentPrefix::from_pair_unchecked(inner).map(Cow::Owned)
        } else {
            Ok(Cow::Borrowed(IdPrefix::new_unchecked(inner.as_str(), false)))
        }
    }
}
impl_fromslice!('i, Cow<'i, IdPrefix<'i>>);

impl<'a> PartialEq<str> for IdPrefix<'a> {
    fn eq(&self, other: &str) -> bool {
        self.value == other
    }
}

impl<'a> PartialOrd for IdPrefix<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<'a> Redeem<'a> for IdPrefix<'a> {
    type Owned = IdentPrefix;
    fn redeem(&self) -> IdentPrefix {
        unsafe {
            IdentPrefix::new_unchecked(self.value.to_string(), self.canonical)
        }
    }
}


#[cfg(test)]
mod tests {

    use std::str::FromStr;
    use pretty_assertions::assert_eq;
    use super::*;

    #[test]
    fn from_str() {
        let prefix = IdentPrefix::from_str("GO").unwrap();
        self::assert_eq!(prefix, IdentPrefix::new(String::from("GO")));
        assert!(IdentPrefix::from_str("GO:").is_err());
    }

    #[test]
    fn to_string() {
        let prefix = IdentPrefix::new(String::from("GO"));
        self::assert_eq!(prefix.to_string(), "GO");

        let prefix = IdentPrefix::new(String::from("some thing"));
        self::assert_eq!(prefix.to_string(), "some\\ thing");
    }
}
