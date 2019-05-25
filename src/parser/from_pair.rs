use pest::error::Error as PestError;
use pest::error::ErrorVariant;
use pest::iterators::Pair;
use url::Url;

use std::str::FromStr;

use crate::error::Error;
use crate::parser::Rule;

/// A trait for structures that can be parsed from a [`pest::Pair`].
///
/// [`pest::Pair`]: https://docs.rs/pest/latest/pest/iterators/struct.Pair.html
pub trait FromPair<'i>: Sized {
    const RULE: Rule;

    /// Create a new instance from a `Pair` without checking the rule.
    ///
    /// # Panic
    /// Panics if the pair was not produced by the right rule, i.e.
    /// `pair.as_rule() != <Self as FromPair>::RULE`.
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, Error>;

    /// Create a new instance from a `Pair`.
    #[inline]
    fn from_pair(pair: Pair<'i, Rule>) -> Result<Self, Error> {
        if pair.as_rule() != Self::RULE {
            return Err(Error::UnexpectedRule {
                actual: pair.as_rule(),
                expected: Self::RULE,
            });
        }

        unsafe { Self::from_pair_unchecked(pair) }
    }
}

impl<'i> FromPair<'i> for bool {
    const RULE: Rule = Rule::Boolean;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, Error> {
        Ok(bool::from_str(pair.as_str()).expect("cannot fail."))
    }
}

impl<'i> FromPair<'i> for Url {
    const RULE: Rule = Rule::Iri;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, Error> {
        Url::parse(pair.as_str()).map_err(|e| {
            Error::from(PestError::new_from_span(
                ErrorVariant::CustomError {
                    message: format!("could not parse URL: {}", e),
                },
                pair.as_span(),
            ))
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::ast::*;
    use crate::parser::OboParser;

    mod url {

        use super::*;

        #[test]
        fn from_pair() {
            let pairs = OboParser::parse(Rule::UnquotedString, "http://not an url");
            let pair = pairs.unwrap().next().unwrap();
            unsafe { assert!(Url::from_pair_unchecked(pair).is_err()) }
        }
    }

    mod boolean {

        use super::*;

        #[test]
        fn from_pair() {
            let pairs = OboParser::parse(Rule::Boolean, "true");
            let pair = pairs.unwrap().next().unwrap();
            assert_eq!(bool::from_pair(pair).unwrap(), true);

            let pairs = OboParser::parse(Rule::Boolean, "false");
            let pair = pairs.unwrap().next().unwrap();
            assert_eq!(bool::from_pair(pair).unwrap(), false);
        }
    }

    #[test]
    fn unexpected_rule() {
        let pairs = OboParser::parse(Rule::Boolean, "true");
        let pair = pairs.unwrap().next().unwrap();

        let err = Ident::from_pair(pair).unwrap_err();
        match err {
            Error::UnexpectedRule {
                ref actual,
                ref expected,
            } => {
                assert_eq!(actual, &Rule::Boolean);
                assert_eq!(expected, &Rule::Id);
            }
            e => panic!("unexpected error: {:?}", e),
        }
    }

}
