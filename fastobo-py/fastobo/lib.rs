#![allow(unused_imports)]

extern crate fastobo;
extern crate pyo3;
extern crate libc;
extern crate url;

#[macro_use]
extern crate opaque_typedef_macros;
extern crate opaque_typedef;

use std::rc::Rc;
use std::str::FromStr;
use std::string::ToString;

use pyo3::prelude::*;
use pyo3::PyTypeInfo;
use pyo3::types::PyAny;
use pyo3::types::PyList;
use pyo3::exceptions::RuntimeError;
use pyo3::exceptions::IndexError;
use pyo3::exceptions::TypeError;
use pyo3::PySequenceProtocol;
use pyo3::PyGCProtocol;
use pyo3::PyObjectProtocol;
use pyo3::gc::PyTraverseError;
use pyo3::class::gc::PyVisit;

use fastobo::ast as obo;

// -------------------------------------------------------------------------

pub mod header;
pub mod id;

use self::header::HeaderFrame;

// -------------------------------------------------------------------------

/// The abstract syntax tree corresponding to an OBO document.
#[pyclass(subclass, weakref)]
#[derive(Clone)]
pub struct OboDoc {
    #[pyo3(get)]
    header: HeaderFrame,
}

impl From<obo::OboDoc> for OboDoc {
    fn from(doc: obo::OboDoc) -> Self {
        Self {
            header: HeaderFrame::from(doc.header().clone())
        }
    }
}

// -------------------------------------------------------------------------








// -------------------------------------------------------------------------



/// This module is implemented in Rust.
#[pymodule]
fn fastobo(py: Python, m: &PyModule) -> PyResult<()> {


    header::module(py, m);
    id::module(py, m);


    // // Note that the `#[pyfn()]` annotation automatically converts the arguments from
    // // Python objects to Rust values; and the Rust return value back into a Python object.
    #[pyfn(m, "loads")]
    fn loads(py: Python, s: pyo3::types::PyString) -> PyResult<OboDoc> {

        let s = unsafe {
            std::str::from_utf8_unchecked(s.as_bytes())
        };

        obo::OboDoc::from_str(s)
            .map(From::from)
            .map_err(|e| RuntimeError::py_err(format!("{}", e))) // FIXME: SyntaxError ?
    }

    m.add_class::<HeaderFrame>()?;
    m.add_class::<OboDoc>()?;
    Ok(())
}
