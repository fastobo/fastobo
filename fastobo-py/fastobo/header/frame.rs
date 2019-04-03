use std::iter::FromIterator;

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

#[pyclass(weakref, gc)]
pub struct HeaderFrame {
    clauses: PyObject,
}

impl HeaderFrame {
    fn as_pylist(&self) -> &PyList {
        let gil = Python::acquire_gil();
        let py = gil.python();
        PyTryFrom::try_from(self.clauses.as_ref(py)).expect("always PyList")
    }
}

impl AsRef<PyList> for HeaderFrame {
    fn as_ref(&self) -> &PyList {
        self.as_pylist()
    }
}

impl AsPyPointer for HeaderFrame {
    fn as_ptr(&self) -> *mut pyo3::ffi::PyObject {
        self.clauses.as_ptr()
    }
}

impl Clone for HeaderFrame {
    fn clone(&self) -> Self {
        let gil = Python::acquire_gil();
        let py = gil.python();
        Self { clauses: self.clauses.clone_ref(py) }
    }
}

impl From<obo::HeaderFrame> for HeaderFrame {
    fn from(frame: fastobo::ast::HeaderFrame) -> Self {
        Self::from(frame.clauses)
    }
}

impl From<Vec<obo::HeaderClause>> for HeaderFrame {
    fn from(clauses: Vec<obo::HeaderClause>) -> Self {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let pylist = PyList::new(py, clauses.into_iter().map(HeaderClause::from));
        Self {
            clauses: pylist.to_object(py),
        }
    }
}

#[pyproto]
impl PyObjectProtocol for HeaderFrame {
    fn __repr__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let fmt = PyString::new(py, "HeaderFrame({!r})").to_object(py);
        fmt.call_method1(py, "format", (&self.clauses,))
    }
}

// FIXME(@althonos)
#[pyproto]
impl PySequenceProtocol for HeaderFrame {
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.as_pylist().len())
    }
    fn __getitem__(&self, index: isize) -> PyResult<PyObject> {
        let list = self.as_pylist();
        if index < list.len() as isize {
            Ok(list.get_item(index).into())
        } else {
            Err(IndexError::py_err("list index out of range"))
        }
    }
    fn __setitem__(&mut self, index: isize, elem: PyObject) -> PyResult<()> {
        let list = self.as_pylist();
        if index < list.len() as isize {
            if Python::acquire_gil().python().is_instance::<HeaderClause, PyObject>(&elem)? {
                list.set_item(index, elem)
            } else {
                Err(TypeError::py_err("expected HeaderClause"))
            }
        } else {
            Err(IndexError::py_err("list index out of range"))
        }
    }
    fn __delitem__(&mut self, index: isize) -> PyResult<()> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.clauses.call_method1(py, "__delitem__", (index, ))?;
        Ok(())
    }
}

#[pyproto]
impl PyGCProtocol for HeaderFrame {
    fn __traverse__(&self, visit: PyVisit) -> Result<(), PyTraverseError> {
        visit.call(&self.clauses)
    }
    fn __clear__(&mut self) {
        let gil = GILGuard::acquire();
        let py = gil.python();
        py.release(&self.clauses)
    }
}
