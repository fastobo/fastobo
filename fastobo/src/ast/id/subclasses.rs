use std::str::FromStr;

use pest::iterators::Pair;

use crate::ast::*;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::Rule;

// FIXME(@althonos): could probably be replaced with `opaque_typedef` macros.
macro_rules! ident_subclass {
    (#[doc = $docstring:literal] $rule:expr => pub struct $name:ident) => {
        #[derive(Clone, Debug, PartialEq, Hash, Eq)]
        #[doc=$docstring]
        pub struct $name {
            id: Ident,
        }

        impl From<Ident> for $name {
            fn from(id: Ident) -> Self {
                $name { id }
            }
        }

        impl From<$name> for Ident {
            fn from(id: $name) -> Self {
                id.id
            }
        }

        impl AsRef<Ident> for $name {
            fn as_ref(&self) -> &Ident {
                &self.id
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.id.fmt(f)
            }
        }

        impl<'i> FromPair<'i> for $name {
            const RULE: Rule = $rule;
            unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
                Ident::from_pair_unchecked(pair.into_inner().next().unwrap()).map(From::from)
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $crate::error::Error;
            fn from_str(s: &str) -> Result<Self> {
                Ident::from_str(s).map(Self::from)
            }
        }
    };
}

macro_rules! ident_subclasses {
    ($(#[doc = $docstring:literal] $rule:expr => pub struct $name:ident;)*) => {
        $(ident_subclass!(#[doc = $docstring] $rule => pub struct $name);)*
    }
}


macro_rules! id_subclass {
    (#[doc = $docstring:literal] $rule:expr => pub struct $name:ident : &$life:lifetime $borrowed:ident) => {
        #[doc=$docstring]
        pub struct $name<$life> {
            inner: Id<$life>
        }

        impl<$life> From<Id<$life>> for $name<$life> {
            fn from(id: Id<$life>) -> Self {
                Self {
                    inner: id
                }
            }
        }

        impl<$life> From<$name<$life>> for Id<$life> {
            fn from(id: $name<$life>) -> Self {
                id.inner
            }
        }

        // TODO
        // impl<'i> FromPair<'i> for $name<'i> {
        //     const RULE: Rule = $rule;
        //     unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
        //         Id::from_pair_unchecked(pair.into_inner().next().unwrap())
        //             .map(From::from)
        //     }
        // }


        impl<$life> crate::borrow::Borrow<$life, $name<$life>> for $borrowed {
            fn borrow(&$life self) -> $name<$life> {
                $name::from(self.id.borrow())
            }
        }

    }
}








// NB(@althonos): All identifiers are defined as separate typedefs so that
//                `PartialEq` is not implemented and trying to compare a
//                `ClassId` with a `RelationId` would fail at compile-time.
ident_subclasses! {
    /// A unique identifier for a class (*i.e.* a term).
    Rule::ClassId => pub struct ClassIdent;

    /// A unique identifier for an instance.
    Rule::InstanceId => pub struct InstanceIdent;

    /// An OBO namespace identifier.
    Rule::NamespaceId => pub struct NamespaceIdent;

    /// A unique identifier for a typedef (*i.e.* a relation).
    Rule::RelationId => pub struct RelationIdent;

    /// A unique identifier for a subset
    Rule::SubsetId => pub struct SubsetIdent;

    /// A unique identifier for a synonym type.
    Rule::SynonymTypeId => pub struct SynonymTypeIdent;
}


id_subclass! {
    ///
    Rule::ClassId => pub struct ClassId: &'a ClassIdent
}
