#[macro_use]
extern crate pest_derive;
extern crate pest;

use pest::error::Error;
use pest::iterators::Pairs;

/// The OBO format version 1.4 parser.
#[derive(Debug, Parser)]
#[grammar = "grammar.pest"]
pub struct OboParser;

impl OboParser {
    pub fn parse(rule: Rule, input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
        <Self as pest::Parser<Rule>>::parse(rule, input)
    }
}
