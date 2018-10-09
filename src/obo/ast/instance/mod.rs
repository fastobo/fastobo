//! Syntax nodes for `[Instance]` frame and clauses.

use chrono::offset::Utc;
use chrono::DateTime;

use super::DbXref;
use super::Id;
use super::InstanceId;
use super::OboNamespace;
use super::PersonId;
use super::Qualifier;
use super::RelationId;
use super::SynonymScope;

#[derive(Debug, PartialEq)]
pub struct InstanceFrame {
    pub clauses: Vec<InstanceClause>,
}

#[derive(Debug, PartialEq)]
pub struct InstanceClause {
    pub value: InstanceTagValue,
    pub qualifiers: Option<Vec<Qualifier>>,
    pub comment: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum InstanceTagValue {
    Id(InstanceId),
    IsAnonymous(bool),
    Name(String),
    Namespace(OboNamespace),
    AltId(InstanceId),
    Def(String, Vec<DbXref>),
    Comment(String),
    Subset(Id),
    Synonym(String, SynonymScope, Option<String>, Vec<DbXref>),
    Xref(DbXref),
    Relationship(RelationId, InstanceId),
    CreatedBy(PersonId),
    CreationDate(DateTime<Utc>),
    IsObsolete(bool),
    ReplacedBy(InstanceId),
    Consider(InstanceId),
}
