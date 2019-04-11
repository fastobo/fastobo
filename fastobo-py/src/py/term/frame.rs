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

use super::clause::TermClause;
use super::super::BaseEntityFrame;
use super::super::id::Ident;
use crate::utils::ClonePy;

#[pyclass(extends=BaseEntityFrame)]
#[derive(Debug)]
pub struct TermFrame {
    id: Ident,
    clauses: Vec<TermClause>
}

impl TermFrame {
    pub fn new(id: Ident) -> Self {
        Self::with_clauses(id, Vec::new())
    }

    pub fn with_clauses(id: Ident, clauses: Vec<TermClause>) -> Self {
        Self { id, clauses }
    }
}

impl ClonePy for TermFrame {
    fn clone_py(&self, py: Python) -> Self {
        Self {
            id: self.id.clone_py(py),
            clauses: self.clauses.clone_py(py),
        }
    }
}

impl Display for TermFrame {
    // FIXME: no clone
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let gil = Python::acquire_gil();
        let py = gil.python();
        fastobo::ast::TermFrame::from_py(self.clone_py(py), py).fmt(f)
    }
}

impl FromPy<fastobo::ast::TermFrame> for TermFrame {
    fn from_py(frame: fastobo::ast::TermFrame, py: Python) -> Self {
        let clauses = frame
            .clauses
            .into_iter()
            .map(|line| TermClause::from_py(line.inner, py))
            .collect();
        Self::with_clauses(Ident::from_py(frame.id.inner, py), clauses)
    }
}

impl FromPy<TermFrame> for fastobo::ast::TermFrame {
    fn from_py(frame: TermFrame, py: Python) -> Self {
        let id = fastobo::ast::Ident::from_py(frame.id, py);
        fastobo::ast::TermFrame::new(fastobo::ast::ClassIdent::from(id))
    }
}

impl FromPy<TermFrame> for fastobo::ast::EntityFrame {
    fn from_py(frame: TermFrame, py: Python) -> Self {
        fastobo::ast::TermFrame::from_py(frame, py).into()
    }
}

#[pymethods]
impl TermFrame {
    #[new]
    fn __init__(obj: &PyRawObject, id: Ident, clauses: Option<Vec<TermClause>>) -> PyResult<()> {
        Ok(obj.init(Self::with_clauses(id, clauses.unwrap_or_else(Vec::new))))
    }
}

#[pyproto]
impl PyObjectProtocol for TermFrame {
    fn __repr__(&self) -> PyResult<PyObject> {
        let py = unsafe { Python::assume_gil_acquired() };
        PyString::new(py, "TermFrame({!r})")
            .to_object(py)
            .call_method1(py, "format", (&self.id,))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
}
