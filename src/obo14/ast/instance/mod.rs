use super::ClassId;
use super::Id;
use super::InstanceId;
use super::IsoDate;
use super::Line;
use super::NamespaceId;
use super::PersonId;
use super::QuotedString;
use super::RelationId;
use super::SubsetId;
use super::SynonymScope;
use super::SynonymTypeId;
use super::UnquotedString;
use super::Xref;
use super::XrefList;

/// An instance frame, describing a particular individual.
pub struct InstanceFrame {
    id: Line<InstanceId>,
    clauses: Vec<Line<InstanceClause>>,
}

/// A clause appearing in an instance frame.
pub enum InstanceClause {
    IsAnonymous(bool),
    Name(UnquotedString),
    Namespace(NamespaceId),
    AltId(Id),
    Def(QuotedString, Vec<Xref>),
    Comment(UnquotedString),
    Subset(SubsetId),
    Synonym(QuotedString, SynonymScope, Option<SynonymTypeId>, XrefList),
    Xref(Id),
    PropertyValue(RelationId, Id),
    InstanceOf(ClassId),
    Relationship(RelationId, Id),
    CreatedBy(PersonId),
    CreationDate(IsoDate),
    IsObsolete(bool),
    ReplacedBy(InstanceId),
    Consider(Id),
}
