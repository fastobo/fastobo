//! OBO `nom` parser implementation.

#[macro_use]
mod _macros;

pub mod chars;
pub mod common;
pub mod header;
pub mod id;
pub mod spacing;
pub mod values;

use super::ast;
