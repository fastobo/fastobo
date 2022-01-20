use pest::iterators::Pair;

use crate::ast::*;
use crate::error::SyntaxError;
use crate::parser::Cache;
use crate::parser::FromPair;
use crate::syntax::Rule;

macro_rules! ident_subclass {
    (#[doc = $docstring:literal] $rule:expr => pub struct $name:ident) => {
        #[derive(Clone, Debug, PartialEq, Hash, Eq, PartialOrd, Ord)]
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

        impl AsMut<Ident> for $name {
            fn as_mut(&mut self) -> &mut Ident {
                &mut self.id
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

        impl<'i> FromPair<'i> for $name {
            const RULE: Rule = $rule;
            unsafe fn from_pair_unchecked(
                pair: Pair<'i, Rule>,
                cache: &Cache,
            ) -> Result<Self, SyntaxError> {
                Ident::from_pair_unchecked(pair.into_inner().next().unwrap(), cache).map(From::from)
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $crate::error::SyntaxError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
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
