use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;

use iri_string::AbsoluteIriStr;
use iri_string::AbsoluteIriString;
use iri_string::RelativeIriString;
use pest::iterators::Pair;

use crate::ast::*;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Parser;
use crate::parser::Rule;

/// An Internationalized Resource Identifier, either absolute or relative.
#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Iri {
    Absolute(AbsoluteIriString),
    Relative(RelativeIriString),
}

impl From<AbsoluteIriString> for Iri {
    fn from(abs: AbsoluteIriString) -> Self {
        Iri::Absolute(abs)
    }
}

impl From<RelativeIriString> for Iri {
    fn from(rel: RelativeIriString) -> Self {
        Iri::Relative(rel)
    }
}

impl Display for Iri {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::Iri::*;
        match self {
            Absolute(iri) => iri.fmt(f),
            Relative(iri) => iri.fmt(f),
        }
    }
}

impl FromPair for Iri {
    const RULE: Rule = Rule::Iri;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        // FIXME(@althonos): proper iri_strng error handling.
        let iri = AbsoluteIriStr::new(pair.as_str()).expect("invalid IRI");
        Ok(Iri::Absolute(iri.to_owned()))
    }
}
impl_fromstr!(Iri);

/// A clause value binding a property to a value in the relevant entity.
#[derive(Debug, Hash, Eq, PartialEq)]
pub enum PropertyValue {
    Identified(RelationId, Id),
    // FIXME(@althonos): maybe replaced `String` with `DatatypeId` newtype.
    Typed(RelationId, QuotedString, Id),
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

impl FromPair for PropertyValue {
    const RULE: Rule = Rule::PropertyValue;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        let mut inner = pair.into_inner();
        let relid = RelationId::from_pair_unchecked(inner.next().unwrap())?;
        let second = inner.next().unwrap();
        match second.as_rule() {
            Rule::Id => {
                let id = Id::from_pair_unchecked(second)?;
                Ok(PropertyValue::Identified(relid, id))
            }
            Rule::QuotedString => {
                let desc = QuotedString::from_pair_unchecked(second)?;
                let datatype = Id::from_str(inner.next().unwrap().as_str())?;
                Ok(PropertyValue::Typed(relid, desc, datatype))
            }
            _ => unreachable!(),
        }
    }
}
impl_fromstr!(PropertyValue);

impl FromPair for bool {
    const RULE: Rule = Rule::Boolean;
    unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
        Ok(bool::from_str(pair.as_str()).expect("cannot fail."))
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
                RelationId::from(Id::Unprefixed(UnprefixedId::new("married_to"))),
                Id::Unprefixed(UnprefixedId::new("heather")),
            );
            assert_eq!(actual, expected);

            let actual = PropertyValue::from_str("shoe_size \"8\" xsd:positiveInteger").unwrap();
            let expected = PropertyValue::Typed(
                RelationId::from(Id::Unprefixed(UnprefixedId::new("shoe_size"))),
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
