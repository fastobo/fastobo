//! Syntax nodes for `[Term]` frame and clauses.

use chrono::offset::Utc;
use chrono::DateTime;

use super::ClassId;
use super::DbXref;
use super::Id;
use super::OboNamespace;
use super::PersonId;
use super::Qualifier;
use super::RelationId;
use super::SynonymScope;

/// A term frame, starting with `[Term]`.
#[derive(Debug, PartialEq)]
pub struct TermFrame {
    /// Additional term clauses within the frame.
    pub clauses: Vec<TermClause>,
}

#[derive(Debug, PartialEq)]
pub struct TermClause {
    pub value: TermTagValue,
    pub qualifiers: Option<Vec<Qualifier>>,
    pub comment: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum TermTagValue {
    /// The term identifier.
    Id(ClassId),

    /// The term name.
    Name(String),

    ///
    Namespace(OboNamespace),

    /// Whether or not the current object has an anonymous id.
    IsAnonymous(bool),

    /// An alternate id for this term.
    AltId(ClassId),

    /// The definition of the current term.
    Def(String, Vec<DbXref>),

    /// A comment for this term.
    Comment(String),

    /// The subset to which this term belongs.
    Subset(Id),

    /// A synonym for this term.
    Synonym(String, SynonymScope, Option<Id>, Vec<DbXref>),

    /// A cross-reference to an analogous term in another vocabulary.
    Xref(DbXref),

    /// A subclassing relationship between this term and another.
    IsA(ClassId),

    /// Indicate the term is equivalent to the intersection of other terms.
    IntersectionOf(Option<RelationId>, ClassId),

    /// Indicate the term is equivalent to the union of other terms.
    UnionOf(ClassId),

    EquivalentTo(ClassId),

    /// Indicate the term is disjoint from another.
    DisjointFrom(ClassId),

    // FIXME: modifiers ?
    /// Describe a typed relationship between this term and another.
    Relationship(RelationId, ClassId),

    /// Whether or not the term is obsolete.
    IsObsolete(bool),

    /// Gives a term which replaces an obsolete term.
    ReplacedBy(ClassId),

    /// Gives a term which may be an appropriate subsistute for an obsolete term.
    Consider(ClassId),

    /// Whether or not this term is built in to the OBO format.
    Builtin(bool),

    /// Name of the creator of the term.
    CreatedBy(PersonId),

    /// Date of the creation of the term.
    CreationDate(DateTime<Utc>),
}
