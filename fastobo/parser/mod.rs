//! Parser and parsing-related traits for the OBO format.

use std::borrow::Cow;
use std::str::FromStr;

use pest::iterators::Pair;
use pest::Parser;

use crate::error::Error;
use crate::error::Result;

#[doc(inline)]
pub use fastobo_syntax::OboParser;
#[doc(inline)]
pub use fastobo_syntax::Rule;

/// A trait for structures that can be parsed from a [`pest::Pair`].
///
/// [`pest::Pair`]: https://docs.rs/pest/2.1.0/pest/iterators/struct.Pair.html
pub trait FromPair<'i>: Sized {
    const RULE: Rule;

    /// Create a new instance from a `Pair` without checking the rule.
    ///
    /// # Panic
    /// Panics if the pair was not produced by the right rule, i.e.
    /// `pair.as_rule() != <Self as FromPair>::RULE`.
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self>;

    /// Create a new instance from a `Pair`.
    #[inline]
    fn from_pair(pair: Pair<'i, Rule>) -> Result<Self> {
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
            fn from_str(s: &str) -> $crate::error::Result<Self> {
                use $crate::error::Error;
                use $crate::parser::OboParser;
                use $crate::pest::error::ErrorVariant;
                use $crate::pest::Parser;
                use $crate::pest::Position;

                // Parse the input string
                let mut pairs = OboParser::parse(Self::RULE, s)?;
                let pair = pairs.next().unwrap();
                // Check EOI was reached
                if pair.as_span().end() != s.len() {
                    let span = pair
                        .as_span()
                        .end_pos()
                        .span(&Position::new(s, s.len()).unwrap());
                    let variant = ErrorVariant::CustomError {
                        message: "remaining input".to_string(),
                    };
                    Err($crate::pest::error::Error::new_from_span(variant, span).into())
                } else {
                    unsafe { <Self as FromPair>::from_pair_unchecked(pair) }
                }
            }
        }
    };
}
