use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;

use fastobo_derive_internal::FromStr;
use pest::iterators::Pair;

use crate::ast::*;
use crate::error::SyntaxError;
use crate::parser::Cache;
use crate::parser::FromPair;
use crate::syntax::Rule;

/// A clause value binding a property to a value in the relevant entity.
#[derive(Clone, Debug, Hash, Eq, FromStr, PartialEq, Ord)]
pub enum PropertyValue {
    /// A property-value binding where the value is specified with an ID.
    Resource(Box<ResourcePropertyValue>),
    /// A property-value binding where the value is given by a typed literal.
    Literal(Box<LiteralPropertyValue>),
}

impl PropertyValue {
    /// Get the identifier of the declared property annotation.
    pub fn property(&self) -> &RelationIdent {
        use self::PropertyValue::*;
        match self {
            Resource(pv) => pv.property(),
            Literal(pv) => pv.property(),
        }
    }
}

impl Display for PropertyValue {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            PropertyValue::Resource(pv) => pv.fmt(f),
            PropertyValue::Literal(pv) => pv.fmt(f),
        }
    }
}

impl From<LiteralPropertyValue> for PropertyValue {
    fn from(pv: LiteralPropertyValue) -> Self {
        PropertyValue::Literal(Box::new(pv))
    }
}

impl From<ResourcePropertyValue> for PropertyValue {
    fn from(pv: ResourcePropertyValue) -> Self {
        PropertyValue::Resource(Box::new(pv))
    }
}

impl<'i> FromPair<'i> for PropertyValue {
    const RULE: Rule = Rule::PropertyValue;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        let inner = pair.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::LiteralPropertyValue => LiteralPropertyValue::from_pair_unchecked(inner, cache)
                .map(Box::new)
                .map(PropertyValue::Literal),
            Rule::ResourcePropertyValue => ResourcePropertyValue::from_pair_unchecked(inner, cache)
                .map(Box::new)
                .map(PropertyValue::Resource),
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for PropertyValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.property()
            .cmp(other.property())
            .then_with(|| self.to_string().cmp(&other.to_string()))
            .into()
    }
}

/// A property-value where the triple target is refered to with an ID.
///
/// This kind of property can be used to declare triples in the OBO document
/// where the property is an annotation property and the target is another
/// element in the semantic graph not necessarily defined as an OBO entity,
/// but for instance with an IDspace mapping:
/// ```rust
/// # extern crate fastobo;
/// # use fastobo::ast::*;
/// let property = RelationIdent::from(PrefixedIdent::new("dc", "creator"));
/// let target = Ident::from(PrefixedIdent::new("ORCID", "0000-0002-3947-4444"));
/// let property_value = ResourcePropertyValue::new(property, target);
/// ```
#[derive(Clone, Debug, Hash, FromStr, PartialOrd, Eq, PartialEq, Ord)]
pub struct ResourcePropertyValue {
    property: RelationIdent,
    target: Ident,
}

impl ResourcePropertyValue {
    pub fn new(property: RelationIdent, target: Ident) -> Self {
        Self { property, target }
    }

    /// Get the identifier of the declared property annotation.
    pub fn property(&self) -> &RelationIdent {
        &self.property
    }

    pub fn property_mut(&mut self) -> &mut RelationIdent {
        &mut self.property
    }

    pub fn target(&self) -> &Ident {
        &self.target
    }

    pub fn target_mut(&mut self) -> &mut Ident {
        &mut self.target
    }
}

impl Display for ResourcePropertyValue {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.property
            .fmt(f)
            .and(f.write_char(' '))
            .and(self.target.fmt(f))
    }
}

impl<'i> FromPair<'i> for ResourcePropertyValue {
    const RULE: Rule = Rule::ResourcePropertyValue;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();
        let relid = RelationIdent::from_pair_unchecked(inner.next().unwrap(), cache)?;
        let id = Ident::from_pair_unchecked(inner.next().unwrap(), cache)?;
        Ok(ResourcePropertyValue::new(relid, id))
    }
}

/// A property-value binding where the value is given by a typed literal.
///
/// This kind of property can be used to add additional annotations to an entity
/// where the annotation value is not an entity itself but a typed value such
/// as a string (of type `xsd:string`), a date (`xsd:date`), etc.
#[derive(Clone, Debug, Hash, FromStr, PartialOrd, Eq, PartialEq, Ord)]
pub struct LiteralPropertyValue {
    property: RelationIdent,
    literal: QuotedString,
    datatype: Ident,
}

impl LiteralPropertyValue {
    pub fn new(property: RelationIdent, literal: QuotedString, datatype: Ident) -> Self {
        Self {
            property,
            literal,
            datatype,
        }
    }

    /// Get the identifier of the declared property annotation.
    pub fn property(&self) -> &RelationIdent {
        &self.property
    }

    pub fn property_mut(&mut self) -> &mut RelationIdent {
        &mut self.property
    }

    pub fn literal(&self) -> &QuotedString {
        &self.literal
    }

    pub fn literal_mut(&mut self) -> &mut QuotedString {
        &mut self.literal
    }

    pub fn datatype(&self) -> &Ident {
        &self.datatype
    }

    pub fn datatype_mut(&mut self) -> &mut Ident {
        &mut self.datatype
    }
}

impl Display for LiteralPropertyValue {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.property
            .fmt(f)
            .and(f.write_char(' '))
            .and(self.literal.fmt(f))
            .and(f.write_char(' '))
            .and(self.datatype.fmt(f))
    }
}

impl<'i> FromPair<'i> for LiteralPropertyValue {
    const RULE: Rule = Rule::LiteralPropertyValue;
    unsafe fn from_pair_unchecked(
        pair: Pair<'i, Rule>,
        cache: &Cache,
    ) -> Result<Self, SyntaxError> {
        let mut inner = pair.into_inner();

        let relid = RelationIdent::from_pair_unchecked(inner.next().unwrap(), cache)?;
        let second = inner.next().unwrap();
        let datatype = Ident::from_pair_unchecked(inner.next().unwrap(), cache)?;
        let desc = match second.as_rule() {
            Rule::QuotedString => QuotedString::from_pair_unchecked(second, cache)?,
            Rule::UnquotedPropertyValueTarget => QuotedString::new(second.as_str().to_string()),
            _ => unreachable!(),
        };

        Ok(LiteralPropertyValue::new(relid, desc, datatype))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;
    use std::str::FromStr;

    #[test]
    fn from_str() {
        let actual = PropertyValue::from_str("married_to heather").unwrap();
        let expected = PropertyValue::from(ResourcePropertyValue::new(
            RelationIdent::from(UnprefixedIdent::new(String::from("married_to"))),
            Ident::from(UnprefixedIdent::new(String::from("heather"))),
        ));
        assert_eq!(actual, expected);

        let actual = PropertyValue::from_str("shoe_size \"8\" xsd:positiveInteger").unwrap();
        let expected = PropertyValue::from(LiteralPropertyValue::new(
            RelationIdent::from(UnprefixedIdent::new(String::from("shoe_size"))),
            QuotedString::new(String::from("8")),
            Ident::from(PrefixedIdent::new("xsd", "positiveInteger")),
        ));
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
