use std::cmp::Ordering;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use fastobo_derive_internal::FromStr;
use pest::iterators::Pair;

use super::IdentLocal;
use super::IdentPrefix;
use crate::error::Error;
use crate::error::SyntaxError;
use crate::parser::FromPair;
use crate::syntax::Rule;

/// An identifier with a prefix.
#[derive(Clone, Debug, Eq, FromStr, Hash, Ord, PartialEq)]
pub struct PrefixedIdent {
    prefix: IdentPrefix,
    local: IdentLocal,
}

impl PrefixedIdent {
    /// Create a new `PrefixedIdent` from a prefix and a local identifier.
    ///
    /// Thanks to conversion traits, the `prefix` and `local` arguments can be
    /// passed either as strings or `ast` structures:
    ///
    /// ```rust
    /// # extern crate fastobo;
    /// # use fastobo::ast::*;
    /// let id1 = PrefixedIdent::new("MS", "1000031");
    /// let id2 = PrefixedIdent::new(IdentPrefix::new("MS"), IdentLocal::new("1000031"));
    /// assert_eq!(id1, id2);
    /// ```
    ///
    /// # Example
    ///
    pub fn new<P, L>(prefix: P, local: L) -> Self
    where
        P: Into<IdentPrefix>,
        L: Into<IdentLocal>,
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
    pub fn is_canonical(&self) -> bool {
        self.prefix.is_canonical() && self.local.is_canonical()
    }

    // /// The prefix of the prefixed identifier.
    // pub fn prefix(&self) -> IdPrefix<'_> {
    //     self.prefix.share()
    // }
    //
    // /// The local part of the prefixed identifier.
    // pub fn local(&self) -> IdLocal<'_> {
    //     self.local.share()
    // }

    /// Get a reference to the prefix of the `PrefixedIdent`.
    pub fn prefix(&self) -> &IdentPrefix {
        &self.prefix
    }

    /// Get a mutable reference to the prefix of the `PrefixedIdent`.
    pub fn prefix_mut(&mut self) -> &mut IdentPrefix {
        &mut self.prefix
    }

    /// Get a reference to the local component of the `PrefixedIdent`.
    pub fn local(&self) -> &IdentLocal {
        &self.local
    }

    /// Get a mutable reference to the local component of the `PrefixedIdent`.
    pub fn local_mut(&mut self) -> &mut IdentLocal {
        &mut self.local
    }
}

impl Display for PrefixedIdent {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.prefix
            .fmt(f)
            .and(f.write_char(':'))
            .and(self.local.fmt(f))
    }
}

impl<'i> FromPair<'i> for PrefixedIdent {
    const RULE: Rule = Rule::PrefixedId;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, SyntaxError> {
        let mut inners = pair.into_inner();
        let prefix = IdentPrefix::from_pair_unchecked(inners.next().unwrap())?;
        let local = IdentLocal::from_pair_unchecked(inners.next().unwrap())?;
        Ok(Self::new(prefix, local))
    }
}

impl PartialOrd for PrefixedIdent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.prefix.partial_cmp(&other.prefix) {
            None => None,
            Some(Ordering::Equal) => self.local.partial_cmp(&other.local),
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
        let expected = PrefixedIdent::new(
            IdentPrefix::new(String::from("GO")),
            IdentLocal::new(String::from("0046154")),
        );
        self::assert_eq!(actual, expected);

        let actual = PrefixedIdent::from_str("PSI:MS").unwrap();
        let expected = PrefixedIdent::new(
            IdentPrefix::new(String::from("PSI")),
            IdentLocal::new(String::from("MS")),
        );
        self::assert_eq!(actual, expected);

        let actual = PrefixedIdent::from_str("CAS:22325-47-9").unwrap();
        let expected = PrefixedIdent::new(
            IdentPrefix::new(String::from("CAS")),
            IdentLocal::new(String::from("22325-47-9")),
        );
        self::assert_eq!(actual, expected);

        let actual =
            PrefixedIdent::from_str("Wikipedia:https\\://en.wikipedia.org/wiki/Gas").unwrap();
        let expected = PrefixedIdent::new(
            IdentPrefix::new(String::from("Wikipedia")),
            IdentLocal::new(String::from("https://en.wikipedia.org/wiki/Gas")),
        );
        self::assert_eq!(actual, expected);

        assert!(PrefixedIdent::from_str("[Term]").is_err());
        assert!(PrefixedIdent::from_str("").is_err());
        assert!(PrefixedIdent::from_str("Some\nthing:spanning").is_err());
        assert!(PrefixedIdent::from_str("GO:0046154 remaining").is_err());
    }

    #[test]
    fn to_string() {
        let id = PrefixedIdent::new(
            IdentPrefix::new(String::from("GO")),
            IdentLocal::new(String::from("0046154")),
        );
        self::assert_eq!(id.to_string(), "GO:0046154")
    }
}
