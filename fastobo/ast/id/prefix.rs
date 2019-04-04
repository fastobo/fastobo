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
    let string = s.as_ref();
    let mut chars = string.chars();
    if let Some(c) = chars.next() {
        c.is_ascii_alphabetic() && chars.all(|ref c| char::is_ascii_alphanumeric(c))
    } else {
        false
    }
}


/// An identifier prefix, either canonical or non-canonical.
///
/// * A canonical ID prefix only contains alphabetic characters (`[a-zA-Z]`)
///   followed by either an underscore or other alphabetic characters.
/// * A non-canonical ID prefix can contain any character besides `:`.
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct IdentPrefix {
    value: String,
    canonical: bool,
}

impl IdentPrefix {
    /// Create a new identifier prefix.
    pub fn new(s: String) -> Self {
        Self {
            canonical: is_canonical(&s),
            value: s,
        }
    }

    /// Check if the identifier prefix is canonical or not.
    pub fn is_canonical(&self) -> bool {
        self.canonical
    }

    /// Create a new `IdPrefix` without checking if it is canonical or not.
    pub unsafe fn new_unchecked(s: String, canonical: bool) -> Self {
        Self { value: s, canonical }
    }
}

impl AsRef<str> for IdentPrefix {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl<'a> Borrow<'a, IdPrefix<'a>> for IdentPrefix {
    fn borrow(&'a self) -> IdPrefix<'a> {
        unsafe { IdPrefix::new_unchecked(&self.value, self.canonical) }
    }
}

impl Display for IdentPrefix {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.borrow().fmt(f)
    }
}

impl<'i> FromPair<'i> for IdentPrefix {
    const RULE: Rule = Rule::IdPrefix;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        // Bail out if the local prefix is canonical (alphanumeric only)
        let inner = pair.into_inner().next().unwrap();
        if inner.as_rule() == Rule::CanonicalIdPrefix {
            return Ok(Self {
                value: inner.as_str().to_string(),
                canonical: true,
            });
        }

        // Unescape the prefix if is non canonical.
        // FIXME: count the number of "\" to avoid underestimating the capacity.
        let mut local = String::with_capacity(inner.as_str().len());
        unescape(&mut local, inner.as_str());

        Ok(Self {
            value: local,
            canonical: false,
        })
    }
}
impl_fromstr!(IdentPrefix);

/// A borrowed `IdentPrefix`
#[derive(Clone, Debug)]
pub struct IdPrefix<'a> {
    value: &'a str,
    canonical: bool,
}

impl<'a> IdPrefix<'a> {
    // FIXME(@althonos): no canonical, add another `new_unchecked` method.

    /// Create a new `IdPrf` from a borrowed string slice.
    pub fn new(s: &'a str) -> Self {
        Self {
            canonical: is_canonical(s),
            value: s,
        }
    }

    pub fn as_str(&self) -> &'a str {
        self.value
    }

    /// Create a new `IdPrf` without checking if it is canonical or not.
    pub unsafe fn new_unchecked(s: &'a str, canonical: bool) -> Self {
        Self { value: s, canonical }
    }
}

impl<'a> Into<&'a str> for IdPrefix<'a> {
    fn into(self) -> &'a str {
        self.value
    }
}

impl<'a> AsRef<str> for IdPrefix<'a> {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl<'a> Display for IdPrefix<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        if self.canonical {
            f.write_str(&self.value)
        } else {
            escape(f, &self.value)
        }
    }
}

impl<'a> ToOwned<'a> for IdPrefix<'a> {
    type Owned = IdentPrefix;
    fn to_owned(&self) -> IdentPrefix {
        unsafe {
            IdentPrefix::new_unchecked(self.value.to_string(), self.canonical)
        }
    }
}