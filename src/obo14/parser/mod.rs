use pest::Parser;

#[derive(Debug, Parser)]
#[grammar = "obo14/parser/grammar.pest"]
pub struct Obo14Parser;
