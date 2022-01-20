use std::collections::HashSet;
use std::str::FromStr;

use pest::iterators::Pair;

use crate::ast::IdentType;
use crate::error::SyntaxError;
use crate::syntax::Rule;

/// A string cache to recycle memory for shared values.
#[derive(Debug, Default)]
pub struct Cache {
    cache: HashSet<IdentType>,
}

/// A trait for structures that can be parsed from a [`pest::Pair`].
///
/// [`pest::Pair`]: https://docs.rs/pest/latest/pest/iterators/struct.Pair.html
pub trait FromPair<'i>: Sized {
    /// The production rule the pair is expected to be obtained from.
    const RULE: Rule;

    /// Create a new instance from a `Pair` without checking the rule.
    ///
    /// # Safety
    /// May panic if the pair was not produced by the right rule, i.e.
    /// `pair.as_rule() != <Self as FromPair>::RULE`.
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>, cache: &Cache)
        -> Result<Self, SyntaxError>;

    /// Create a new instance from a `Pair`.
    #[inline]
    fn from_pair(pair: Pair<'i, Rule>, cache: &Cache) -> Result<Self, SyntaxError> {
        if pair.as_rule() != Self::RULE {
            Err(SyntaxError::UnexpectedRule {
                actual: pair.as_rule(),
                expected: Self::RULE,
            })
        } else {
            unsafe { Self::from_pair_unchecked(pair, cache) }
        }
    }
}

impl<'i> FromPair<'i> for bool {
    const RULE: Rule = Rule::Boolean;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>, ctx: &Cache) -> Result<Self, SyntaxError> {
        Ok(bool::from_str(pair.as_str()).expect("cannot fail."))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::ast::*;
    use crate::syntax::Lexer;

    mod boolean {

        use super::*;

        #[test]
        fn from_pair() {
            let pairs = Lexer::tokenize(Rule::Boolean, "true");
            let pair = pairs.unwrap().next().unwrap();
            assert_eq!(bool::from_pair(pair).unwrap(), true);

            let pairs = Lexer::tokenize(Rule::Boolean, "false");
            let pair = pairs.unwrap().next().unwrap();
            assert_eq!(bool::from_pair(pair).unwrap(), false);
        }
    }

    #[test]
    fn unexpected_rule() {
        let pairs = Lexer::tokenize(Rule::Boolean, "true");
        let pair = pairs.unwrap().next().unwrap();

        let err = Ident::from_pair(pair).unwrap_err();
        match err {
            SyntaxError::UnexpectedRule {
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
