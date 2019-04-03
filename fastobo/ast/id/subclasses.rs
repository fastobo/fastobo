use std::str::FromStr;

use pest::iterators::Pair;

use crate::ast::*;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

// FIXME(@althonos): could probably be replaced with `opaque_typedef` macros.
macro_rules! identifier_subclass {
    (#[doc = $docstring:literal] pub struct $name:ident) => {
        #[derive(Clone, Debug, PartialEq, Hash, Eq)]
        #[doc=$docstring]
        pub struct $name {
            id: Identifier,
        }

        impl From<Identifier> for $name {
            fn from(id: Identifier) -> Self {
                $name { id }
            }
        }

        impl From<$name> for Identifier {
            fn from(id: $name) -> Self {
                id.id
            }
        }

        impl AsRef<Identifier> for $name {
            fn as_ref(&self) -> &Identifier {
                &self.id
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.id.fmt(f)
            }
        }

        impl<'i> FromPair<'i> for $name {
            const RULE: Rule = Rule::$name;
            unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
                Identifier::from_pair_unchecked(pair.into_inner().next().unwrap()).map(From::from)
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $crate::error::Error;
            fn from_str(s: &str) -> Result<Self> {
                Identifier::from_str(s).map(Self::from)
            }
        }
    };
}

macro_rules! identifier_subclasses {
    ($(#[doc = $docstring:literal] pub struct $name:ident;)*) => {
        $(identifier_subclass!(#[doc = $docstring] pub struct $name);)*
    }
}


// macro_rules! identifier_borrow {
//     (#[doc = $docstring:literal] pub struct $name:ident : &$life:lifetime $borrowed) => {
//         #[doc=$docstring]
//         pub struct $name<$life> {
//             inner: $crate::borrow::Cow<$life, &$life $borrowed>
//         }
//     }
// }








// NB(@althonos): All identifiers are defined as separate typedefs so that
//                `PartialEq` is not implemented and trying to compare a
//                `ClassId` with a `RelationId` would fail at compile-time.
identifier_subclasses! {
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


// identifier_borrow! {
//     ///
//     pub struct ClassIdentifier<'a>: &'a
// }
