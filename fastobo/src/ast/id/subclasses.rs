use std::str::FromStr;

use pest::iterators::Pair;
use url::Url;

use crate::ast::*;
use crate::error::Result;
use crate::parser::FromPair;
use crate::parser::FromSlice;
use crate::parser::Rule;
use crate::share::Share;
use crate::share::Redeem;

// FIXME(@althonos): could probably be replaced with `opaque_typedef` macros.
macro_rules! ident_subclass {
    (#[doc = $docstring:literal] $rule:expr => pub struct $name:ident) => {
        #[derive(Clone, Debug, PartialEq, Hash, Eq)]
        #[doc=$docstring]
        pub struct $name {
            id: Ident,
        }

        impl $name {
            pub fn new(id: Ident) -> Self {
                Self { id }
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

        impl From<Ident> for $name {
            fn from(id: Ident) -> Self {
                $name::new(id)
            }
        }

        impl From<UnprefixedIdent> for $name {
            fn from(id: UnprefixedIdent) -> Self {
                $name::new(Ident::from(id))
            }
        }

        impl From<PrefixedIdent> for $name {
            fn from(id: PrefixedIdent) -> Self {
                $name::new(Ident::from(id))
            }
        }

        impl From<Url> for $name {
            fn from(id: Url) -> Self {
                $name::new(Ident::from(id))
            }
        }

        impl From<$name> for Ident {
            fn from(id: $name) -> Self {
                id.id
            }
        }

        impl<'i> crate::parser::FromPair<'i> for $name {
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
    (#[doc = $docstring:literal] $rule:expr => pub struct $name:ident : &$life:lifetime $owned:ident) => {
        #[doc=$docstring]
        pub struct $name<$life> {
            inner: Id<$life>
        }

        impl<$life> ::std::fmt::Display for $name<$life> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.inner.fmt(f)
            }
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

        impl<'i> FromPair<'i> for $name<'i> {
            const RULE: Rule = $rule;
            unsafe fn from_pair_unchecked(pair: Pair<'i, Rule>) -> Result<Self> {
                Id::from_pair_unchecked(pair.into_inner().next().unwrap())
                    .map(From::from)
            }
        }

        impl<'i> FromSlice<'i> for $name<'i> {
            type Err = $crate::error::Error;
            fn from_slice(s: &'i str) -> $crate::error::Result<Self> {
                Id::from_slice(s).map(From::from)
            }
        }

        impl<$life> Share<$life, $name<$life>> for $owned {
            fn share(&$life self) -> $name<$life> {
                $name::from(self.id.share())
            }
        }

        impl<$life> Redeem<$life> for $name<$life> {
            type Owned = $owned;
            fn redeem(&$life self) -> $owned {
                $owned::from(self.inner.redeem())
            }
        }
    }
}


macro_rules! id_subclasses {
    ($(#[doc = $docstring:literal] $rule:expr => pub struct $name:ident : &$life:lifetime $owned:ident;)*) => {
        $(id_subclass!(#[doc = $docstring] $rule => pub struct $name : &$life $owned);)*
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


id_subclasses! {
    /// A borrowed `ClassIdent`.
    Rule::ClassId => pub struct ClassId: &'a ClassIdent;

    /// A borrowed `InstanceIdent`
    Rule::InstanceId => pub struct InstanceId: &'a InstanceIdent;

    /// A borrowed `NamespaceIdent`.
    Rule::NamespaceId => pub struct NamespaceId: &'a NamespaceIdent;

    /// A borrowed `RelationIdent`.
    Rule::RelationId => pub struct RelationId: &'a RelationIdent;

    /// A borrowed `SubsetIdent`.
    Rule::SubsetId => pub struct SubsetId: &'a SubsetIdent;

    /// A borrowed `SynonymTypeIdent`.
    Rule::SynonymTypeId => pub struct SynonymTypeId: &'a SynonymTypeIdent;
}
