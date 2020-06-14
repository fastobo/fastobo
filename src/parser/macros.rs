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
            type Err = $crate::error::SyntaxError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
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
