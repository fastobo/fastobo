use std::iter::FromIterator;
use std::iter::IntoIterator;

use fastobo::ast as obo;
use pyo3::prelude::*;
use pyo3::PyTypeInfo;
use pyo3::types::PyAny;
use pyo3::types::PyList;
use pyo3::types::PyString;
use pyo3::exceptions::RuntimeError;
use pyo3::exceptions::IndexError;
use pyo3::exceptions::TypeError;
use pyo3::PySequenceProtocol;
use pyo3::PyGCProtocol;
use pyo3::PyObjectProtocol;
use pyo3::gc::PyTraverseError;
use pyo3::class::gc::PyVisit;
use pyo3::AsPyPointer;

use super::HeaderClause;
use super::BaseHeaderClause;

#[pyclass]
#[derive(Clone, Debug)]
pub struct HeaderFrame {
    clauses: Vec<HeaderClause>
}

impl HeaderFrame {
    pub fn new(clauses: Vec<HeaderClause>) -> Self {
        Self { clauses }
    }
}

impl FromIterator<HeaderClause> for HeaderFrame {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item=HeaderClause>
    {
        Self::new(iter.into_iter().collect())
    }
}

impl FromPy<obo::HeaderFrame> for HeaderFrame {
    fn from_py(frame: fastobo::ast::HeaderFrame, py: Python) -> Self {
        frame.clauses
            .into_iter()
            .map(|clause| HeaderClause::from_py(clause, py))
            .collect()
    }
}

impl FromPy<HeaderFrame> for obo::HeaderFrame {
    fn from_py(frame: HeaderFrame, py: Python) -> Self {
        frame.clauses
            .into_iter()
            .map(|clause| obo::HeaderClause::from_py(clause, py))
            .collect()
    }
}

impl ToPyObject for HeaderFrame {
    fn to_object(&self, py: Python) -> PyObject {
        PyList::new(py, &self.clauses).into_object(py)
    }
}

#[pyproto]
impl PyObjectProtocol for HeaderFrame {
    fn __repr__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let fmt = PyString::new(py, "HeaderFrame({!r})").to_object(py);
        fmt.call_method1(py, "format", (self.to_object(py),))
    }

    fn __str__(&self) -> PyResult<String> {
        let py = unsafe { Python::assume_gil_acquired() };
        let frame: obo::HeaderFrame = self
            .clauses
            .iter()
            .map(|c| FromPy::from_py(c, py))
            .collect();
        Ok(frame.to_string())
    }
}

// FIXME(@althonos)
#[pyproto]
impl PySequenceProtocol for HeaderFrame {
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.clauses.len())
    }
    fn __getitem__(&self, index: isize) -> PyResult<PyObject> {

        let py = unsafe {
            Python::assume_gil_acquired()
        };

        if index < self.clauses.len() as isize {
            let item = &self.clauses[index as usize];
            Ok(item.to_object(py))
        } else {
            IndexError::into("list index out of range")
        }
    }
    fn __setitem__(&mut self, index: isize, elem: &PyAny) -> PyResult<()> {
        if index as usize > self.clauses.len() {
            return IndexError::into("list index out of range");
        }
        let clause = HeaderClause::extract(elem)?;
        self.clauses[index as usize] = clause;
        Ok(())
    }
    fn __delitem__(&mut self, index: isize) -> PyResult<()> {
        if index as usize > self.clauses.len() {
            return IndexError::into("list index out of range");
        }
        self.clauses.remove(index as usize);
        Ok(())
    }
}
