use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;

use pest::iterators::Pair;
use url::Url;

use crate::ast::*;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

/// A clause value binding a property to a value in the relevant entity.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum PropertyValue {
    Identified(RelationIdent, Ident),
    // FIXME(@althonos): maybe replaced `String` with `DatatypeId` newtype.
    Typed(RelationIdent, QuotedString, Ident),
}

impl Display for PropertyValue {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::PropertyValue::*;
        match self {
            Identified(relation, instance) => {
                relation.fmt(f).and(f.write_char(' ')).and(instance.fmt(f))
            }
            Typed(relation, desc, datatype) => relation
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
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let relid = RelationIdent::from_pair_unchecked(inner.next().unwrap())?;
        let second = inner.next().unwrap();
        match second.as_rule() {
            Rule::Id => {
                let id = Ident::from_pair_unchecked(second)?;
                Ok(PropertyValue::Identified(relid, id))
            }
            Rule::PvValue => {
                let desc = QuotedString::new(second.as_str().to_string());
                let datatype = Ident::from_str(inner.next().unwrap().as_str())?;
                Ok(PropertyValue::Typed(relid, desc, datatype))
            }
            Rule::QuotedString => {
                let desc = QuotedString::from_pair_unchecked(second)?;
                let datatype = Ident::from_str(inner.next().unwrap().as_str())?;
                Ok(PropertyValue::Typed(relid, desc, datatype))
            }
            _ => unreachable!(),
        }
    }
}
impl_fromstr!(PropertyValue);

impl<'i> FromPair<'i> for bool {
    const RULE: Rule = Rule::Boolean;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        Ok(bool::from_str(pair.as_str()).expect("cannot fail."))
    }
}

impl<'i> FromPair<'i> for Url {
    const RULE: Rule = Rule::Iri;
    unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        Ok(Url::parse(pair.as_str()).unwrap()) // FIXME
    }
}

#[cfg(test)]
mod tests {

    use super::super::IdLocal;
    use super::super::IdPrefix;
    use super::super::PrefixedId;
    use super::super::UnprefixedId;
    use super::*;

    mod property_value {

        use super::*;

        #[test]
        fn from_str() {
            let actual = PropertyValue::from_str("married_to heather").unwrap();
            let expected = PropertyValue::Identified(
                RelationIdent::from(Id::Unprefixed(UnprefixedId::new("married_to"))),
                Id::Unprefixed(UnprefixedId::new("heather")),
            );
            assert_eq!(actual, expected);

            let actual = PropertyValue::from_str("shoe_size \"8\" xsd:positiveInteger").unwrap();
            let expected = PropertyValue::Typed(
                RelationIdent::from(Id::Unprefixed(UnprefixedId::new("shoe_size"))),
                QuotedString::new("8"),
                Id::Prefixed(PrefixedId::new(
                    IdPrefix::new("xsd"),
                    IdLocal::new("positiveInteger"),
                )),
            );
            assert_eq!(actual, expected);
        }

    }

}
