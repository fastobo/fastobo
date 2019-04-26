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
    /// Parse an input string using the given production rule.
    ///
    /// This is basically a specialized version of [`pest::Parser::parse`]
    /// that only accepts [`Rule`], and does not need the `Parser` trait to
    /// be in scope.
    ///
    /// [`Rule`]: ./enum.Rule.html
    /// [`pest::Parser::parse`]: https://docs.rs/pest/latest/pest/trait.Parser.html
    pub fn parse(rule: Rule, input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
        <Self as pest::Parser<Rule>>::parse(rule, input)
    }
}
