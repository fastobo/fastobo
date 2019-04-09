use std::str::FromStr;

use pyo3::PyObjectProtocol;
use pyo3::PyTypeInfo;
use pyo3::PyNativeType;
use pyo3::prelude::*;
use pyo3::exceptions::TypeError;
use pyo3::exceptions::ValueError;
use pyo3::types::PyAny;
use pyo3::types::PyString;

use fastobo::ast;
use fastobo::borrow::Cow;
use fastobo::borrow::Borrow;

use crate::id::Ident;
use crate::_utils::AsGILRef;

// --- Module export ---------------------------------------------------------

#[pymodule(pv)]
fn module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<self::BasePropertyValue>()?;
    m.add_class::<self::TypedPropertyValue>()?;
    m.add_class::<self::IdentifiedPropertyValue>()?;
    Ok(())
}

// --- Conversion Wrapper ----------------------------------------------------

#[derive(Debug, PartialEq, PyWrapper)]
#[wraps(BasePropertyValue)]
pub enum PropertyValue {
    Typed(Py<TypedPropertyValue>),
    Identified(Py<IdentifiedPropertyValue>),
}

impl FromPy<fastobo::ast::PropertyValue> for PropertyValue {
    fn from_py(pv: fastobo::ast::PropertyValue, py: Python) -> Self {
        match pv {
            fastobo::ast::PropertyValue::Typed(r, d, ty) =>
                Py::new(py, TypedPropertyValue::new(py, r, d, ty))
                    .map(PropertyValue::Typed),
            fastobo::ast::PropertyValue::Identified(r, v) =>
                Py::new(py, IdentifiedPropertyValue::new(py, r, v))
                    .map(PropertyValue::Identified),
        }.expect("could not allocate on Python heap")
    }
}

impl FromPy<PropertyValue> for fastobo::ast::PropertyValue {
    fn from_py(pv: PropertyValue, py: Python) -> Self {
        match pv {
            PropertyValue::Typed(t) =>
                Self::from_py(t.as_ref(py).clone(), py),
            PropertyValue::Identified(i) =>
                Self::from_py(i.as_ref(py).clone(), py)
        }
    }
}

// --- Base ------------------------------------------------------------------

#[pyclass(subclass)]
#[derive(Debug)]
pub struct BasePropertyValue {}

// --- Typed -----------------------------------------------------------------

#[pyclass(extends=BasePropertyValue)]
#[derive(Clone, Debug)]
pub struct TypedPropertyValue {
    relation: Ident,
    value: ast::QuotedString,
    datatype: Ident,
}

impl TypedPropertyValue {
    pub fn new<R, V, D>(py: Python, relation: R, value: V, datatype: D) -> Self
    where
        R: IntoPy<Ident>,
        V: Into<fastobo::ast::QuotedString>,
        D: IntoPy<Ident>
    {
        TypedPropertyValue {
            relation: relation.into_py(py),
            value: value.into(),
            datatype: datatype.into_py(py),
        }
    }
}

impl<'p> AsGILRef<'p, fastobo::ast::PropVal<'p>> for TypedPropertyValue  {
    fn as_ref(&'p self, py: Python<'p>) -> fastobo::ast::PropVal<'p> {
        fastobo::ast::PropVal::Typed(
            Cow::Borrowed(self.relation.as_ref(py).into()),
            Cow::Borrowed(self.value.borrow()),
            Cow::Borrowed(self.datatype.as_ref(py))
        )
    }
}

impl FromPy<TypedPropertyValue> for fastobo::ast::PropertyValue {
    fn from_py(pv: TypedPropertyValue, py: Python) -> Self {
        fastobo::ast::PropertyValue::Typed(
            pv.relation.into_py(py),
            pv.value,
            pv.datatype.into_py(py),
        )
    }
}

#[pymethods]
impl TypedPropertyValue {
    #[new]
    fn __init__(obj: &PyRawObject, relation: &PyAny , value: &PyAny, datatype: &PyAny) -> PyResult<()> {
        let r = relation.extract::<Ident>()?;
        let v = if let Ok(s) = value.extract::<&PyString>() {
            ast::QuotedString::new(s.to_string()?.to_string())
        } else {
            let n = value.get_type().name();
            return TypeError::into(format!("expected str for value, found {}", n))
        };
        let dt = datatype.extract::<Ident>()?;
        Ok(obj.init(Self::new(obj.py(), r, v, dt)))
    }

    #[getter]
    fn get_relation(&self) -> PyResult<&Ident> {
        Ok(&self.relation)
    }

    #[setter]
    fn set_relation(&mut self, relation: Ident) -> PyResult<()> {
        self.relation = relation;
        Ok(())
    }

    #[getter]
    fn get_value(&self) -> PyResult<&str> {
        Ok(self.value.as_str())
    }

    #[setter]
    fn set_value(&mut self, value: String) -> PyResult<()> {
        self.value = fastobo::ast::QuotedString::new(value);
        Ok(())
    }

    #[getter]
    fn get_datatype(&self) -> PyResult<&Ident> {
        Ok(&self.datatype)
    }

    #[setter]
    fn set_datatype(&mut self, datatype: Ident) -> PyResult<()> {
        self.datatype = datatype;
        Ok(())
    }
}

#[pyproto]
impl PyObjectProtocol for TypedPropertyValue {

    fn __repr__(&self) -> PyResult<PyObject> {
        let py = unsafe { Python::assume_gil_acquired() };
        let fmt = PyString::new(py, "TypedPropertyValue({!r}, {!r}, {!r})").to_object(py);
        fmt.call_method1(py, "format", (
            self.relation.to_object(py),
            self.value.as_str(),
            self.datatype.to_object(py))
        )
    }

    fn __str__(&self) -> PyResult<String> {
        let py = unsafe { Python::assume_gil_acquired() };
        Ok(self.as_ref(py).to_string())
    }
}

// --- Identified ------------------------------------------------------------

#[pyclass(extends=BasePropertyValue)]
#[derive(Clone, Debug)]
pub struct IdentifiedPropertyValue {
    relation: Ident,
    value: Ident,
}

impl IdentifiedPropertyValue {
    pub fn new<R, V>(py: Python, relation: R, value: V) -> Self
    where
        R: IntoPy<Ident>,
        V: IntoPy<Ident>,
    {
        IdentifiedPropertyValue {
            relation: relation.into_py(py),
            value: value.into_py(py)
        }
    }
}

impl FromPy<IdentifiedPropertyValue> for fastobo::ast::PropertyValue {
    fn from_py(pv: IdentifiedPropertyValue, py: Python) -> Self {
        fastobo::ast::PropertyValue::Identified(
            pv.relation.into_py(py),
            pv.value.into_py(py),
        )
    }
}
