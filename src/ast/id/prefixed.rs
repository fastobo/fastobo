use std::convert::TryFrom;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use iri_string::Url;
use pest::iterators::Pair;

use crate::error::Error;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Parser;
use crate::parser::Rule;

/// An identifier with a prefix.
#[derive(Debug, PartialEq, Hash, Eq)]
pub struct PrefixedId {
    prefix: IdPrefix,
    local: IdLocal,
}

impl PrefixedId {
    /// Create a new `PrefixedId` from a prefix and a local identifier.
    pub fn new(prefix: IdPrefix, local: IdLocal) -> Self {
        Self { prefix, local }
    }

    /// Check if the prefixed identifier is canonical or not.
    ///
    /// # Example
    /// ```rust
    /// # extern crate fastobo;
    /// # use fastobo::obo14::*;
    /// # use std::str::FromStr;
    /// let canonical_id = PrefixedId::from_str("GO:0046154").unwrap();
    /// assert!(canonical_id.is_canonical());
    ///
    /// let noncanonical_id = PrefixedId::from_str("PATO:something").unwrap();
    /// assert!(!noncanonical_id.is_canonical());
    pub fn is_canonical(&self) -> bool {
        self.prefix.is_canonical() && self.local.is_canonical()
    }
}

impl Display for PrefixedId {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.prefix
            .fmt(f)
            .and(f.write_char(':'))
            .and(self.local.fmt(f))
    }
}

impl FromPair for PrefixedId {
    const RULE: Rule = Rule::PrefixedId;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        let mut inners = pair.into_inner();
        let prefix = IdPrefix::from_pair_unchecked(inners.next().unwrap())?;
        let local = IdLocal::from_pair_unchecked(inners.next().unwrap())?;
        Ok(PrefixedId::new(prefix, local))
    }
}
impl_fromstr!(PrefixedId);

/// An identifier prefix, either canonical or non-canonical.
///
/// * A canonical ID prefix only contains alphabetic characters (`[a-zA-Z]`)
///   followed by either an underscore or other alphabetic characters.
/// * A non-canonical ID prefix can contain any character besides `:`.
#[derive(Debug, PartialEq, Hash, Eq)]
pub struct IdPrefix {
    value: String,
    canonical: bool,
}

impl IdPrefix {
    /// Create a new identifier prefix.
    pub fn new<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        let string = s.into();
        let mut chars = string.chars();

        let canonical = if let Some(c) = chars.next() {
            match c {
                'A'...'Z' | 'a'...'z' => chars.all(|c| match c {
                    'A'...'Z' | 'a'...'z' | '_' => true,
                    _ => false,
                }),
                _ => false,
            }
        } else {
            false
        };

        IdPrefix {
            value: string,
            canonical: canonical,
        }
    }

    /// Check if the identifier prefix is canonical or not.
    pub fn is_canonical(&self) -> bool {
        self.canonical
    }
}

impl AsRef<str> for IdPrefix {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl Display for IdPrefix {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if self.canonical {
            f.write_str(&self.value)
        } else {
            self.value.chars().try_for_each(|char| match char {
                '\r' => f.write_str("\\r"),
                '\n' => f.write_str("\\n"),
                '\u{000c}' => f.write_str("\\f"),
                ' ' => f.write_str("\\ "),
                '\t' => f.write_str("\\t"),
                ':' => f.write_str("\\:"), // FIXME(@althonos) ?
                _ => f.write_char(char),
            })
        }
    }
}

impl FromPair for IdPrefix {
    const RULE: Rule = Rule::IdPrefix;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        // Bail out if the local prefix is canonical (alphanumeric only)
        let inner = pair.into_inner().next().unwrap();
        if inner.as_rule() == Rule::CanonicalIdPrefix {
            return Ok(IdPrefix {
                value: inner.as_str().to_string(),
                canonical: true,
            });
        }

        // Unescape the prefix if is non canonical.
        let mut local = String::with_capacity(inner.as_str().len());
        let mut chars = inner.as_str().chars();
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

        Ok(IdPrefix {
            value: inner.as_str().to_string(),
            canonical: false,
        })
    }
}
impl_fromstr!(IdPrefix);

/// A local identifier, preceded by a prefix in prefixed IDs.
///
/// * A canonical local ID only contains digits (`[0-9]`).
/// * A non-canonical local ID can contain any character excepting
///   whitespaces and newlines.
#[derive(Debug, PartialEq, Hash, Eq)]
pub struct IdLocal {
    value: String,
    canonical: bool,
}

impl IdLocal {
    /// Create a new local identifier.
    pub fn new<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        let string = s.into();
        let canonical = string.chars().all(|c| c.is_digit(10));

        IdLocal {
            value: string,
            canonical: canonical,
        }
    }

    /// Check if the local identifier is canonical or not.
    pub fn is_canonical(&self) -> bool {
        self.canonical
    }
}

impl Display for IdLocal {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if self.canonical {
            f.write_str(&self.value)
        } else {
            self.value.chars().try_for_each(|char| match char {
                '\r' => f.write_str("\\r"),
                '\n' => f.write_str("\\n"),
                '\u{000c}' => f.write_str("\\f"),
                ' ' => f.write_str("\\ "),
                '\t' => f.write_str("\\t"),
                _ => f.write_char(char),
            })
        }
    }
}

impl FromPair for IdLocal {
    const RULE: Rule = Rule::IdLocal;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        // Bail out if the local ID is canonical (digits only).
        let inner = pair.into_inner().next().unwrap();
        if inner.as_rule() == Rule::CanonicalIdLocal {
            return Ok(IdLocal {
                value: inner.as_str().to_string(),
                canonical: true,
            });
        }

        // Unescape the local ID if it is non canonical.
        let mut local = String::with_capacity(inner.as_str().len());
        let mut chars = inner.as_str().chars();
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

        Ok(IdLocal {
            value: local,
            canonical: false,
        })
    }
}
impl_fromstr!(IdLocal);

#[cfg(test)]
mod tests {

    use super::*;
    use std::str::FromStr;
    use std::string::ToString;

    #[test]
    fn from_str() {
        let actual = PrefixedId::from_str("GO:0046154").unwrap();
        let expected = PrefixedId::new(IdPrefix::new("GO"), IdLocal::new("0046154"));
        assert_eq!(actual, expected);

        let actual = PrefixedId::from_str("PSI:MS").unwrap();
        let expected = PrefixedId::new(IdPrefix::new("PSI"), IdLocal::new("MS"));
        assert_eq!(actual, expected);

        let actual = PrefixedId::from_str("CAS:22325-47-9").unwrap();
        let expected = PrefixedId::new(IdPrefix::new("CAS"), IdLocal::new("22325-47-9"));
        assert_eq!(actual, expected);

        let actual = PrefixedId::from_str("Wikipedia:https\\://en.wikipedia.org/wiki/Gas").unwrap();
        let expected = PrefixedId::new(
            IdPrefix::new("Wikipedia"),
            IdLocal::new("https://en.wikipedia.org/wiki/Gas"),
        );
        assert_eq!(actual, expected);

        assert!(PrefixedId::from_str("[Term]").is_err());
        assert!(PrefixedId::from_str("").is_err());
        assert!(PrefixedId::from_str("Some\nthing:spanning").is_err());
        assert!(PrefixedId::from_str("GO:0046154 remaining").is_err());
    }

    #[test]
    fn to_string() {
        let id = PrefixedId::new(IdPrefix::new("GO"), IdLocal::new("0046154"));
        assert_eq!(id.to_string(), "GO:0046154")
    }
}
