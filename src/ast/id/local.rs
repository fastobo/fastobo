use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::hash::Hash;

use fastobo_derive_internal::FromStr;
use pest::iterators::Pair;

use crate::ast::StringType;
use crate::error::SyntaxError;
use crate::parser::Cache;
use crate::parser::FromPair;
use crate::parser::QuickFind;
use crate::syntax::Rule;

use super::escape;
use super::unescape;

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
#[derive(Clone, Debug, Eq, Hash, FromStr, Ord, PartialEq, PartialOrd)]
pub struct IdentLocal(StringType);

impl IdentLocal {
    /// Create a new local identifier.
    pub fn new<S>(local: S) -> Self
    where
        S: Into<StringType>,
    {
        Self(local.into())
    }

    /// Check if the local identifier is canonical or not.
    pub fn is_canonical(&self) -> bool {
        is_canonical(&self.0)
    }

    /// Get the local identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Extract the unescaped local identifier as a `String`.
    pub fn into_string(self) -> String {
        self.0.into()
    }

    /// Extract the unescaped local identifier as the raw inner type.
    pub fn into_inner(self) -> StringType {
        self.0
    }
}

impl AsRef<str> for IdentLocal {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(feature = "smartstring")]
impl From<IdentLocal> for String {
    fn from(id: IdentLocal) -> Self {
        id.0.into()
    }
}

impl From<&str> for IdentLocal {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<StringType> for IdentLocal {
    fn from(s: StringType) -> Self {
        Self::new(s)
    }
}

impl From<IdentLocal> for StringType {
    fn from(id: IdentLocal) -> Self {
        id.0
    }
}

impl Display for IdentLocal {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if self.is_canonical() {
            f.write_str(&self.0)
        } else {
            escape(f, &self.0)
        }
    }
}

impl<'i> FromPair<'i> for IdentLocal {
    const RULE: Rule = Rule::IdLocal;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>, cache: &Cache) -> Result<Self, SyntaxError> {
        // Bail out if the local ID is canonical (digits only).
        let inner = pair.into_inner().next().unwrap();
        if inner.as_rule() == Rule::CanonicalIdLocal {
            return Ok(Self::new(inner.as_str().to_string()));
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

impl PartialEq<str> for IdentLocal {
    fn eq(&self, other: &str) -> bool {
        self.0.as_str() == other
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
