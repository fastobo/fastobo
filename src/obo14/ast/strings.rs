use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use pest::iterators::Pair;

use super::super::parser::FromPair;
use super::super::parser::Parser;
use super::super::parser::Rule;
use crate::error::Error;
use crate::error::Result;

/// A string enclosed by quotes, used for definitions.
///
/// # Example
/// ```rust
/// # extern crate fastobo;
/// # use fastobo::obo14::QuotedString;
/// let s = QuotedString::new("Hello, world!");
/// assert_eq!(s.as_ref(), "Hello, world!");
/// assert_eq!(s.to_string(), "\"Hello, world!\"");
/// ```
#[derive(Debug, Eq, Hash, PartialEq)]
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
                '\\' => f.write_str("\\"),
                '"' => f.write_str("\\\""),
                _ => f.write_char(char),
            }))
            .and(f.write_char('"'))
    }
}

impl FromPair for QuotedString {
    const RULE: Rule = Rule::QuotedString;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        let s = pair.as_str();
        let mut local = String::with_capacity(s.len());
        let mut chars = s.get_unchecked(1..s.len() - 1).chars();
        while let Some(char) = chars.next() {
            if char == '\\' {
                match chars.next() {
                    Some('r') => local.push('\r'),
                    Some('n') => local.push('\n'),
                    Some('f') => local.push('\u{000c}'),
                    Some('t') => local.push('\t'),
                    Some(other) => local.push(other),
                    None => panic!("missing stuff"), // FIXME(@althonos)
                }
            } else {
                local.push(char);
            }
        }

        Ok(QuotedString::new(local))
    }
}
impl_fromstr!(QuotedString);

/// A string without delimiters, used as values in different clauses.
#[derive(Debug, Eq, Hash, PartialEq)]
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

            // QUESTION(@althonos): Not required in the spec, but most do it.
            '"' => f.write_str("\\\""),
            '\\' => f.write_str("\\\\"),

            _ => f.write_char(char),
        })
    }
}

impl FromPair for UnquotedString {
    const RULE: Rule = Rule::UnquotedString;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        let s = pair.as_str();
        let mut local = String::with_capacity(s.len());
        let mut chars = s.chars();
        while let Some(char) = chars.next() {
            if char == '\\' {
                match chars.next() {
                    Some('r') => local.push('\r'),
                    Some('n') => local.push('\n'),
                    Some('f') => local.push('\u{000c}'),
                    Some('t') => local.push('\t'),
                    Some(other) => local.push(other),
                    None => panic!("missing stuff"), // FIXME(@althonos)
                }
            } else {
                local.push(char);
            }
        }

        Ok(UnquotedString::new(local))
    }
}
impl_fromstr!(UnquotedString);

#[cfg(test)]
mod tests {

    use super::*;
    use std::str::FromStr;
    use std::string::ToString;

    mod quoted {

        use super::*;

        #[test]
        fn from_str() {
            let actual = QuotedString::from_str("\"something in quotes\"");
            let expected = QuotedString::new("something in quotes");
            assert_eq!(expected, actual.unwrap());

            let actual = QuotedString::from_str("\"something in \\\"escaped\\\" quotes\"");
            let expected = QuotedString::new("something in \"escaped\" quotes");
            assert_eq!(expected, actual.unwrap());
        }
    }

    mod unquoted {

        use super::*;

        #[test]
        fn from_str() {
            let actual = UnquotedString::from_str("something\\ttabbed");
            let expected = UnquotedString::new("something\ttabbed");
            assert_eq!(expected, actual.unwrap());

            let actual = UnquotedString::from_str("namespace-id-rule").unwrap();
            let expected = UnquotedString::new("namespace-id-rule");
            assert_eq!(expected, actual);
        }
    }

}
