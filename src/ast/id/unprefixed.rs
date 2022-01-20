use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

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

/// An identifier without a prefix.
#[derive(Clone, Debug, Hash, Eq, FromStr, Ord, PartialEq, PartialOrd)]
pub struct UnprefixedIdent(IdentType);

impl UnprefixedIdent {
    /// Create a new unprefixed identifier.
    pub fn new<S>(id: S) -> Self
    where
        S: Into<IdentType>,
    {
        // FIXME: check the given string is a valid unprefixed identifier?.
        Self(id.into())
    }

    /// Return a reference to the underlying `str`.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for UnprefixedIdent {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for UnprefixedIdent {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        escape(f, &self.0)
    }
}

impl From<UnprefixedIdent> for IdentType {
    fn from(id: UnprefixedIdent) -> Self {
        id.0
    }
}

impl From<&str> for UnprefixedIdent {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<IdentType> for UnprefixedIdent {
    fn from(s: IdentType) -> Self {
        Self::new(s)
    }
}

impl<'i> FromPair<'i> for UnprefixedIdent {
    const RULE: Rule = Rule::UnprefixedId;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        let s = pair.as_str();
        let escaped = s.quickcount(b'\\');
        let mut local = String::with_capacity(s.len() + escaped);
        unescape(&mut local, s).expect("fmt::Write cannot fail on a String");
        Ok(Self::new(cache.intern(&local)))
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
        let actual = UnprefixedIdent::from_str("biological_process").unwrap();
        let expected = UnprefixedIdent::new(String::from("biological_process"));
        self::assert_eq!(actual, expected);

        assert!(UnprefixedIdent::from_str("").is_err());
        assert!(UnprefixedIdent::from_str("Some\nthing:spanning").is_err());
        assert!(UnprefixedIdent::from_str("goslim_plant remaining").is_err());
    }

    #[test]
    fn to_string() {
        let id = UnprefixedIdent::new(String::from("something:with:colons"));
        self::assert_eq!(id.to_string(), "something\\:with\\:colons");
    }
}
