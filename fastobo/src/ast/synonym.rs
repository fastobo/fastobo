use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use pest::iterators::Pair;
use url::Url;

use crate::ast::*;
use crate::error::Error;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

/// A synonym scope specifier.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SynonymScope {
    Exact,
    Broad,
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
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        match pair.as_str() {
            "EXACT" => Ok(SynonymScope::Exact),
            "BROAD" => Ok(SynonymScope::Broad),
            "NARROW" => Ok(SynonymScope::Narrow),
            "RELATED" => Ok(SynonymScope::Related),
            _ => unreachable!(),
        }
    }
}
impl_fromstr!(SynonymScope);

/// A synonym, denoting an alternative name for the embedding entity.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Synonym {
    pub desc: QuotedString,
    pub scope: SynonymScope,
    pub ty: Option<SynonymTypeIdent>,
    pub xrefs: XrefList,
}

impl Synonym {

    pub fn new(desc: QuotedString, scope: SynonymScope) -> Self {
        Self {
            desc,
            scope,
            ty: Default::default(),
            xrefs: Default::default(),
        }
    }

    pub fn with_type<T>(desc: QuotedString, scope: SynonymScope, ty: T) -> Self
    where
        T: Into<Option<SynonymTypeIdent>>,
    {
        Self {
            desc,
            scope,
            ty: ty.into(),
            xrefs: Default::default(),
        }
    }

    pub fn with_xrefs(desc: QuotedString, scope: SynonymScope, xrefs: XrefList) -> Self {
        Self {
            desc,
            scope,
            ty: None,
            xrefs,
        }
    }

    pub fn with_type_and_xrefs<T>(
        desc: QuotedString,
        scope: SynonymScope,
        ty: T,
        xrefs: XrefList,
    ) -> Self
    where
        T: Into<Option<SynonymTypeIdent>>,
    {
        Self {
            desc,
            scope,
            ty: ty.into(),
            xrefs,
        }
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
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();

        let desc = QuotedString::from_pair_unchecked(inner.next().unwrap())?;
        let scope = SynonymScope::from_pair_unchecked(inner.next().unwrap())?;

        let nxt = inner.next().unwrap();
        match nxt.as_rule() {
            Rule::SynonymTypeId => {
                let ty = Some(SynonymTypeIdent::from_pair_unchecked(nxt)?);
                let xrefs = XrefList::from_pair_unchecked(inner.next().unwrap())?;
                Ok(Synonym {
                    desc,
                    scope,
                    ty,
                    xrefs,
                })
            }
            Rule::XrefList => {
                let ty = None;
                let xrefs = XrefList::from_pair_unchecked(nxt)?;
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
impl_fromstr!(Synonym);



#[cfg(test)]
mod tests {

    use std::str::FromStr;
    use super::*;

    mod scope {

        use super::*;
        use self::SynonymScope::*;

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

        use super::*;

        #[test]
        fn from_str() {
            let actual = Synonym::from_str("\"ssDNA-specific endodeoxyribonuclease activity\" RELATED [GOC:mah]").unwrap();
            let expected = Synonym::with_xrefs(
                QuotedString::new("ssDNA-specific endodeoxyribonuclease activity"),
                SynonymScope::Related,
                XrefList::new(vec![Xref::new(PrefixedIdent::new("GOC", "mah"))]),
            );
            assert_eq!(actual, expected);
        }

        #[test]
        fn to_string() {
            let s = Synonym::with_xrefs(
                QuotedString::new(String::from("ssDNA-specific endodeoxyribonuclease activity")),
                SynonymScope::Related,
                XrefList::new(vec![Xref::new(
                    Ident::from(
                        PrefixedIdent::new(
                            IdentPrefix::new(String::from("GOC")),
                            IdentLocal::new(String::from("mah"))
                        )
                    )
                )])
            );

            assert_eq!(s.to_string(), "\"ssDNA-specific endodeoxyribonuclease activity\" RELATED [GOC:mah]");
        }

    }


}
