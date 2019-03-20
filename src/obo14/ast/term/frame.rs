use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use pest::iterators::Pair;

use crate::obo14::parser::FromPair;
use crate::obo14::parser::Parser;
use crate::obo14::parser::Rule;
use crate::obo14::ast::Line;
use crate::obo14::ast::ClassId;
use super::TermClause;
use crate::error::Result;

/// A term frame, describing a class.
pub struct TermFrame {
    id: Line<ClassId>,
    clauses: Vec<Line<TermClause>>,
}

impl Display for TermFrame {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str("[Term]\nid: ").and(self.id.fmt(f))?;
        self.clauses.iter().try_for_each(|clause| clause.fmt(f))
    }
}

impl FromPair for TermFrame {
    const RULE: Rule = Rule::TermFrame;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let clsid = ClassId::from_pair_unchecked(inner.next().unwrap())?;
        let id = Line::<()>::from_pair_unchecked(inner.next().unwrap())?
            .with_content(clsid);

        let mut clauses = Vec::new();
        for pair in inner {
            clauses.push(Line::<TermClause>::from_pair_unchecked(pair)?);
        }

        Ok(TermFrame { id, clauses })
    }
}
impl_fromstr!(TermFrame);


#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use crate::obo14::ast::PrefixedId;
    use crate::obo14::ast::IdPrefix;
    use crate::obo14::ast::IdLocal;
    use crate::obo14::ast::Id;
    use super::*;

    #[test]
    fn from_str() {
        let actual = TermFrame::from_str(
            "[Term]
            id: MS:1000008
            name: ionization type
            def: \"The method by which gas phase ions are generated from the sample.\" [PSI:MS]
            relationship: part_of MS:1000458 ! source\n"
        ).unwrap();
        assert_eq!(
            actual.id.as_ref(),
            &ClassId::from(Id::from(PrefixedId::new(IdPrefix::new("MS"), IdLocal::new("1000008"))))
        );

        let actual = TermFrame::from_str(
            "[Term]
            id: PO:0000067
            name: proteoid root
            namespace: plant_anatomy
            xref: PO_GIT:588
            is_a: PO:0009005 ! root
            created_by: austinmeier
            creation_date: 2015-08-11T15:05:12Z\n"
        ).unwrap();


    }

}
