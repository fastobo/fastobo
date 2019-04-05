#![recursion_limit="128"]
#![allow(unused_imports)]

extern crate fastobo;
extern crate pyo3;
extern crate libc;
extern crate url;

#[macro_use]
extern crate opaque_typedef_macros;
extern crate opaque_typedef;

#[macro_use]
extern crate fastobo_py_derive;

use std::rc::Rc;
use std::str::FromStr;
use std::string::ToString;

use pyo3::prelude::*;
use pyo3::PyTypeInfo;
use pyo3::types::PyAny;
use pyo3::types::PyList;
use pyo3::types::PyString;
use pyo3::exceptions::RuntimeError;
use pyo3::exceptions::IndexError;
use pyo3::exceptions::TypeError;
use pyo3::exceptions::ValueError;
use pyo3::PySequenceProtocol;
use pyo3::PyGCProtocol;
use pyo3::PyObjectProtocol;
use pyo3::gc::PyTraverseError;
use pyo3::class::gc::PyVisit;

use fastobo::ast as obo;

// -------------------------------------------------------------------------

pub mod header;
pub mod id;
pub mod pv;

use self::header::frame::HeaderFrame;

// -------------------------------------------------------------------------

/// The abstract syntax tree corresponding to an OBO document.
#[pyclass(subclass)]
pub struct OboDoc {
    header: Py<HeaderFrame>,
}

impl Clone for OboDoc {
    fn clone(&self) -> Self {
        let gil = Python::acquire_gil();
        let py = gil.python();
        Self { header: self.header.clone_ref(py) }
    }
}

impl FromPy<obo::OboDoc> for OboDoc {
    fn from_py(doc: obo::OboDoc, py: Python) -> Self {
        let header = HeaderFrame::from(doc.header().clone());
        Self {
            header: Py::new(py, header)
                .expect("could not move header to Python heap")
        }
    }
}

#[pymethods]
impl OboDoc {
    #[getter]
    fn get_header(&self) -> PyResult<Py<HeaderFrame>> {
        let py = unsafe { Python::assume_gil_acquired() };
        Ok(self.header.clone_ref(py))
    }

    #[setter]
    fn set_header(&mut self, header: &HeaderFrame) -> PyResult<()> {
        let py = unsafe { Python::assume_gil_acquired() };
        self.header = Py::new(py, header.clone())?;
        Ok(())
    }
}

// -------------------------------------------------------------------------

#[pymodule]
fn fastobo(py: Python, m: &PyModule) -> PyResult<()> {

    {
        use self::header::*;
        m.add_wrapped(pyo3::wrap_pymodule!(header))?;
    }

    {
        use self::id::*;
        m.add_wrapped(pyo3::wrap_pymodule!(id))?;
    }


    #[pyfn(m, "load")]
    fn load(py: Python, fh: &PyAny) -> PyResult<OboDoc> {
        if let Ok(s) = fh.downcast_ref::<PyString>() {
            let path = s.to_string()?;
            match obo::OboDoc::from_file(path.as_ref()) {
                Ok(doc) => Ok(doc.into_py(py)),
                Err(e) => ValueError::into(format!("load failed: {}", e)),
            }
        } else {
            return pyo3::exceptions::NotImplementedError::into(
                "cannot only use load with a path right now"
            );
        }
    }


    #[pyfn(m, "loads")]
    fn loads(py: Python, s: &str) -> PyResult<OboDoc> {
        match fastobo::ast::OboDoc::from_str(s) {
            Ok(doc) => Ok(doc.into_py(py)),
            Err(e) => ValueError::into(format!("loads failed: {}", e)),
        }
    }

    m.add_class::<HeaderFrame>()?;
    m.add_class::<OboDoc>()?;
    Ok(())
}
