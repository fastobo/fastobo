use std::str::FromStr;

use pest::iterators::Pair;

use crate::ast::*;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

// FIXME(@althonos): could probably be replaced with `opaque_typedef` macros.
macro_rules! id_subclass {
    (#[doc = $docstring:literal] pub struct $name:ident;) => {
        #[derive(Debug, PartialEq, Hash, Eq)]
        #[doc=$docstring]
        pub struct $name {
            id: Id,
        }

        impl From<Id> for $name {
            fn from(id: Id) -> Self {
                $name { id }
            }
        }

        impl From<$name> for Id {
            fn from(id: $name) -> Self {
                id.id
            }
        }

        impl AsRef<Id> for $name {
            fn as_ref(&self) -> &Id {
                &self.id
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.id.fmt(f)
            }
        }

        impl FromPair for $name {
            const RULE: Rule = Rule::$name;
            unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
                Id::from_pair_unchecked(pair.into_inner().next().unwrap()).map(From::from)
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $crate::error::Error;
            fn from_str(s: &str) -> Result<Self> {
                Id::from_str(s).map(Self::from)
            }
        }
    };
}

macro_rules! id_subclasses {
    ($(#[doc = $docstring:literal] pub struct $name:ident;)*) => {
        $(id_subclass!(#[doc = $docstring] pub struct $name;);)*
    }
}

// NB(@althonos): All identifiers are defined as separate typedefs so that
//                `PartialEq` is not implemented and trying to compare a
//                `ClassId` with a `RelationId` would fail at compile-time.
id_subclasses! {
    /// A unique identifier for a class (*i.e.* a term).
    pub struct ClassId;

    /// A unique identifier for an instance.
    pub struct InstanceId;

    /// An OBO namespace identifier.
    pub struct NamespaceId;

    /// A unique identifier for a typedef (*i.e.* a relation).
    pub struct RelationId;

    /// A unique identifier for a subset
    pub struct SubsetId;

    /// A unique identifier for a synonym type.
    pub struct SynonymTypeId;
}
