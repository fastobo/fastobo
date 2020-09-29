use std::cmp::Ordering;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use fastobo_derive_internal::FromStr;
use pest::iterators::Pair;

use crate::ast::StringType;
use crate::error::SyntaxError;
use crate::parser::FromPair;
use crate::syntax::Rule;

use super::escape;
use super::unescape;

/// An identifier with a prefix.
#[derive(Clone, Debug, Eq, FromStr, Hash, Ord, PartialEq)]
pub struct PrefixedIdent {
    data: StringType,
    local_offset: usize,
    // prefix: IdentPrefix,
    // local: IdentLocal,
}

impl PrefixedIdent {
    /// Create a new `PrefixedIdent` from a prefix and a local identifier.
    ///
    /// ```rust
    /// # extern crate fastobo;
    /// # use fastobo::ast::*;
    /// let id1 = PrefixedIdent::new("MS", "1000031");
    /// let id2 = PrefixedIdent::from_str("MS:1000031"));
    /// assert_eq!(id1, id2);
    /// ```
    pub fn new(prefix: &str, local: &str) -> Self {
        Self {
            data: StringType::from(format!("{}{}", prefix, local)),
            local_offset: prefix.len()
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
        todo!()
    }

    pub fn prefix(&self) -> &str {
        &self.data.as_str()[..self.local_offset]
    }

    pub fn local(&self) -> &str {
        &self.data.as_str()[self.local_offset..]
    }

    // /// Get a reference to the prefix of the `PrefixedIdent`.
    // pub fn prefix(&self) -> &IdentPrefix {
    //     &self.prefix
    // }
    //
    // /// Get a mutable reference to the prefix of the `PrefixedIdent`.
    // pub fn prefix_mut(&mut self) -> &mut IdentPrefix {
    //     &mut self.prefix
    // }
    //
    // /// Get a reference to the local component of the `PrefixedIdent`.
    // pub fn local(&self) -> &IdentLocal {
    //     &self.local
    // }
    //
    // /// Get a mutable reference to the local component of the `PrefixedIdent`.
    // pub fn local_mut(&mut self) -> &mut IdentLocal {
    //     &mut self.local
    // }
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
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, SyntaxError> {
        let mut inners = pair.into_inner();
        let prefix = inners.next().unwrap();
        let local = inners.next().unwrap();

        let mut data = StringType::new();
        unescape(&mut data, prefix.as_str())
            .expect("cannot contain invalid escape characters");

        let local_offset = data.len();
        unescape(&mut data, local.as_str())
            .expect("cannot contain invalid escape characters");

        Ok(Self { data, local_offset })
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
        let expected = PrefixedIdent::new("web","https://example.com");
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
