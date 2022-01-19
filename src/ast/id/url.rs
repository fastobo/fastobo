use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use fastobo_derive_internal::FromStr;
use pest::iterators::Pair;

use crate::ast::IdentType;
use crate::error::SyntaxError;
use crate::parser::FromPair;
use crate::syntax::Rule;

/// A Uniform Resource Locator used as an identifier for an entity.
#[derive(Clone, Debug, FromStr, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Url(IdentType);

impl Url {
    /// Create a new `Url` from a string containing a URL representation.
    ///
    /// This method checks the URL is a syntactically correct IRI, but does
    /// not attempt any kind of canonicalization. This can affect comparison
    /// of semantically-equivalent URLs in the `PartialEq` implementation.
    ///
    /// # Example
    /// ```
    /// # extern crate fastobo;
    /// # use fastobo::ast::Url;
    /// assert!( Url::parse("http://example.com").is_ok() );
    /// assert!( Url::parse("not a URL").is_err() );
    /// ```
    pub fn parse(s: &str) -> Result<Self, SyntaxError> {
        std::str::FromStr::from_str(s)
    }

    /// View the URL as a string slice.
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl AsRef<str> for Url {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Display for Url {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.fmt(f)
    }
}

impl<'i> FromPair<'i> for Url {
    const RULE: Rule = Rule::Iri;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, SyntaxError> {
        Ok(Url(IdentType::from(pair.as_str())))
    }
}
