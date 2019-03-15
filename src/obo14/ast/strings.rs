use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

/// A string enclosed by quotes.
///
/// # Example
/// ```rust
/// # extern crate ontology;
/// # use ontology::obo14::QuotedString;
/// let s = QuotedString::new("Hello, world!");
/// assert_eq!(s.as_ref(), "Hello, world!");
/// assert_eq!(s.to_string(), "\"Hello, world!\"");
/// ```
pub struct QuotedString {
    value: String,
}

impl QuotedString {
    /// Create a new `QuotedString`.
    pub fn new<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        QuotedString { value: s.into() }
    }
}

impl AsRef<str> for QuotedString {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl Display for QuotedString {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_char('"')
            .and(self.value.chars().try_for_each(|char| match char {
                '\r' => f.write_str("\\r"),
                '\n' => f.write_str("\\n"),
                '\u{000c}' => f.write_str("\\f"),
                '"' => f.write_str("\\\""),
                _ => f.write_char(char),
            }))
            .and(f.write_char('"'))
    }
}

/// A string without delimiters, used as values in different clauses.
pub struct UnquotedString {
    value: String,
}

impl UnquotedString {
    /// Create a new `UnquotedString`.
    pub fn new<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        UnquotedString { value: s.into() }
    }
}

impl AsRef<str> for UnquotedString {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl Display for UnquotedString {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.value.chars().try_for_each(|char| match char {
            '\r' => f.write_str("\\r"),
            '\n' => f.write_str("\\n"),
            '\u{000c}' => f.write_str("\\f"),
            _ => f.write_char(char),
        })
    }
}
