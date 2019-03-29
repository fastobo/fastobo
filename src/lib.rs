//! Syntax tree and parser for the [OBO format version 1.4].
//!
//! [OBO format version 1.4]: http://owlcollab.github.io/oboformat/doc/GO.format.obo-1_4.html.
//!
#![allow(dead_code, unused_imports)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate pest_derive;
extern crate pest;
extern crate url;

#[macro_use]
pub mod parser;
pub mod ast;
pub mod error;
