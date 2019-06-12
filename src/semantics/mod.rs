//! A selection of useful traits that exceed the scope of OBO syntax.
#![cfg_attr(feature = "_doc", doc(cfg(feature = "semantics")))]

use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ptr::NonNull;

use crate::ast::*;
use crate::error::CardinalityError;
use crate::visit::VisitMut;

mod treat_xrefs;
pub(crate) use self::treat_xrefs::*;

/// The cardinality constraint for a given clause type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Cardinality {
    ZeroOrOne,
    One,
    NotOne,
    Any,
}

impl Cardinality {
    pub fn is_match(&self, n: usize) -> bool {
        match self {
            Cardinality::ZeroOrOne => n < 2,
            Cardinality::One => n == 1,
            Cardinality::NotOne => n != 1,
            Cardinality::Any => true,
        }
    }

    pub fn to_error<S: Into<String>>(&self, n: usize, tag: S) -> Option<CardinalityError> {
        use self::CardinalityError::*;
        let name = tag.into();
        match self {
            Cardinality::ZeroOrOne if n > 2 => Some(DuplicateClauses { name }),
            Cardinality::One if n == 0 => Some(MissingClause { name }),
            Cardinality::One if n > 1 => Some(DuplicateClauses { name }),
            Cardinality::NotOne if n == 1 => Some(SingleClause { name }),
            _ => None,
        }
    }
}

/// A trait for structs that can be sorted in an order specified in the OBO spec.
pub trait Orderable {
    /// Sort the elements of the collection in the right serialization order.
    ///
    /// # See Also
    /// - The [Serializer conventions](https://owlcollab.github.io/oboformat/doc/GO.format.obo-1_4.html#S.3.5)
    ///   section of the OBO Flat File format guide.
    fn sort(&mut self);

    /// Check if the collection is sorted in the right serialization order.
    fn is_sorted(&self) -> bool;
}
