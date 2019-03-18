use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use crate::obo14::ast::ClassId;
use crate::obo14::ast::Id;
use crate::obo14::ast::IsoDate;
use crate::obo14::ast::Line;
use crate::obo14::ast::NamespaceId;
use crate::obo14::ast::PersonId;
use crate::obo14::ast::PropertyValue;
use crate::obo14::ast::QuotedString;
use crate::obo14::ast::RelationId;
use crate::obo14::ast::SubsetId;
use crate::obo14::ast::SynonymScope;
use crate::obo14::ast::SynonymTypeId;
use crate::obo14::ast::UnquotedString;
use crate::obo14::ast::Xref;
use crate::obo14::ast::XrefList;

/// A clause appearing in a term frame.
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum TermClause {
    IsAnonymous(bool),
    Name(UnquotedString),
    Namespace(NamespaceId),
    AltId(Id),
    Def(QuotedString, XrefList),
    Comment(UnquotedString),
    Subset(SubsetId),
    Synonym(QuotedString, SynonymScope, Option<SynonymTypeId>, XrefList),
    Xref(Xref),
    Builtin(bool),
    PropertyValue(PropertyValue),
    IsA(ClassId),
    IntersectionOf(Option<RelationId>, ClassId),
    UnionOf(ClassId),
    EquivalentTo(ClassId),
    DisjointFrom(ClassId),
    Relationship(RelationId, ClassId),
    IsObsolete(bool),
    ReplacedBy(ClassId),
    Consider(ClassId),
    CreatedBy(PersonId),
    CreationDate(IsoDate),
    // FIXME(@althonos): in the guide but not in the syntax.
    // ExpandAssertionTo(QuotedString, XrefList),
    // ExpandExpressionTO(QuotedString, XrefList),
    // IsMetadata(bool),
    // IsClassLevel(bool),
}


impl Display for TermClause {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::TermClause::*;
        match self {
            IsAnonymous(b) => f.write_str("is_anonymous: ").and(b.fmt(f)),
            Name(name) => f.write_str("name: ").and(name.fmt(f)),
            Namespace(id) => f.write_str("namespace: ").and(id.fmt(f)),
            AltId(id) => f.write_str("alt_id: ").and(id.fmt(f)),
            Def(desc, xreflist) =>
                f.write_str("def: ").and(desc.fmt(f)).and(xreflist.fmt(f)),
            Comment(comment) =>
                f.write_str("comment: ").and(comment.fmt(f)),
            Subset(subset) => f.write_str("subset: ").and(subset.fmt(f)),
            Synonym(desc, scope, opttype, xreflist) => {
                f.write_str("synonym: ").and(desc.fmt(f)).and(f.write_char(' '))
                    .and(scope.fmt(f))?;
                if let Some(syntype) = opttype {
                    f.write_char(' ').and(syntype.fmt(f))?;
                }
                f.write_char(' ').and(xreflist.fmt(f))
            }
            Xref(xref) => f.write_str("xref: ").and(xref.fmt(f)),
            Builtin(b) => f.write_str("builtin: ").and(b.fmt(f)),
            PropertyValue(pv) => f.write_str("property_value: ").and(pv.fmt(f)),
            IsA(id) => f.write_str("is_a: ").and(id.fmt(f)),
            IntersectionOf(Some(rel), id) =>
                f.write_str("intersection_of: ").and(rel.fmt(f)).and(f.write_char(' '))
                    .and(id.fmt(f)),
            IntersectionOf(None, id) =>
                f.write_str("intersection_of: ").and(id.fmt(f)),
            UnionOf(id) => f.write_str("union_of: ").and(id.fmt(f)),
            EquivalentTo(id) => f.write_str("equivalent_to: ").and(id.fmt(f)),
            DisjointFrom(id) => f.write_str("disjoint_from: ").and(id.fmt(f)),
            Relationship(rel, id) => f.write_str("relationship: ").and(rel.fmt(f))
                .and(f.write_char(' ')).and(id.fmt(f)),
            IsObsolete(b) => f.write_str("is_obsolete: ").and(b.fmt(f)),
            ReplacedBy(id) => f.write_str("replaced_by: ").and(id.fmt(f)),
            Consider(id) => f.write_str("consider: ").and(id.fmt(f)),
            CreatedBy(id) => f.write_str("created_by: ").and(id.fmt(f)),
            CreationDate(date) => f.write_str("creation_date: ").and(date.fmt(f)),
        }
    }
}
