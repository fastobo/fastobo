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

        impl $crate::obo14::parser::FromPair for $name {
            const RULE: Rule = <Id as FromPair>::RULE;
            unsafe fn from_pair_unchecked(pair: Pair<Rule>) -> Result<Self> {
                Id::from_pair_unchecked(pair).map(From::from)
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
