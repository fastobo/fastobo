//! Syntax nodes for `[Typedef]` frame and clauses.

use chrono::offset::Utc;
use chrono::DateTime;

use super::DbXref;
use super::Id;
use super::OboNamespace;
use super::PersonId;
use super::Qualifier;
use super::RelationId;
use super::SynonymScope;

#[derive(Debug, PartialEq)]
pub struct TypedefFrame {
    /// Optional typedef clauses within the frame.
    pub clauses: Vec<TypedefClause>,
}

#[derive(Debug, PartialEq)]
pub struct TypedefClause {
    pub value: TypedefTagValue,
    pub qualifiers: Option<Vec<Qualifier>>,
    pub comment: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum TypedefTagValue {
    /// The typedef ID.
    Id(RelationId),

    /// The term name.
    Name(String),

    Namespace(OboNamespace),

    /// Whether or not the current object has an anonymous id.
    IsAnonymous(bool),

    /// An alternate id for this typedef.
    AltId(RelationId),

    /// The definition of the current typedef.
    Def(String, Vec<DbXref>),

    /// A comment for this typedef.
    Comment(String),

    /// The subset to which this typedef belongs.
    Subset(Id),

    /// A synonym for this typedef.
    Synonym(String, SynonymScope, Option<String>, Vec<DbXref>),

    /// A cross-reference to an analogous typedef in another vocabulary.
    Xref(DbXref),

    IsAntiSymetric(bool),

    IsCyclic(bool),

    IsReflexive(bool),

    IsTransitive(bool),

    IsFunctional(bool),

    IsInverseFunctional(bool),

    /// A subclassing relationship between this typedef and another.
    IsA(RelationId),

    /// Indicate the typedef is equivalent to the intersection of other typedefs.
    IntersectionOf(RelationId),

    /// Indicate the typedef is equivalent to the union of other typedefs.
    UnionOf(RelationId),

    EquivalentTo(RelationId),

    /// Indicate the typedef is disjoint from another.
    DisjointFrom(RelationId),

    // FIXME: modifiers ?
    /// Describe a typed relationship between this typedef and another.
    Relationship(RelationId, RelationId),

    /// Whether or not the typedef is obsolete.
    IsObsolete(bool),

    /// Gives a typedef which replaces an obsolete typedef.
    ReplacedBy(RelationId),

    /// Gives a typedef which may be an appropriate subsistute for an obsolete typedef.
    Consider(RelationId),

    /// Whether or not this typedef is built in to the OBO format.
    Builtin(bool),

    /// Name of the creator of the typedef.
    CreatedBy(PersonId),

    /// Date of the creation of the typedef.
    CreationDate(DateTime<Utc>),

    ExpandAssertionTo(String, Vec<DbXref>),

    ExpandExpressionTo(String, Vec<DbXref>),

    IsMetadata(bool),

    IsClassLevel(bool),
}
