use std::borrow::Cow;
use std::str::FromStr;

use pest::iterators::Pair;
use pest::Parser as PestParser;

use crate::error::Error;
use crate::error::Result;

/// The OBO format version 1.4 parser.
#[derive(Debug, Parser)]
#[grammar = "obo14/parser/grammar.pest"]
pub struct Parser;

/// A trait for structures that can be parsed from a [`pest::Pair`].
///
/// [`pest::Pair`]: https://docs.rs/pest/2.1.0/pest/iterators/struct.Pair.html
pub trait FromPair: Sized {
    const RULE: Rule;

    /// Create a new instance from a `Pair` without checking the rule.
    ///
    /// # Panic
    /// Panics if the pair was not produced by the right rule, i.e.
    /// `pair.as_rule() != <Self as FromPair>::RULE`.
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self>;

    #[inline]
    fn from_pair(pair: Pair<Rule>) -> Result<Self> {
        if pair.as_rule() != Self::RULE {
            return Err(Error::UnexpectedRule {
                actual: pair.as_rule(),
                expected: Self::RULE,
            });
        }

        unsafe { Self::from_pair_unchecked(pair) }
    }
}

/// A macro to provide `FromStr` implementation to `FromPair` implementors.
///
/// Ideally, a blanket implementation would be sufficient:
/// ```rust,nocompile
/// impl<T: FromPair> FromStr for T {
///     type Error = Error;
///     fn from_str(s: &str) -> Result<Self> {
///        ...
///     }
/// }
/// ```
/// but this is not permitted because of Rust trait implementation hygiene,
/// which prevents implementing a foreign trait on foreign type.
macro_rules! impl_fromstr {
    ($type:ty) => {
        impl std::str::FromStr for $type {
            type Err = $crate::error::Error;
            fn from_str(s: &str) -> Result<Self> {
                use $crate::pest::Parser;
                let mut pairs = $crate::obo14::parser::Parser::parse(Self::RULE, s)?;
                // FIXME: check for EOF ?
                unsafe { <Self as FromPair>::from_pair_unchecked(pairs.next().unwrap()) }
            }
        }
    };
}
