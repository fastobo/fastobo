use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;

use pest::iterators::Pair;
use url::Url;

use crate::ast::*;
use crate::borrow::Cow;
use crate::borrow::ToOwned;
use crate::borrow::Borrow;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;


/// A clause value binding a property to a value in the relevant entity.
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum PropertyValue {
    Identified(RelationIdent, Ident),
    Typed(RelationIdent, QuotedString, Ident),
}

impl<'a> Borrow<'a, PropVal<'a>> for PropertyValue {
    fn borrow(&'a self) -> PropVal<'a> {
        match self {
            PropertyValue::Identified(p, v) => PropVal::Identified(
                Cow::Borrowed(p.borrow()),
                Cow::Borrowed(v.borrow()),
            ),
            PropertyValue::Typed(p, v, t) => PropVal::Typed(
                Cow::Borrowed(p.borrow()),
                Cow::Borrowed(v.borrow()),
                Cow::Borrowed(t.borrow()),
            )
        }
    }
}

impl Display for PropertyValue {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.borrow().fmt(f)
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

/// A borrowed `PropertyValue`.
pub enum PropVal<'a> {
    Identified(Cow<'a, RelationId<'a>>, Cow<'a, Id<'a>>),
    Typed(Cow<'a, RelationId<'a>>, Cow<'a, &'a QuotedStr>, Cow<'a, Id<'a>>)
}

impl<'a> ToOwned<'a> for PropVal<'a> {
    type Owned = PropertyValue;
    fn to_owned(&'a self) -> PropertyValue {
        match self {
            PropVal::Identified(p, v) => PropertyValue::Identified(
                p.to_owned(),
                <Cow<Id> as crate::borrow::ToOwned>::to_owned(v)
            ),
            PropVal::Typed(p, v, t) => PropertyValue::Typed(
                p.to_owned(),
                <Cow<&QuotedStr> as crate::borrow::ToOwned>::to_owned(v),
                t.to_owned(),
            )
        }
    }
}

impl<'a> Display for PropVal<'a> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::PropVal::*;
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


#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn from_str() {
        let actual = PropertyValue::from_str("married_to heather").unwrap();
        let expected = PropertyValue::Identified(
            RelationIdent::from(Ident::Unprefixed(UnprefixedIdent::new(String::from("married_to")))),
            Ident::Unprefixed(UnprefixedIdent::new(String::from("heather"))),
        );
        assert_eq!(actual, expected);

        let actual = PropertyValue::from_str("shoe_size \"8\" xsd:positiveInteger").unwrap();
        let expected = PropertyValue::Typed(
            RelationIdent::from(Ident::Unprefixed(UnprefixedIdent::new(String::from("shoe_size")))),
            QuotedString::new(String::from("8")),
            Ident::from(PrefixedIdent::new(
                IdentPrefix::new(String::from("xsd")),
                IdentLocal::new(String::from("positiveInteger")),
            )),
        );
        assert_eq!(actual, expected);
    }
}
