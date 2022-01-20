/// A macro for deriving `From` for the various clause enums.
#[doc(hidden)]
#[macro_export]
macro_rules! clause_impl_from {
    ($Clause:ident) => {
        impl From<Box<CreationDate>> for $Clause {
            fn from(date: Box<CreationDate>) -> Self {
                $Clause::CreationDate(date)
            }
        }

        impl From<Box<Definition>> for $Clause {
            fn from(d: Box<Definition>) -> Self {
                $Clause::Def(d)
            }
        }

        impl From<CreationDate> for $Clause {
            fn from(date: CreationDate) -> Self {
                $Clause::from(Box::new(date))
            }
        }

        impl From<Definition> for $Clause {
            fn from(d: Definition) -> Self {
                $Clause::from(Box::new(d))
            }
        }
    };
}
