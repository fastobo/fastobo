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

use super::super::BaseEntityFrame;
use super::clause::TermClause;
use super::super::id::Ident;

#[pyclass(extends=BaseEntityFrame)]
#[derive(Clone, Debug)]
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
