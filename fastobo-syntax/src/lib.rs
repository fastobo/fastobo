#[macro_use]
extern crate pest_derive;
extern crate pest;

/// The OBO format version 1.4 parser.
#[derive(Debug, Parser)]
#[grammar = "grammar.pest"]
pub struct OboParser;
