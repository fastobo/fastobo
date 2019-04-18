use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;
use std::string::ToString;

use pest::error::ErrorVariant;
use pest::error::Error as PestError;
use pest::iterators::Pair;
use url::Url;

use crate::ast::*;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

impl<'i> FromPair<'i> for bool {
    const RULE: Rule = Rule::Boolean;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        Ok(bool::from_str(pair.as_str()).expect("cannot fail."))
    }
}

impl<'i> FromPair<'i> for Url {
    const RULE: Rule = Rule::Iri;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        Url::parse(pair.as_str()).map_err(|e| {
            PestError::new_from_span(
                ErrorVariant::CustomError { message: e.to_string() },
                pair.as_span(),
            ).into()
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::parser::OboParser;

    #[test]
    fn from_pair() {
        let mut pairs = OboParser::parse(Rule::UnquotedString, "http://not an url");
        let pair = pairs.unwrap().next().unwrap();
        unsafe {
            assert!(Url::from_pair_unchecked(pair).is_err())
        }
    }
}
