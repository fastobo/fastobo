use std::borrow::Borrow;
use std::borrow::Cow;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::hash::Hash;

use fastobo_derive_internal::FromStr;
use pest::iterators::Pair;

use crate::ast::IdentType;
use crate::error::SyntaxError;
use crate::parser::Cache;
use crate::parser::FromPair;
use crate::parser::QuickFind;
use crate::syntax::Rule;

use super::escape;
use super::unescape;

/// Return whether a prefix is canonical.
#[allow(clippy::redundant_closure)]
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
pub struct IdentPrefix(IdentType);

impl IdentPrefix {
    /// Create a new identifier prefix.
    pub fn new<P>(prefix: P) -> Self
    where
        P: Into<IdentType>,
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
        self.0.as_ref().into()
    }

    /// Extract the unescaped prefix as the raw inner type.
    pub fn into_inner(self) -> IdentType {
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

impl From<IdentPrefix> for IdentType {
    fn from(prefix: IdentPrefix) -> Self {
        prefix.0
    }
}

impl From<&str> for IdentPrefix {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<IdentType> for IdentPrefix {
    fn from(i: IdentType) -> Self {
        Self::new(i)
    }
}

impl<'i> FromPair<'i> for IdentPrefix {
    const RULE: Rule = Rule::IdPrefix;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        // Bail out if the local prefix is canonical (alphanumeric only)
        let inner = pair.into_inner().next().unwrap();
        if inner.as_rule() == Rule::CanonicalIdPrefix {
            return Ok(Self::new(inner.as_str().to_string()));
        }

        // Unescape the prefix if it was not produced by CanonicalIdPrefix.
        let s = inner.as_str();
        let prefix = if s.quickfind(b'\\').is_some() {
            let mut prefix = String::with_capacity(s.len());
            unescape(&mut prefix, s).expect("fmt::Write cannot fail on a String");
            Cow::Owned(prefix)
        } else {
            Cow::Borrowed(s)
        };

        // FIXME(@althonos): possible syntax issue, which uses a non-canonical
        //                   rule on canonical prefixes (workaround is to check
        //                   one more time if the prefix is canonical)
        Ok(Self::new(cache.intern(&prefix)))
    }
}

impl PartialEq<str> for IdentPrefix {
    fn eq(&self, other: &str) -> bool {
        self.0.as_ref() == other
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
