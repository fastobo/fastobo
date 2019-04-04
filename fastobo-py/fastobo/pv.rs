use std::str::FromStr;

use pyo3::PyObjectProtocol;
use pyo3::PyTypeInfo;
use pyo3::prelude::*;
use pyo3::exceptions::TypeError;
use pyo3::exceptions::ValueError;
use pyo3::types::PyAny;
use pyo3::types::PyString;

use fastobo::ast;

use crate::id::Ident;

// --- Module export ---------------------------------------------------------

#[pymodule(pv)]
fn module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<self::BasePropertyValue>()?;
    m.add_class::<self::TypedPropertyValue>()?;
    m.add_class::<self::IdentifiedPropertyValue>()?;
    Ok(())
}

// --- Conversion Wrapper ----------------------------------------------------

#[derive(Clone, Debug, Eq, Hash, PartialEq, OpaqueTypedef)]
#[opaque_typedef(derive(FromInner, IntoInner))]
pub struct PropertyValue(ast::PropertyValue);

// --- Base ------------------------------------------------------------------

#[pyclass(subclass)]
pub struct BasePropertyValue {}

// --- Typed -----------------------------------------------------------------

#[pyclass(extends=BasePropertyValue)]
pub struct TypedPropertyValue {
    relation: Ident,
    value: ast::QuotedString,
    datatype: Ident,
}

// --- Identified ------------------------------------------------------------

#[pyclass(extends=BasePropertyValue)]
pub struct IdentifiedPropertyValue {
    relation: Ident,
    value: Ident,
}
