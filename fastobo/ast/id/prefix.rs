use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use opaque_typedef::OpaqueTypedefUnsized;
use pest::iterators::Pair;

use crate::borrow::Borrow;
use crate::borrow::Cow;
use crate::borrow::ToOwned;
use crate::error::Error;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;


/// An identifier prefix, either canonical or non-canonical.
///
/// * A canonical ID prefix only contains alphabetic characters (`[a-zA-Z]`)
///   followed by either an underscore or other alphabetic characters.
/// * A non-canonical ID prefix can contain any character besides `:`.
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct IdPrefix {
    value: String,
    canonical: bool,
}

impl IdPrefix {
    /// Create a new identifier prefix.
    pub fn new(s: String) -> Self {
        let mut chars = s.chars();

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
            value: s,
            canonical: canonical,
        }
    }

    /// Check if the identifier prefix is canonical or not.
    pub fn is_canonical(&self) -> bool {
        self.canonical
    }

    /// Create a new `IdPrefix` without checking if it is canonical.
    unsafe fn new_unchecked(s: String, canonical: bool) -> Self {
        Self { value: s, canonical }
    }
}

impl AsRef<str> for IdPrefix {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl<'a> Borrow<'a, IdPrf<'a>> for IdPrefix {
    fn borrow(&'a self) -> IdPrf<'a> {
        IdPrf::new(&self.value, self.canonical)
    }
}

impl Display for IdPrefix {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.borrow().fmt(f)
    }
}

impl<'i> FromPair<'i> for IdPrefix {
    const RULE: Rule = Rule::IdPrefix;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
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

/// A borrowed `IdPrefix`
#[derive(Clone, Debug)]
pub struct IdPrf<'a> {
    value: &'a str,
    canonical: bool,
}

impl<'a> IdPrf<'a> {
    // FIXME(@althonos): no canonical, add another `new_unchecked` method.

    /// Create a new `IdPrf` from a borrowed string slice.
    pub fn new(s: &'a str, canonical: bool) -> Self {
        IdPrf {
            value: s,
            canonical
        }
    }

    pub fn as_str(&self) -> &'a str {
        self.value
    }
}

impl<'a> Into<&'a str> for IdPrf<'a> {
    fn into(self) -> &'a str {
        self.value
    }
}

impl<'a> AsRef<str> for IdPrf<'a> {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl<'a> Display for IdPrf<'a> {
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

impl<'a> ToOwned<'a> for IdPrf<'a> {
    type Owned = IdPrefix;
    fn to_owned(&self) -> IdPrefix {
        unsafe {
            IdPrefix::new_unchecked(self.value.to_string(), self.canonical)
        }
    }
}
