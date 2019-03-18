use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;

use pest::iterators::Pair;

use super::super::parser::FromPair;
use super::super::parser::Parser;
use super::super::parser::Rule;
use super::ClassId;
use super::Id;
use super::IdPrefix;
use super::Iri;
use super::Line;
use super::NaiveDate;
use super::NamespaceId;
use super::PropertyValue;
use super::QuotedString;
use super::RelationId;
use super::SubsetId;
use super::SynonymScope;
use super::SynonymTypeId;
use super::UnquotedString;
use crate::error::Error;
use crate::error::Result;

/// The header frame, containing metadata about an OBO document.
pub struct HeaderFrame {
    clauses: Vec<HeaderClause>,
}

/// An clause appearing in a header frame.
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum HeaderClause {
    FormatVersion(UnquotedString),
    DataVersion(UnquotedString),
    Date(NaiveDate),
    SavedBy(UnquotedString),
    AutoGeneratedBy(UnquotedString),
    Import(Import),
    Subsetdef(SubsetId, QuotedString),
    SynonymTypedef(SynonymTypeId, QuotedString, Option<SynonymScope>),
    DefaultNamespace(NamespaceId),
    Idspace(IdPrefix, Iri, Option<QuotedString>),
    TreatXrefsAsEquivalent(IdPrefix),
    TreatXrefsAsGenusDifferentia(IdPrefix, RelationId, ClassId),
    TreatXrefsAsReverseGenusDifferentia(IdPrefix, RelationId, ClassId),
    TreatXrefsAsRelationship(IdPrefix, RelationId),
    TreatXrefsAsIsA(IdPrefix),
    TreatXrefsAsHasSubclass(IdPrefix),
    PropertyValue(Line<PropertyValue>),
    Remark(UnquotedString),
    Ontology(UnquotedString),
    OwlAxioms(UnquotedString),
    Unreserved(UnquotedString, UnquotedString),
}

impl Display for HeaderClause {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::HeaderClause::*;
        match self {
            FormatVersion(version) => f.write_str("format-version: ").and(version.fmt(f)),
            DataVersion(version) => f.write_str("data-version: ").and(version.fmt(f)),
            Date(date) => f.write_str("date: ").and(date.fmt(f)),
            SavedBy(person) => f.write_str("saved-by: ").and(person.fmt(f)),
            AutoGeneratedBy(thing) => f.write_str("auto-generated-by: ").and(thing.fmt(f)),
            Import(import) => f.write_str("import: ").and(import.fmt(f)),
            Subsetdef(subset, desc) => f
                .write_str("subsetdef: ")
                .and(subset.fmt(f))
                .and(f.write_char(' '))
                .and(desc.fmt(f)),
            SynonymTypedef(syntype, desc, optscope) => {
                f.write_str("synonymtypedef: ")
                    .and(syntype.fmt(f))
                    .and(f.write_char(' '))
                    .and(desc.fmt(f))?;
                match optscope {
                    Some(scope) => f.write_char(' ').and(scope.fmt(f)),
                    None => Ok(()),
                }
            }
            DefaultNamespace(ns) => f.write_str("default-namespace: ").and(ns.fmt(f)),
            Idspace(prefix, iri, optdesc) => {
                f.write_str("idspace: ")
                    .and(prefix.fmt(f))
                    .and(f.write_char(' '))
                    .and(iri.fmt(f))?;
                match optdesc {
                    Some(desc) => f.write_char(' ').and(desc.fmt(f)),
                    None => Ok(()),
                }
            }
            TreatXrefsAsEquivalent(prefix) => f
                .write_str("treat-xrefs-as-equivalent: ")
                .and(prefix.fmt(f)),
            TreatXrefsAsGenusDifferentia(prefix, rel, cls) => f
                .write_str("treat-xrefs-as-genus-differentia: ")
                .and(prefix.fmt(f))
                .and(f.write_char(' '))
                .and(rel.fmt(f))
                .and(f.write_char(' '))
                .and(cls.fmt(f)),
            TreatXrefsAsReverseGenusDifferentia(prefix, rel, cls) => f
                .write_str("treat-xrefs-as-reverse-genus-differentia: ")
                .and(prefix.fmt(f))
                .and(f.write_char(' '))
                .and(rel.fmt(f))
                .and(f.write_char(' '))
                .and(cls.fmt(f)),
            TreatXrefsAsRelationship(prefix, rel) => f
                .write_str("treat-xrefs-as-relationship: ")
                .and(prefix.fmt(f))
                .and(f.write_char(' '))
                .and(rel.fmt(f)),
            TreatXrefsAsIsA(prefix) => f.write_str("treat-xrefs-as-is_a: ").and(prefix.fmt(f)),
            TreatXrefsAsHasSubclass(prefix) => f
                .write_str("treat-xrefs-as-has-subclass")
                .and(prefix.fmt(f)),
            PropertyValue(line) => f.write_str("property_value: ").and(line.fmt(f)),
            Remark(remark) => f.write_str("remark: ").and(remark.fmt(f)),
            Ontology(ont) => f.write_str("ontology: ").and(ont.fmt(f)),
            OwlAxioms(axioms) => f.write_str("owl-axioms: ").and(axioms.fmt(f)),
            Unreserved(key, value) => key.fmt(f).and(f.write_str(": ")).and(value.fmt(f)),
        }
    }
}

impl FromPair for HeaderClause {
    const RULE: Rule = Rule::HeaderClause;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let tag = inner.next().unwrap();
        match tag.as_rule() {
            Rule::FormatVersionTag => {
                let version = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::FormatVersion(version))
            }
            Rule::DataVersionTag => {
                let version = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::DataVersion(version))
            }
            Rule::DateTag => {
                let date = NaiveDate::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::Date(date))
            }
            Rule::SavedByTag => {
                let person = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::SavedBy(person))
            }
            Rule::AutoGeneratedByTag => {
                let soft = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::AutoGeneratedBy(soft))
            }
            Rule::ImportTag => {
                let import = Import::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::Import(import))
            }
            Rule::SubsetdefTag => {
                let subset = SubsetId::from_pair_unchecked(inner.next().unwrap())?;
                let desc = QuotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::Subsetdef(subset, desc))
            }
            Rule::SynonymTypedefTag => {
                let id = SynonymTypeId::from_pair_unchecked(inner.next().unwrap())?;
                let desc = QuotedString::from_pair_unchecked(inner.next().unwrap())?;
                let scope = match inner.next() {
                    Some(pair) => Some(SynonymScope::from_pair_unchecked(pair)?),
                    None => None,
                };
                Ok(HeaderClause::SynonymTypedef(id, desc, scope))
            }
            Rule::DefaultNamespaceTag => {
                let id = NamespaceId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::DefaultNamespace(id))
            }
            Rule::IdspaceTag => {
                let prefix = IdPrefix::from_pair_unchecked(inner.next().unwrap())?;
                let iri = Iri::from_pair_unchecked(inner.next().unwrap())?;
                let desc = match inner.next() {
                    Some(pair) => Some(QuotedString::from_pair_unchecked(pair)?),
                    None => None,
                };
                Ok(HeaderClause::Idspace(prefix, iri, desc))
            }
            Rule::TreatXrefsAsEquivalentTag => {
                let prefix = IdPrefix::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::TreatXrefsAsEquivalent(prefix))
            }
            Rule::TreatXrefsAsGenusDifferentiaTag => {
                let prefix = IdPrefix::from_pair_unchecked(inner.next().unwrap())?;
                let rel = RelationId::from_pair_unchecked(inner.next().unwrap())?;
                let cls = ClassId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::TreatXrefsAsGenusDifferentia(prefix, rel, cls))
            }
            Rule::TreatXrefsAsReverseGenusDifferentiaTag => {
                let prefix = IdPrefix::from_pair_unchecked(inner.next().unwrap())?;
                let rel = RelationId::from_pair_unchecked(inner.next().unwrap())?;
                let cls = ClassId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::TreatXrefsAsReverseGenusDifferentia(
                    prefix, rel, cls,
                ))
            }
            Rule::TreatXrefsAsRelationshipTag => {
                let prefix = IdPrefix::from_pair_unchecked(inner.next().unwrap())?;
                let rel = RelationId::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::TreatXrefsAsRelationship(prefix, rel))
            }
            Rule::TreatXrefsAsIsATag => {
                let prefix = IdPrefix::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::TreatXrefsAsIsA(prefix))
            }
            Rule::TreatXrefsAsHasSubclassTag => {
                let prefix = IdPrefix::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::TreatXrefsAsHasSubclass(prefix))
            }
            Rule::PropertyValueTag => {
                // Parse the property value
                let relid = RelationId::from_pair_unchecked(inner.next().unwrap())?;
                let second = inner.next().unwrap();
                let property_value = match second.as_rule() {
                    Rule::Id => {
                        let id = Id::from_pair_unchecked(second)?;
                        PropertyValue::Identified(relid, id)
                    }
                    Rule::QuotedString => {
                        let desc = QuotedString::from_pair_unchecked(second)?;
                        let datatype = Id::from_str(inner.next().unwrap().as_str())?;
                        PropertyValue::Typed(relid, desc, datatype)
                    }
                    _ => unreachable!(),
                };

                // Parse the rest of the line
                let pair1 = inner.next();
                let pair2 = inner.next();
                // if let Some(pair) = pair2 {
                //     let comment = Comment::from_pair_unchecked(pair)?;
                //     let qualifiers = Vec<Qualifiers>::from_pair_unchecked(pair1.unwrap())?;
                //
                //     unimplem
                //
                // } else {
                //     match pair
                //
                //
                // }
                unimplemented!()
            }
            Rule::RemarkTag => {
                let remark = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::Remark(remark))
            }
            Rule::OntologyTag => {
                let ont = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::Ontology(ont))
            }
            Rule::OwlAxiomsTag => {
                let axioms = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::OwlAxioms(axioms))
            }
            Rule::UnquotedString => {
                let tag = UnquotedString::from_pair_unchecked(tag)?;
                let value = UnquotedString::from_pair_unchecked(inner.next().unwrap())?;
                Ok(HeaderClause::Unreserved(tag, value))
            }
            _ => unreachable!(),
        }
    }
}
impl_fromstr!(HeaderClause);

/// A reference to another document to be imported.
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Import {
    Iri(Iri),
    Abbreviated(Id), // QUESTION(@althonos): UnprefixedID ?
}

impl From<Iri> for Import {
    fn from(iri: Iri) -> Self {
        Import::Iri(iri)
    }
}

impl From<Id> for Import {
    fn from(id: Id) -> Self {
        Import::Abbreviated(id)
    }
}

impl Display for Import {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::Import::*;
        match self {
            Iri(iri) => iri.fmt(f),
            Abbreviated(id) => id.fmt(f),
        }
    }
}

impl FromPair for Import {
    const RULE: Rule = Rule::Import;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::Iri => Iri::from_pair_unchecked(inner).map(From::from),
            Rule::Id => Id::from_pair_unchecked(inner).map(From::from),
            _ => unreachable!(),
        }
    }
}
impl_fromstr!(Import);

#[cfg(test)]
mod tests {

    use super::super::UnprefixedId;
    use super::*;
    use std::str::FromStr;

    mod clause {

        use super::*;

        #[test]
        fn from_str() {
            let actual = HeaderClause::from_str("format-version: 1.2").unwrap();
            let expected = HeaderClause::FormatVersion(UnquotedString::new("1.2"));
            assert_eq!(actual, expected);

            let actual = HeaderClause::from_str("subsetdef: GO_SLIM \"GO Slim\"").unwrap();
            let expected = HeaderClause::Subsetdef(
                SubsetId::from(Id::from(UnprefixedId::new("GO_SLIM"))),
                QuotedString::new("GO Slim"),
            );
            assert_eq!(actual, expected);

            let actual = HeaderClause::from_str("date: 17:03:2019 20:16").unwrap();
            let expected = HeaderClause::Date(NaiveDate::new(17, 3, 2019, 20, 16));
            assert_eq!(actual, expected);
        }

    }

}