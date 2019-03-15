//! Syntax tree and parser for the [OBO format version 1.4].
//!
//! [OBO format version 1.4]: http://owlcollab.github.io/oboformat/doc/GO.format.obo-1_4.html.

pub mod ast;
pub mod parser;

pub use self::ast::*;
