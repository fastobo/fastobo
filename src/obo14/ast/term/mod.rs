use super::ClassId;
use super::Id;
use super::IsoDate;
use super::Line;
use super::NamespaceId;
use super::PersonId;
use super::PropertyValue;
use super::QuotedString;
use super::RelationId;
use super::SubsetId;
use super::SynonymScope;
use super::SynonymTypeId;
use super::UnquotedString;
use super::Xref;

/// A term frame, describing a class.
pub struct TermFrame {
    id: Line<ClassId>,
    clauses: Vec<Line<TermClause>>,
}

/// A clause appearing in a term frame.
pub enum TermClause {
    IsAnonymous(bool),
    Name(UnquotedString),
    Namespace(NamespaceId),
    AltId(Id),
    Def(QuotedString, Vec<Xref>),
    Comment(UnquotedString),
    Subset(SubsetId),
    Synonym(QuotedString, SynonymScope, Option<SynonymTypeId>, Vec<Xref>),
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
    ExpandAssertionTo(QuotedString, Vec<Xref>),
    ExpandExpressionTO(QuotedString, Vec<Xref>),
    IsMetadata(bool),
    IsClassLevel(bool),
}
