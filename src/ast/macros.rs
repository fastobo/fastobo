/// A macro for deriving `From` for the various clause enums.
#[doc(hidden)]
#[macro_export]
macro_rules! clause_impl_from {
    ($Clause:ident) => {
        impl From<Box<$crate::ast::CreationDate>> for $Clause {
            fn from(date: Box<$crate::ast::CreationDate>) -> Self {
                $Clause::CreationDate(date)
            }
        }

        impl From<Box<$crate::ast::Definition>> for $Clause {
            fn from(d: Box<$crate::ast::Definition>) -> Self {
                $Clause::Def(d)
            }
        }

        impl From<Box<$crate::ast::LiteralPropertyValue>> for $Clause {
            fn from(pv: Box<$crate::ast::LiteralPropertyValue>) -> Self {
                $Clause::from($crate::ast::PropertyValue::Literal(pv))
            }
        }

        impl From<Box<$crate::ast::PropertyValue>> for $Clause {
            fn from(pv: Box<PropertyValue>) -> Self {
                $Clause::PropertyValue(pv)
            }
        }

        impl From<Box<$crate::ast::ResourcePropertyValue>> for $Clause {
            fn from(pv: Box<$crate::ast::ResourcePropertyValue>) -> Self {
                $Clause::from($crate::ast::PropertyValue::Resource(pv))
            }
        }

        impl From<Box<$crate::ast::Synonym>> for $Clause {
            fn from(s: Box<$crate::ast::Synonym>) -> Self {
                $Clause::Synonym(s)
            }
        }

        impl From<Box<$crate::ast::Xref>> for $Clause {
            fn from(x: Box<$crate::ast::Xref>) -> Self {
                $Clause::Xref(x)
            }
        }

        impl From<$crate::ast::CreationDate> for $Clause {
            fn from(date: $crate::ast::CreationDate) -> Self {
                $Clause::from(Box::new(date))
            }
        }

        impl From<$crate::ast::Definition> for $Clause {
            fn from(d: $crate::ast::Definition) -> Self {
                $Clause::from(Box::new(d))
            }
        }

        impl From<$crate::ast::LiteralPropertyValue> for $Clause {
            fn from(pv: $crate::ast::LiteralPropertyValue) -> Self {
                $Clause::from(Box::new(pv))
            }
        }

        impl From<$crate::ast::PropertyValue> for $Clause {
            fn from(pv: $crate::ast::PropertyValue) -> Self {
                $Clause::from(Box::new(pv))
            }
        }

        impl From<$crate::ast::ResourcePropertyValue> for $Clause {
            fn from(pv: $crate::ast::ResourcePropertyValue) -> Self {
                $Clause::from(Box::new(pv))
            }
        }

        impl From<$crate::ast::Synonym> for $Clause {
            fn from(s: $crate::ast::Synonym) -> Self {
                $Clause::from(Box::new(s))
            }
        }

        impl From<$crate::ast::Xref> for $Clause {
            fn from(x: $crate::ast::Xref) -> Self {
                $Clause::from(Box::new(x))
            }
        }
    };
}
