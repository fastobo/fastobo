//! Parser and parsing-related traits for the OBO format.

use std::str::FromStr;

use pest::iterators::Pair;
use pest::Parser;

use crate::borrow::Borrow;
use crate::borrow::ToOwned;
use crate::error::Error;

mod quickfind;

#[doc(inline)]
pub use fastobo_syntax::OboParser;
#[doc(inline)]
pub use fastobo_syntax::Rule;

pub use self::quickfind::QuickFind;

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

/// Parse a value from a slice with a lifetime parameter.
///
/// This trait is an extension of the `FromStr` trait from the standard library,
/// and allows keeping a reference to the slice passed as argument.
pub trait FromSlice<'i>: Sized {
    /// The associated error which can be returned from parsing.
    type Err;
    /// Parses a string slice `s` to return a value of this type.
    fn from_slice(s: &'i str) -> Result<Self, Self::Err>;
}

impl<'i, T> FromSlice<'i> for T
where
    T: FromStr,
{
    type Err = <Self as FromStr>::Err;
    fn from_slice(s: &'i str) -> Result<Self, Self::Err> {
        <Self as FromStr>::from_str(s)
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

// FIXME: Proper type !
macro_rules! impl_fromslice {
    ($life:lifetime, $type:ty) => {
        impl<$life> $crate::parser::FromSlice<$life> for $type {
            type Err = $crate::error::Error;
            fn from_slice(s: &$life str) -> $crate::error::Result<Self> {
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
