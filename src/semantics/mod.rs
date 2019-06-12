//! A selection of useful traits that exceed the scope of OBO syntax.
#![cfg_attr(feature = "_doc", doc(cfg(feature = "semantics")))]

use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::ptr::NonNull;

use crate::ast::*;
use crate::error::CardinalityError;
use crate::visit::VisitMut;

mod cardinality;
mod treat_xrefs;

pub use cardinality::Cardinality;
pub use cardinality::CardinalityBound;

pub(crate) use self::treat_xrefs::*;
