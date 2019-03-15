
// FIXME(@althonos): could probably be replaced with `opaque_typedef` macros.
macro_rules! id_subclass {
    (#[doc = $docstring:literal] pub struct $name:ident;) => {
        // #[derive(Debug, PartialEq)]
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

        // impl ::std::fmt::Display for $name {
        //     fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        //         self.0.fmt(f)
        //     }
        // }

        // impl ::std::str::FromStr for $name {
        //     type Err = $crate::errors::ParseError;
        //     fn from_str(s: &str) -> Result<Self, Self::Err> {
        //         Id::from_str(s).map(Self::from)
        //     }
        // }
    };
}


macro_rules! id_subclasses {
    ($(#[doc = $docstring:literal] pub struct $name:ident;)*) => {
        $(id_subclass!(#[doc = $docstring] pub struct $name;);)*
    }
}
