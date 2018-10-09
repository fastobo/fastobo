use std::convert::From;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write;

use super::Id;

/// The identifier of an instance.
#[derive(Debug, PartialEq)]
pub struct InstanceId(pub Id);

impl From<Id> for InstanceId {
    fn from(id: Id) -> Self {
        InstanceId(id)
    }
}

impl Display for InstanceId {
    fn fmt(&self, f: &mut Formatter) -> ::std::fmt::Result {
        self.0.fmt(f)
    }
}
