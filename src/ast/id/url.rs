use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;

use fastobo_derive_internal::FromStr;

use crate::ast::IdentType;
use crate::error::SyntaxError;
use crate::parser::Cache;
use crate::parser::FromPair;
use crate::parser::QuickFind;
use crate::syntax::pest::error::Error as PestError;
use crate::syntax::pest::error::ErrorVariant;
use crate::syntax::pest::iterators::Pair;
use crate::syntax::pest::Position;
use crate::syntax::Lexer;
use crate::syntax::Rule;

use super::unescape;

/// A Uniform Resource Locator used as an identifier for an entity.
#[derive(Clone, Debug, FromStr, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Url(IdentType);

impl Url {
    /// Create a new `Url` from an `IdentType`.
    ///
    /// This method checks the URL is a syntactically correct IRI, but does
    /// not attempt any kind of canonicalization. This can affect comparison
    /// of semantically-equivalent URLs in the `PartialEq` implementation.
    ///
    /// # Example
    /// ```
    /// # extern crate fastobo;
    /// # use std::str::FromStr;
    /// # use fastobo::ast::Url;
    /// assert!( Url::from_str("http://example.com/resource").is_ok() );
    /// assert!( Url::from_str("not a URL").is_err() );
    /// ```
    pub fn new<S>(url: S) -> Result<Self, SyntaxError>
    where
        S: Into<IdentType>,
    {
        let u = url.into();

        let pair = Lexer::tokenize(Self::RULE, &u)?.next().unwrap();
        if pair.as_span().end() != u.as_ref().len() {
            let span = pair
                .as_span()
                .end_pos()
                .span(&Position::new(&u, u.len()).unwrap());
            let variant = ErrorVariant::CustomError {
                message: "remaining input".to_string(),
            };
            Err(PestError::new_from_span(variant, span).into())
        } else {
            Ok(Url(u))
        }
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
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        let s = pair.as_str();
        let escaped = s.quickcount(b'\\');
        if escaped > 0 {
            let mut local = String::with_capacity(s.len() + escaped);
            unescape(&mut local, s).expect("fmt::Write cannot fail on a String");
            Ok(Url(cache.intern(&local)))
        } else {
            Ok(Url(cache.intern(s)))
        }
    }
}
