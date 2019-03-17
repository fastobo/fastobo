//! Syntax tree and parser for the [OBO format version 1.4].
//!
//! [OBO format version 1.4]: http://owlcollab.github.io/oboformat/doc/GO.format.obo-1_4.html.

#[macro_use]
pub mod parser;
pub mod ast;

pub use self::ast::*;
