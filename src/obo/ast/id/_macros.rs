macro_rules! id_subclass {
    (#[doc = $docstring:literal] $name:ident) => {
        #[derive(Debug, PartialEq)]
        #[doc=$docstring]
        pub struct $name(pub Id);

        impl ::std::convert::From<Id> for $name {
            fn from(id: Id) -> Self {
                $name(id)
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $crate::errors::ParseError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Id::from_str(s).map(Self::from)
            }
        }
    };
}
