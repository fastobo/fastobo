use std::convert::From;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write;

use super::Id;

/// The identifier of a person, used in a `created_by` tag-value pair.
#[derive(Debug, PartialEq)]
pub struct PersonId(pub Id);

impl From<Id> for PersonId {
    fn from(id: Id) -> Self {
        PersonId(id)
    }
}

impl Display for PersonId {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
