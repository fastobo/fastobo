use std::borrow::Borrow;
use std::borrow::Cow;
use std::borrow::ToOwned;
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
/// assert!(id.prefix().is_canonical());
/// assert_eq!(id.prefix(), "GO");
/// ```
#[derive(Clone, Debug, Eq, OpaqueTypedef, Ord)]
#[opaque_typedef(derive(FromInner))]
pub struct IdentPrefix {
    value: String,
}

impl IdentPrefix {
    /// Create a new identifier prefix.
    pub fn new<S>(prefix: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            value: prefix.into()
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
        // self.canonical
        is_canonical(&self.value)
    }

    // /// Create a new `IdPrefix` without checking if it is canonical or not.
    // ///
    // /// This is unsafe because the `canonical` flag will be used to determine
    // /// if the prefix needs escaping. If not set right, the syntax of the
    // /// produced serialization could be invalid.
    // pub unsafe fn new_unchecked(s: String, canonical: bool) -> Self {
    //     Self {
    //         canonical,
    //         value: s,
    //     }
    // }

    /// Get the prefix as a string slice.
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Extract the unescaped prefix as a `String`.
    pub fn into_string(self) -> String {
        self.value
    }
}

impl AsRef<str> for IdentPrefix {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl Display for IdentPrefix {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if self.is_canonical() {
            f.write_str(&self.value)
        } else {
            escape(f, &self.value)
        }
    }
}

impl From<IdentPrefix> for String {
    fn from(prefix: IdentPrefix) -> String {
        prefix.value
    }
}

impl From<&str> for IdentPrefix {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl<'i> FromPair<'i> for IdentPrefix {
    const RULE: Rule = Rule::IdPrefix;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, SyntaxError> {
        // Bail out if the local prefix is canonical (alphanumeric only)
        let inner = pair.into_inner().next().unwrap();
        if inner.as_rule() == Rule::CanonicalIdPrefix {
            return Ok(Self::new(inner.as_str().to_string()));
        }

        // Unescape the prefix if it was not produced by CanonicalIdPrefix.
        let s = inner.as_str();
        let escaped = s.quickcount(b'\\');
        let mut local = String::with_capacity(s.len() + escaped);
        unescape(&mut local, s).expect("fmt::Write cannot fail on a String");
        // FIXME(@althonos): possible syntax issue, which uses a non-canonical
        //                   rule on canonical prefixes (workaround is to check
        //                   one more time if the prefix is canonical)
        Ok(Self::new(local))
    }
}
impl_fromstr!(IdentPrefix);

impl Hash for IdentPrefix {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}

impl PartialEq for IdentPrefix {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

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

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    #[test]
    fn is_canonical() {
        assert!(IdentPrefix::from_str("GO").unwrap().is_canonical());
        assert!(IdentPrefix::new("GO").is_canonical());

        assert!(!IdentPrefix::from_str("n°t").unwrap().is_canonical());
        assert!(!IdentPrefix::new("n°t").is_canonical());
    }

    #[test]
    fn from_str() {
        let prefix = IdentPrefix::from_str("GO").unwrap();
        self::assert_eq!(prefix, IdentPrefix::new(String::from("GO")));
        assert!(prefix.is_canonical());
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
