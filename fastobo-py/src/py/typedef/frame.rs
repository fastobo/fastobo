use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;

use pyo3::AsPyPointer;
use pyo3::PyNativeType;
use pyo3::PyObjectProtocol;
use pyo3::PyTypeInfo;
use pyo3::prelude::*;
use pyo3::exceptions::TypeError;
use pyo3::exceptions::ValueError;
use pyo3::types::PyAny;
use pyo3::types::PyString;

use fastobo::ast;
use fastobo::share::Share;
use fastobo::share::Cow;
use fastobo::share::Redeem;

use super::clause::TypedefClause;
use super::super::BaseEntityFrame;
use super::super::id::Ident;
use crate::utils::ClonePy;

#[pyclass(extends=BaseEntityFrame)]
#[derive(Debug)]
pub struct TypedefFrame {
    id: Ident,
    clauses: Vec<TypedefClause>,
}

impl TypedefFrame {
    pub fn new(id: Ident) -> Self {
        Self::with_clauses(id, Vec::new())
    }

    pub fn with_clauses(id: Ident, clauses: Vec<TypedefClause>) -> Self {
        Self { id, clauses }
    }
}

impl ClonePy for TypedefFrame {
    fn clone_py(&self, py: Python) -> Self {
        Self {
            id: self.id.clone_py(py),
            clauses: self.clauses.clone_py(py),
        }
    }
}

impl Display for TypedefFrame {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let gil = Python::acquire_gil();
        let py = gil.python();
        fastobo::ast::TypedefFrame::from_py(self.clone_py(py), py).fmt(f)
    }
}

impl FromPy<fastobo::ast::TypedefFrame> for TypedefFrame {
    fn from_py(frame: fastobo::ast::TypedefFrame, py: Python) -> Self {
        Self::with_clauses(
            Ident::from_py(frame.id.inner, py),
            frame
                .clauses
                .into_iter()
                .map(|line| TypedefClause::from_py(line.inner, py))
                .collect()
        )
    }
}

impl FromPy<TypedefFrame> for fastobo::ast::TypedefFrame {
    fn from_py(frame: TypedefFrame, py: Python) -> Self {
        fastobo::ast::TypedefFrame::with_clauses(
            fastobo::ast::RelationIdent::new(frame.id.into_py(py)),
            frame
                .clauses
                .iter()
                .map(|f| fastobo::ast::TypedefClause::from_py(f, py))
                .map(fastobo::ast::Line::from)
                .collect()
        )
    }
}

impl FromPy<TypedefFrame> for fastobo::ast::EntityFrame {
    fn from_py(frame: TypedefFrame, py: Python) -> Self {
        Self::from(fastobo::ast::TypedefFrame::from_py(frame, py))
    }
}

#[pymethods]
impl TypedefFrame {
    #[getter]
    fn get_id(&self) -> PyResult<&Ident> {
        Ok(&self.id)
    }
}

#[pyproto]
impl PyObjectProtocol for TypedefFrame {
    fn __repr__(&self) -> PyResult<PyObject> {
        let py = unsafe { Python::assume_gil_acquired() };
        PyString::new(py, "TypedefFrame({!r})")
            .to_object(py)
            .call_method1(py, "format", (&self.id,))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
}
