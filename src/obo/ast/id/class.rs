use std::convert::From;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write;

use super::Id;

/// The identifier of a term.
#[derive(Debug, PartialEq)]
pub struct ClassId(pub Id);

impl From<Id> for ClassId {
    fn from(id: Id) -> Self {
        ClassId(id)
    }
}

impl Display for ClassId {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
