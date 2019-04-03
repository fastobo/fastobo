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

/// An identifier with a prefix.
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub struct PrefixedIdentifier {
    prefix: IdPrefix,
    local: IdLocal,
}

impl PrefixedIdentifier {
    /// Create a new `PrefixedIdentifier` from a prefix and a local identifier.
    pub fn new(prefix: IdPrefix, local: IdLocal) -> Self {
        Self { prefix, local }
    }

    /// Check if the prefixed identifier is canonical or not.
    ///
    /// # Example
    /// ```rust
    /// # extern crate fastobo;
    /// # use fastobo::ast::*;
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

impl<'a> Borrow<'a, PrefixedId<'a>> for PrefixedIdentifier {
    fn borrow(&'a self) -> PrefixedId<'a> {
        PrefixedId::new(
            self.prefix.borrow(),
            self.local.borrow(),
        )
    }
}

impl Display for PrefixedIdentifier {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.prefix
            .fmt(f)
            .and(f.write_char(':'))
            .and(self.local.fmt(f))
    }
}

impl<'i> FromPair<'i> for PrefixedIdentifier {
    const RULE: Rule = Rule::PrefixedId;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let mut inners = pair.into_inner();
        let prefix = IdPrefix::from_pair_unchecked(inners.next().unwrap())?;
        let local = IdLocal::from_pair_unchecked(inners.next().unwrap())?;
        Ok(Self::new(prefix, local))
    }
}
impl_fromstr!(PrefixedIdentifier);

/// A borrowed `PrefixedIdentifier`
#[derive(Clone, Debug)]
pub struct PrefixedId<'a> {
    prefix: Cow<'a, IdPrf<'a>>,
    local: Cow<'a, IdLcl<'a>>,
}

impl<'a> PrefixedId<'a> {
    /// Create a new `PrefixedId` from references.
    pub fn new(prefix: IdPrf<'a>, local: IdLcl<'a>) -> Self {
        Self {
            prefix: Cow::Borrowed(prefix),
            local: Cow::Borrowed(local),
        }
    }
}

impl<'a> ToOwned<'a> for PrefixedId<'a> {
    type Owned = PrefixedIdentifier;
    fn to_owned(&'a self) -> PrefixedIdentifier {
        PrefixedIdentifier::new(
            <Cow<IdPrf> as ToOwned<'a>>::to_owned(&self.prefix),
            <Cow<IdLcl> as ToOwned<'a>>::to_owned(&self.local),
        )
    }
}

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
    pub fn new(s: &'a str, canonical: bool) -> Self {
        IdPrf {
            value: s,
            canonical
        }
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
