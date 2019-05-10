use std::str::FromStr;

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
