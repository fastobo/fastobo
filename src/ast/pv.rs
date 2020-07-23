use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;

use pest::iterators::Pair;
use url::Url;

use crate::ast::*;
use crate::error::Error;
use crate::error::SyntaxError;
use crate::parser::FromPair;
use crate::syntax::Rule;

/// A clause value binding a property to a value in the relevant entity.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord)]
pub enum PropertyValue {
    /// A property-value binding where the value is specified with an ID.
    Resource(RelationIdent, Ident),
    /// A property-value binding where the value is given by a typed literal.
    Literal(RelationIdent, QuotedString, Ident),
}

impl PropertyValue {
    /// Get the identifier of the declared property annotation.
    pub fn property(&self) -> &RelationIdent {
        use self::PropertyValue::*;
        match self {
            Resource(ref prop, _) => prop,
            Literal(ref prop, _, _) => prop,
        }
    }
}

impl Display for PropertyValue {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            PropertyValue::Resource(relation, instance) => {
                relation.fmt(f).and(f.write_char(' ')).and(instance.fmt(f))
            }
            PropertyValue::Literal(relation, desc, datatype) => relation
                .fmt(f)
                .and(f.write_char(' '))
                .and(desc.fmt(f))
                .and(f.write_char(' '))
                .and(datatype.fmt(f)),
        }
    }
}

impl<'i> FromPair<'i> for PropertyValue {
    const RULE: Rule = Rule::PropertyValue;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();
        let relid = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
        let second = inner.next().unwrap();
        match second.as_rule() {
            Rule::Id => {
                let id = Ident::from_pair_unchecked(second)?;
                Ok(PropertyValue::Resource(relid, id))
            }
            Rule::PvValue => {
                let desc = QuotedString::new(second.as_str().to_string());
                let datatype = Ident::from_str(inner.next().unwrap().as_str())?;
                Ok(PropertyValue::Literal(relid, desc, datatype))
            }
            Rule::QuotedString => {
                let desc = QuotedString::from_pair_unchecked(second)?;
                let datatype = Ident::from_str(inner.next().unwrap().as_str())?;
                Ok(PropertyValue::Literal(relid, desc, datatype))
            }
            _ => unreachable!(),
        }
    }
}
impl_fromstr!(PropertyValue);

impl PartialOrd for PropertyValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.property()
            .cmp(&other.property())
            .then_with(|| self.to_string().cmp(&other.to_string()))
            .into()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn from_str() {
        let actual = PropertyValue::from_str("married_to heather").unwrap();
        let expected = PropertyValue::Resource(
            RelationIdent::from(Ident::Unprefixed(UnprefixedIdent::new(String::from(
                "married_to",
            )))),
            Ident::Unprefixed(UnprefixedIdent::new(String::from("heather"))),
        );
        assert_eq!(actual, expected);

        let actual = PropertyValue::from_str("shoe_size \"8\" xsd:positiveInteger").unwrap();
        let expected = PropertyValue::Literal(
            RelationIdent::from(Ident::Unprefixed(UnprefixedIdent::new(String::from(
                "shoe_size",
            )))),
            QuotedString::new(String::from("8")),
            Ident::from(PrefixedIdent::new(
                IdentPrefix::new(String::from("xsd")),
                IdentLocal::new(String::from("positiveInteger")),
            )),
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn partial_cmp() {
        let l1 = PropertyValue::from_str("engaged_to heather").unwrap();
        let r1 = PropertyValue::from_str("married_to heather").unwrap();
        assert!(l1 < r1);

        let l2 = PropertyValue::from_str("married_to ashley").unwrap();
        let r2 = PropertyValue::from_str("married_to heather").unwrap();
        assert!(l2 < r2);

        let l3 = PropertyValue::from_str("has_kids \"8\" xsd:positiveInteger").unwrap();
        let r3 = PropertyValue::from_str("married_to heather").unwrap();
        assert!(l3 < r3);

        let l4 = PropertyValue::from_str("has_kid \"true\" xsd:boolean").unwrap();
        let r4 = PropertyValue::from_str("has_kid jenny").unwrap();
        assert!(l4 < r4);
    }
}
