use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use opaque_typedef::OpaqueTypedefUnsized;
use pest::iterators::Pair;

use crate::share::Share;
use crate::share::Cow;
use crate::share::Redeem;
use crate::error::Error;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;
use crate::parser::QuickFind;

fn escape<W: Write>(f: &mut W, s: &str) -> FmtResult {
    s.chars().try_for_each(|char| match char {
        '\r' => f.write_str("\\r"),
        '\n' => f.write_str("\\n"),
        '\u{000c}' => f.write_str("\\f"),
        ' ' => f.write_str("\\ "),
        '\t' => f.write_str("\\t"),
        ':' => f.write_str("\\:"),
        '"' => f.write_str("\\\""),
        '\\' => f.write_str("\\\\"),
        _ => f.write_char(char)
    })
}

fn unescape<W: Write>(f: &mut W, s: &str) -> FmtResult {
    let mut chars = s.chars();
    while let Some(char) = chars.next() {
        if char == '\\' {
            match chars.next() {
                Some('r') => f.write_char('\r')?,
                Some('n') => f.write_char('\n')?,
                Some('f') => f.write_char('\u{000c}')?,
                Some('t') => f.write_char('\t')?,
                Some(other) => f.write_char(other)?,
                None => panic!("invalid escape"), // FIXME(@althonos)
            }
        } else {
            f.write_char(char)?;
        }
    }
    Ok(())
}

fn is_canonical<S: AsRef<str>>(s: S) -> bool {
    s.as_ref().chars().all(|c| c.is_ascii_digit())
}


/// A local identifier, preceded by a prefix in prefixed IDs.
///
/// * A canonical local ID only contains digits (`[0-9]`).
/// * A non-canonical local ID can contain any character excepting
///   whitespaces and newlines.
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct IdentLocal {
    value: String,
    canonical: bool,
}

impl IdentLocal {
    /// Create a new local identifier.
    pub fn new(s: String) -> Self {
        Self {
            canonical: is_canonical(&s),
            value: s,
        }
    }

    /// Check if the local identifier is canonical or not.
    pub fn is_canonical(&self) -> bool {
        self.canonical
    }

    /// Create a new `IdLocal` without checking if it is canonical.
    pub unsafe fn new_unchecked(s: String, canonical: bool) -> Self {
        Self { value: s, canonical }
    }

    /// Get the local identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl AsRef<str> for IdentLocal {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl<'a> Share<'a, IdLocal<'a>> for IdentLocal {
    fn share(&'a self) -> IdLocal<'a> {
        unsafe { IdLocal::new_unchecked(&self.value, self.canonical) }
    }
}

impl Display for IdentLocal {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.share().fmt(f)
    }
}

impl<'i> FromPair<'i> for IdentLocal {
    const RULE: Rule = Rule::IdLocal;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        // Bail out if the local ID is canonical (digits only).
        let inner = pair.into_inner().next().unwrap();
        if inner.as_rule() == Rule::CanonicalIdLocal {
            return Ok(Self::new_unchecked(inner.as_str().to_string(), true));
        }

        // Unescape the local ID if it is non canonical.
        let s = inner.as_str();
        let escaped = s.quickcount(b'\\');
        let mut local = String::with_capacity(s.len() + escaped);
        unescape(&mut local, s).expect("fmt::Write cannot fail on a String");

        Ok(Self::new_unchecked(local, false))
    }
}
impl_fromstr!(IdentLocal);

/// A borrowed `IdLocal`.
#[derive(Clone, Debug)]
pub struct IdLocal<'a> {
    value: &'a str,
    canonical: bool,
}

impl<'a> IdLocal<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            canonical: is_canonical(s),
            value: s,
        }
    }

    pub unsafe fn new_unchecked(s: &'a str, canonical: bool) -> Self {
        Self {
            value: s,
            canonical,
        }
    }
}

impl<'a> AsRef<str> for IdLocal<'a> {
    fn as_ref(&self) -> &str {
        self.value
    }
}

impl<'a> Display for IdLocal<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if self.canonical {
            f.write_str(&self.value)
        } else {
            escape(f, &self.value)
        }
    }
}

impl<'i> FromPair<'i> for Cow<'i, IdLocal<'i>> {
    const RULE: Rule = Rule::IdLocal;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let inner = pair.into_inner().next().unwrap();
        if inner.as_rule() == Rule::CanonicalIdLocal {
            Ok(Cow::Borrowed(IdLocal::new_unchecked(inner.as_str(), true)))
        } else if inner.as_str().find('\\').is_some() {
            IdentLocal::from_pair_unchecked(inner).map(Cow::Owned)
        } else {
            Ok(Cow::Borrowed(IdLocal::new_unchecked(inner.as_str(), false)))
        }
    }
}
impl_fromslice!('i, Cow<'i, IdLocal<'i>>);

impl<'a> Redeem<'a> for IdLocal<'a> {
    type Owned = IdentLocal;
    fn redeem(&self) -> Self::Owned {
        unsafe {
            IdentLocal::new_unchecked(self.value.to_owned(), self.canonical)
        }
    }
}
