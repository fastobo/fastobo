use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

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

/// An identifier with a prefix.
#[derive(Clone, Debug, Eq, FromStr, Hash, Ord, PartialEq)]
pub struct PrefixedIdent {
    prefix: IdentType,
    local: IdentType,
}

impl PrefixedIdent {
    /// Create a new `PrefixedIdent` from a prefix and a local identifier.
    ///
    /// ```rust
    /// # extern crate fastobo;
    /// # use std::str::FromStr;
    /// # use fastobo::ast::*;
    /// let id1 = PrefixedIdent::new("MS", "1000031");
    /// let id2 = PrefixedIdent::from_str("MS:1000031").unwrap();
    /// assert_eq!(id1, id2);
    /// ```
    pub fn new<P, L>(prefix: P, local: L) -> Self
    where
        P: Into<IdentType>,
        L: Into<IdentType>,
    {
        Self {
            prefix: prefix.into(),
            local: local.into(),
        }
    }

    /// Check if the prefixed identifier is canonical or not.
    ///
    /// # Example
    /// ```rust
    /// # extern crate fastobo;
    /// # use fastobo::ast::*;
    /// # use std::str::FromStr;
    /// let canonical_id = PrefixedIdent::from_str("GO:0046154").unwrap();
    /// assert!(canonical_id.is_canonical());
    ///
    /// let noncanonical_id = PrefixedIdent::from_str("PATO:something").unwrap();
    /// assert!(!noncanonical_id.is_canonical());
    /// ```
    pub fn is_canonical(&self) -> bool {
        super::prefix::is_canonical(self.prefix())
            && self.local().chars().all(|c| c.is_ascii_digit())
    }

    /// Get the prefix part of the identifier.
    ///
    /// # Example
    /// ```rust
    /// # extern crate fastobo;
    /// # use fastobo::ast::*;
    /// let id = PrefixedIdent::new("MS", "1000031");
    /// assert_eq!(id.prefix(), "MS");
    /// ```
    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Get the local part of the identifier.
    ///
    /// # Example
    /// ```rust
    /// # extern crate fastobo;
    /// # use fastobo::ast::*;
    /// let id = PrefixedIdent::new("MS", "1000031");
    /// assert_eq!(id.local(), "1000031");
    /// ```
    pub fn local(&self) -> &str {
        &self.local
    }
}

impl Display for PrefixedIdent {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        escape(f, self.prefix())
            .and_then(|_| f.write_char(':'))
            .and_then(|_| escape(f, self.local()))
    }
}

impl<'i> FromPair<'i> for PrefixedIdent {
    const RULE: Rule = Rule::PrefixedId;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        let mut inners = pair.into_inner();
        let prefix = inners.next().unwrap();
        let local = inners.next().unwrap();

        // unescape prefix part if needed, otherwise don't allocate
        let p = if prefix.as_str().quickfind(b'\\').is_some() {
            let mut p = String::with_capacity(prefix.as_str().len());
            unescape(&mut p, prefix.as_str()).expect("cannot contain invalid escape characters");
            Cow::Owned(p)
        } else {
            Cow::Borrowed(prefix.as_str())
        };
        // unescape local part if needed, otherwise don't allocate
        let l = if local.as_str().quickfind(b'\\').is_some() {
            let mut l = String::with_capacity(local.as_str().len());
            unescape(&mut l, local.as_str()).expect("cannot contain invalid escape characters");
            Cow::Owned(l)
        } else {
            Cow::Borrowed(local.as_str())
        };

        // use a builder to allow recycling the string data, in particular
        // the prefix which should be shared by a lot of identifiers
        Ok(Self::new(cache.intern(&p), cache.intern(&l)))
    }
}

impl PartialOrd for PrefixedIdent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.prefix().partial_cmp(other.prefix()) {
            None => None,
            Some(Ordering::Equal) => self.local().partial_cmp(other.local()),
            Some(ord) => Some(ord),
        }
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
        let actual = PrefixedIdent::from_str("GO:0046154").unwrap();
        let expected = PrefixedIdent::new("GO", "0046154");
        assert_eq!(actual, expected);

        let actual = PrefixedIdent::from_str("PSI:MS").unwrap();
        let expected = PrefixedIdent::new("PSI", "MS");
        assert_eq!(actual, expected);

        let actual = PrefixedIdent::from_str("CAS:22325-47-9").unwrap();
        let expected = PrefixedIdent::new("CAS", "22325-47-9");
        assert_eq!(actual, expected);

        let actual = PrefixedIdent::from_str("web:https\\://example.com").unwrap();
        let expected = PrefixedIdent::new("web", "https://example.com");
        assert_eq!(actual, expected);

        assert!(PrefixedIdent::from_str("[Term]").is_err());
        assert!(PrefixedIdent::from_str("").is_err());
        assert!(PrefixedIdent::from_str("Some\nthing:spanning").is_err());
        assert!(PrefixedIdent::from_str("GO:0046154 remaining").is_err());
    }

    #[test]
    fn to_string() {
        let id = PrefixedIdent::new("GO", "0046154");
        assert_eq!(id.to_string(), "GO:0046154");

        let id_url = PrefixedIdent::new("web", "https://example.com");
        assert_eq!(id_url.to_string(), "web:https\\://example.com")
    }
}
