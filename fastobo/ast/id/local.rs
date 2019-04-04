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

/// A local identifier, preceded by a prefix in prefixed IDs.
///
/// * A canonical local ID only contains digits (`[0-9]`).
/// * A non-canonical local ID can contain any character excepting
///   whitespaces and newlines.
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct IdLocal {
    value: String,
    canonical: bool,
}

impl IdLocal {
    /// Create a new local identifier.
    pub fn new(s: String) -> Self {
        IdLocal {
            canonical: s.chars().all(|c| c.is_digit(10)),
            value: s,
        }
    }

    /// Check if the local identifier is canonical or not.
    pub fn is_canonical(&self) -> bool {
        self.canonical
    }

    /// Create a new `IdLocal` without checking if it is canonical.
    unsafe fn new_unchecked(s: String, canonical: bool) -> Self {
        Self { value: s, canonical }
    }
}

impl AsRef<str> for IdLocal {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl<'a> Borrow<'a, IdLcl<'a>> for IdLocal {
    fn borrow(&'a self) -> IdLcl<'a> {
        IdLcl::new(&self.value, self.canonical)
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

impl<'i> FromPair<'i> for IdLocal {
    const RULE: Rule = Rule::IdLocal;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
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

/// A borrowed `IdLocal`.
#[derive(Clone, Debug)]
pub struct IdLcl<'a> {
    value: &'a str,
    canonical: bool,
}

impl<'a> IdLcl<'a> {
    fn new(s: &'a str, canonical: bool) -> Self {
        IdLcl {
            value: s,
            canonical
        }
    }
}

impl<'a> AsRef<str> for IdLcl<'a> {
    fn as_ref(&self) -> &str {
        self.value
    }
}

impl<'a> ToOwned<'a> for IdLcl<'a> {
    type Owned = IdLocal;
    fn to_owned(&self) -> Self::Owned {
        unsafe {
            IdLocal::new_unchecked(self.value.to_owned(), self.canonical)
        }
    }
}
