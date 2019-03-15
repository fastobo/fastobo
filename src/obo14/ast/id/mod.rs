#[macro_use]
mod _macros;

use std::convert::TryFrom;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use iri_string::Url;
use pest::iterators::Pair;

use super::super::parser::FromPair;
use super::super::parser::Parser;
use super::super::parser::Rule;
use crate::error::Error;
use crate::error::Result;

/// An identifier, either prefixed, unprefixed, or a valid URL.
#[derive(Debug)]
pub enum Id {
    Prefixed(PrefixedId),
    Unprefixed(UnprefixedId),
    Url(Url),
}

impl From<PrefixedId> for Id {
    fn from(id: PrefixedId) -> Self {
        Id::Prefixed(id)
    }
}

impl From<UnprefixedId> for Id {
    fn from(id: UnprefixedId) -> Self {
        Id::Unprefixed(id)
    }
}

impl From<Url> for Id {
    fn from(url: Url) -> Self {
        Id::Url(url)
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::Id::*;
        match self {
            Prefixed(id) => id.fmt(f),
            Unprefixed(id) => id.fmt(f),
            Url(url) => url.fmt(f),
        }
    }
}

impl FromPair for Id {
    const RULE: Rule = Rule::Id;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::PrefixedId => PrefixedId::from_pair_unchecked(inner).map(From::from),
            Rule::UnprefixedId => UnprefixedId::from_pair_unchecked(inner).map(From::from),
            // FIXME(@althonos): need proper error report if the parser fails.
            Rule::UrlId => Ok(Id::Url(Url::parse(inner.as_str()).unwrap())),
            _ => unreachable!(),
        }
    }
}
impl_fromstr!(Id);

/// An identifier without a prefix.
#[derive(Debug)]
pub struct UnprefixedId {
    value: String,
}

impl Display for UnprefixedId {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
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

impl FromPair for UnprefixedId {
    const RULE: Rule = Rule::UnprefixedId;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        let mut local = String::with_capacity(pair.as_str().len());
        let mut chars = pair.as_str().chars();
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

        Ok(UnprefixedId { value: local })
    }
}
impl_fromstr!(UnprefixedId);

/// An identifier with a prefix.
#[derive(Debug)]
pub struct PrefixedId {
    prefix: IdPrefix,
    local: IdLocal,
}

impl PrefixedId {

    /// Create a new `PrefixedId` from a prefix and a local identifier.
    pub fn new(prefix: IdPrefix, local: IdLocal) -> Self {
        Self {prefix, local}
    }

    /// Check if the prefixed identifier is canonical or not.
    ///
    /// # Example
    /// ```rust
    /// # extern crate ontology;
    /// # use ontology::obo14::*;
    /// # use std::str::FromStr;
    /// let canonical_id = PrefixedId::from_str("GO:0046154").unwrap();
    /// assert!(canonical_id.is_canonical());
    ///
    /// let noncanonical_id = PrefixedId::from_str("PATO:something").unwrap();
    /// assert!(!noncanonical_id.is_canonical());
    pub fn is_canonical(&self) -> bool {
        match (&self.prefix, &self.local) {
            (IdPrefix::Canonical(_), IdLocal::Canonical(_)) => true,
            (_, _) => false,
        }
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
        Ok(PrefixedId { prefix, local })
    }
}
impl_fromstr!(PrefixedId);

/// An identifier prefix, either canonical or non-canonical.
///
/// * A canonical ID prefix only contains alphabetic characters (`[a-zA-Z]`)
///   followed by either an underscore or other alphabetic characters.
/// * A non-canonical ID prefix can contain any character besides `:`.
#[derive(Debug)]
pub enum IdPrefix {
    Canonical(String),
    NonCanonical(String),
}

impl IdPrefix {
    /// Check if the identifier prefix is canonical or not.
    pub fn is_canonical(&self) -> bool {
        match self {
            IdPrefix::Canonical(_) => true,
            IdPrefix::NonCanonical(_) => false
        }
    }
}

impl Display for IdPrefix {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::IdPrefix::*;
        match self {
            Canonical(s) => f.write_str(s),
            NonCanonical(s) => s.chars().try_for_each(|char| match char {
                '\r' => f.write_str("\\r"),
                '\n' => f.write_str("\\n"),
                '\u{000c}' => f.write_str("\\f"),
                ' ' => f.write_str("\\ "),
                '\t' => f.write_str("\\t"),
                ':' => f.write_str("\\:"), // FIXME(@althonos) ?
                _ => f.write_char(char),
            }),
        }
    }
}

impl FromPair for IdPrefix {
    const RULE: Rule = Rule::IdPrefix;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        // Bail out if the local prefix is canonical (alphanumeric only)
        let inner = pair.into_inner().next().unwrap();
        if inner.as_rule() == Rule::CanonicalIdPrefix {
            return Ok(IdPrefix::Canonical(inner.as_str().to_string()));
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

        Ok(IdPrefix::NonCanonical(local))
    }
}
impl_fromstr!(IdPrefix);


/// A local identifier, preceded by a prefix in prefixed IDs.
///
/// * A canonical local ID only contains digits (`[0-9]`).
/// * A non-canonical local ID can contain any character excepting
///   whitespaces and newlines.
#[derive(Debug)]
pub enum IdLocal {
    Canonical(String),
    NonCanonical(String),
}

impl IdLocal {

    /// Create a new local identifier.
    pub fn new<S>(s: S) -> Self
    where
        S: AsRef<str> + Into<String>
    {
        for char in s.as_ref().chars() {
            match char {
                '0'...'9' => (),
                _ => return IdLocal::NonCanonical(s.into()),
            }
        }
        IdLocal::Canonical(s.into())
    }

    /// Check if the local identifier is canonical or not.
    pub fn is_canonical(&self) -> bool {
        match self {
            IdLocal::Canonical(_) => true,
            IdLocal::NonCanonical(_) => false
        }
    }


}

impl Display for IdLocal {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::IdLocal::*;
        match self {
            Canonical(s) => f.write_str(s),
            NonCanonical(s) => s.chars().try_for_each(|char| match char {
                '\r' => f.write_str("\\r"),
                '\n' => f.write_str("\\n"),
                '\u{000c}' => f.write_str("\\f"),
                ' ' => f.write_str("\\ "),
                '\t' => f.write_str("\\t"),
                _ => f.write_char(char),
            }),
        }
    }
}

impl FromPair for IdLocal {
    const RULE: Rule = Rule::IdLocal;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        // Bail out if the local ID is canonical (digits only).
        let inner = pair.into_inner().next().unwrap();
        if inner.as_rule() == Rule::CanonicalIdLocal {
            return Ok(IdLocal::Canonical(inner.as_str().to_string()));
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

        Ok(IdLocal::NonCanonical(local))
    }
}
impl_fromstr!(IdLocal);


// NB(@althonos): All identifiers are defined as separate typedefs so that
//                `PartialEq` is not implemented and trying to compare a
//                `ClassId` with a `RelationId` would fail at compile-time.
id_subclasses! {
    /// A unique identifier for a class (*i.e.* a term).
    pub struct ClassId;

    /// A unique identifier for a typedef (*i.e.* a relation).
    pub struct RelationId;

    /// A unique identifier for an instance.
    pub struct InstanceId;

    /// A unique identifier for a subset
    pub struct SubsetId;

    /// A unique identifier for a person (used in the `created_by` clause).
    pub struct PersonId;

    /// A unique identifier for a synonym type.
    pub struct SynonymTypeId;

    /// An OBO namespace.
    pub struct NamespaceId;
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;
    use super::*;

    mod prefixed {

        use super::*;

        #[test]
        fn from_str() {

            let actual = PrefixedId::from_str("GO:0046154");
            // let expected = PrefixedId::new();
        }
    }

}
