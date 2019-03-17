use super::Id;
use super::Line;
use super::NamespaceId;
use super::QuotedString;
use super::RelationId;
use super::SubsetId;
use super::SynonymScope;
use super::SynonymTypeId;
use super::UnquotedString;
use super::Xref;

/// A typedef clause, describing a relationship.
pub struct TypedefFrame {
    id: Line<RelationId>,
    clauses: Vec<Line<TypedefClause>>,
}

/// A clause appearing in a typedef frame.
pub enum TypedefClause {
    IsAnonymous(bool),
    Name(UnquotedString),
    Namespace(NamespaceId),
    AltId(Id),
    Def(QuotedString, Vec<Xref>),
    Comment(UnquotedString),
    Subset(SubsetId),
    Synonym(QuotedString, SynonymScope, Option<SynonymTypeId>, Vec<Xref>),
}
