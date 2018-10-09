use std::convert::From;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write;

use super::Id;

/// The identifier of a typedef.
#[derive(Debug, PartialEq)]
pub struct RelationId(pub Id);

impl From<Id> for RelationId {
    fn from(id: Id) -> Self {
        RelationId(id)
    }
}

impl Display for RelationId {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
