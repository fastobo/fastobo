use std::borrow::Borrow;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::hash::Hash;

use fastobo_derive_internal::FromStr;
use pest::iterators::Pair;

use crate::ast::StringType;
use crate::error::SyntaxError;
use crate::parser::FromPair;
use crate::parser::QuickFind;
use crate::syntax::Rule;

use super::escape;
use super::unescape;

/// Return whether a prefix is canonical.
pub fn is_canonical<S: AsRef<str>>(s: S) -> bool {
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
#[derive(Clone, Debug, Eq, Hash, FromStr, Ord, PartialEq, PartialOrd)]
pub struct IdentPrefix(StringType);

impl IdentPrefix {
    /// Create a new identifier prefix.
    pub fn new<S>(prefix: S) -> Self
    where
        S: Into<StringType>,
    {
        Self(prefix.into())
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
        is_canonical(&self.0)
    }

    /// Get the prefix as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Extract the unescaped prefix as a `String`.
    pub fn into_string(self) -> String {
        self.0.into()
    }

    /// Extract the unescaped prefix as the raw inner type.
    pub fn into_inner(self) -> StringType {
        self.0
    }
}

impl AsRef<str> for IdentPrefix {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for IdentPrefix {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl Display for IdentPrefix {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if self.is_canonical() {
            f.write_str(&self.0)
        } else {
            escape(f, &self.0)
        }
    }
}

#[cfg(feature = "smartstring")]
impl From<IdentPrefix> for String {
    fn from(prefix: IdentPrefix) -> String {
        prefix.0.into()
    }
}

impl From<IdentPrefix> for StringType {
    fn from(prefix: IdentPrefix) -> Self {
        prefix.0
    }
}

impl From<&str> for IdentPrefix {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<StringType> for IdentPrefix {
    fn from(s: StringType) -> Self {
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

impl PartialEq<str> for IdentPrefix {
    fn eq(&self, other: &str) -> bool {
        self.0.as_str() == other
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
