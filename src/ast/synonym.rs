use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use fastobo_derive_internal::FromStr;
use pest::iterators::Pair;

use crate::ast::*;
use crate::error::SyntaxError;
use crate::parser::Cache;
use crate::parser::FromPair;
use crate::syntax::Rule;

/// A synonym scope specifier.
#[derive(Clone, Debug, Eq, FromStr, Hash, PartialEq, Ord, PartialOrd)]
pub enum SynonymScope {
    Broad,
    Exact,
    Narrow,
    Related,
}

impl Display for SynonymScope {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::SynonymScope::*;
        match self {
            Exact => f.write_str("EXACT"),
            Broad => f.write_str("BROAD"),
            Narrow => f.write_str("NARROW"),
            Related => f.write_str("RELATED"),
        }
    }
}

impl<'i> FromPair<'i> for SynonymScope {
    const RULE: Rule = Rule::SynonymScope;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        _cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        match pair.as_str() {
            "EXACT" => Ok(SynonymScope::Exact),
            "BROAD" => Ok(SynonymScope::Broad),
            "NARROW" => Ok(SynonymScope::Narrow),
            "RELATED" => Ok(SynonymScope::Related),
            _ => unreachable!(),
        }
    }
}

/// A synonym, denoting an alternative name for the embedding entity.
#[derive(Clone, Debug, Eq, FromStr, Hash, PartialEq, PartialOrd, Ord)]
pub struct Synonym {
    desc: QuotedString,
    scope: SynonymScope,
    ty: Option<Box<SynonymTypeIdent>>,
    xrefs: XrefList,
}

impl Synonym {
    /// Create a `Synonym` with the given description and scope.
    pub fn new<D>(desc: D, scope: SynonymScope) -> Self
    where
        D: Into<QuotedString>,
    {
        Self::with_type(desc, scope, None)
    }

    /// Create a `Synonym` with the given description, scope, and type.
    pub fn with_type<D, T>(desc: D, scope: SynonymScope, ty: T) -> Self
    where
        D: Into<QuotedString>,
        T: Into<Option<SynonymTypeIdent>>,
    {
        Self::with_type_and_xrefs(desc, scope, ty, XrefList::default())
    }

    /// Create a `Synonym` with the given description, scope, and xrefs.
    pub fn with_xrefs<D, L>(desc: D, scope: SynonymScope, xrefs: L) -> Self
    where
        D: Into<QuotedString>,
        L: Into<XrefList>,
    {
        Self::with_type_and_xrefs(desc, scope, None, xrefs)
    }

    /// Create a `Synonym` with the given description, scope, type, and xrefs.
    pub fn with_type_and_xrefs<D, T, L>(desc: D, scope: SynonymScope, ty: T, xrefs: L) -> Self
    where
        D: Into<QuotedString>,
        T: Into<Option<SynonymTypeIdent>>,
        L: Into<XrefList>,
    {
        Self {
            desc: desc.into(),
            scope,
            ty: ty.into().map(Box::new),
            xrefs: xrefs.into(),
        }
    }
}

impl Synonym {
    /// Get a reference to the description of the `Synonym`.
    pub fn description(&self) -> &QuotedString {
        &self.desc
    }

    /// Get a mutable reference to the description of the `Synonym`.
    pub fn description_mut(&mut self) -> &mut QuotedString {
        &mut self.desc
    }

    /// Get a reference to the scope of the `Synonym`.
    pub fn scope(&self) -> &SynonymScope {
        &self.scope
    }

    /// Get a mutable reference to the scope of the `Synonym`.
    pub fn scope_mut(&mut self) -> &mut SynonymScope {
        &mut self.scope
    }

    /// Get a reference to the type of the `Synonym`, if any.
    pub fn ty(&self) -> Option<&SynonymTypeIdent> {
        self.ty.as_deref()
    }

    /// Get a mutable reference to the type of the `Synonym`, if any.
    pub fn ty_mut(&mut self) -> Option<&mut SynonymTypeIdent> {
        self.ty.as_deref_mut()
    }

    /// Get a reference to the xrefs of the `Synonym`.
    pub fn xrefs(&self) -> &XrefList {
        &self.xrefs
    }

    /// Get a mutable reference to the xrefs of the `Synonym`.
    pub fn xrefs_mut(&mut self) -> &mut XrefList {
        &mut self.xrefs
    }
}

impl Display for Synonym {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.desc
            .fmt(f)
            .and(f.write_char(' '))
            .and(self.scope.fmt(f))
            .and(f.write_char(' '))?;

        if let Some(ref syntype) = self.ty {
            syntype.fmt(f).and(f.write_char(' '))?;
        }

        self.xrefs.fmt(f)
    }
}

impl<'i> FromPair<'i> for Synonym {
    const RULE: Rule = Rule::Synonym;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();

        let desc = QuotedString::from_pair_unchecked(inner.next().unwrap(), cache)?;
        let scope = SynonymScope::from_pair_unchecked(inner.next().unwrap(), cache)?;

        let nxt = inner.next().unwrap();
        match nxt.as_rule() {
            Rule::SynonymTypeId => {
                let ty = Some(Box::new(SynonymTypeIdent::from_pair_unchecked(nxt, cache)?));
                let xrefs = XrefList::from_pair_unchecked(inner.next().unwrap(), cache)?;
                Ok(Synonym {
                    desc,
                    scope,
                    ty,
                    xrefs,
                })
            }
            Rule::XrefList => {
                let ty = None;
                let xrefs = XrefList::from_pair_unchecked(nxt, cache)?;
                Ok(Synonym {
                    desc,
                    scope,
                    ty,
                    xrefs,
                })
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    mod scope {

        use self::assert_eq;
        use self::SynonymScope::*;
        use super::*;

        #[test]
        fn from_str() {
            assert_eq!(SynonymScope::from_str("EXACT").unwrap(), Exact);
            assert_eq!(SynonymScope::from_str("BROAD").unwrap(), Broad);
            assert_eq!(SynonymScope::from_str("NARROW").unwrap(), Narrow);
            assert_eq!(SynonymScope::from_str("RELATED").unwrap(), Related);
            assert!(SynonymScope::from_str("something").is_err());
        }

        #[test]
        fn to_string() {
            assert_eq!(Exact.to_string(), "EXACT");
            assert_eq!(Broad.to_string(), "BROAD");
            assert_eq!(Narrow.to_string(), "NARROW");
            assert_eq!(Related.to_string(), "RELATED");
        }
    }

    mod synonym {

        use self::assert_eq;
        use super::*;

        #[test]
        fn from_str() {
            let actual = Synonym::from_str(
                "\"ssDNA-specific endodeoxyribonuclease activity\" RELATED [GOC:mah]",
            )
            .unwrap();
            let expected = Synonym::with_xrefs(
                "ssDNA-specific endodeoxyribonuclease activity",
                SynonymScope::Related,
                vec![Xref::new(PrefixedIdent::new("GOC", "mah"))],
            );
            assert_eq!(actual, expected);
        }

        #[test]
        fn to_string() {
            let s = Synonym::with_xrefs(
                QuotedString::new(String::from(
                    "ssDNA-specific endodeoxyribonuclease activity",
                )),
                SynonymScope::Related,
                vec![Xref::new(Ident::from(PrefixedIdent::new("GOC", "mah")))],
            );

            assert_eq!(
                s.to_string(),
                "\"ssDNA-specific endodeoxyribonuclease activity\" RELATED [GOC:mah]"
            );
        }
    }
}
