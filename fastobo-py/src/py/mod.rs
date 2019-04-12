use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::rc::Rc;
use std::str::FromStr;
use std::string::ToString;

use pyo3::prelude::*;
use pyo3::PyTypeInfo;
use pyo3::PyNativeType;
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

use crate::utils::AsGILRef;
use crate::utils::ClonePy;
use crate::pyfile::PyFile;

// ---------------------------------------------------------------------------

pub mod header;
pub mod id;
pub mod term;
pub mod typedef;
pub mod pv;
pub mod xref;

use self::header::frame::HeaderFrame;
use self::term::frame::TermFrame;
use self::typedef::frame::TypedefFrame;

// --- Module export ---------------------------------------------------------

#[pymodule]
fn fastobo(py: Python, m: &PyModule) -> PyResult<()> {

    m.add_class::<BaseEntityFrame>()?;
    m.add_class::<OboDoc>()?;

    {
        use self::header::*;
        m.add_wrapped(pyo3::wrap_pymodule!(header))?;
    }

    {
        use self::id::*;
        m.add_wrapped(pyo3::wrap_pymodule!(id))?;
    }

    {
        use self::term::*;
        m.add_wrapped(pyo3::wrap_pymodule!(term))?;
    }

    {
        use self::typedef::*;
        m.add_wrapped(pyo3::wrap_pymodule!(typedef));
    }

    {
        use self::pv::*;
        m.add_wrapped(pyo3::wrap_pymodule!(pv))?;
    }

    {
        use self::xref::*;
        m.add_wrapped(pyo3::wrap_pymodule!(xref))?;
    }

    #[pyfn(m, "load")]
    fn load(py: Python, fh: &PyAny) -> PyResult<OboDoc> {
        if let Ok(s) = fh.downcast_ref::<PyString>() {
            let path = s.to_string()?;
            match obo::OboDoc::from_file(path.as_ref()) {
                Ok(doc) => Ok(doc.into_py(py)),
                Err(e) => ValueError::into(format!("load failed: {}", e)),
            }

        } else if let Ok(f) = PyFile::from_object(fh.py(), fh) {
            let mut bufreader = std::io::BufReader::new(f);
            match obo::OboDoc::from_stream(&mut bufreader) {
                Ok(doc) => Ok(doc.into_py(py)),
                Err(e) => ValueError::into(format!("load failed: {}", e)),
            }
        } else {
            pyo3::exceptions::NotImplementedError::into(
                "cannot only use load with a path right now"
            )
        }
    }

    #[pyfn(m, "loads")]
    fn loads(py: Python, s: &str) -> PyResult<OboDoc> {
        match fastobo::ast::OboDoc::from_str(s) {
            Ok(doc) => Ok(doc.into_py(py)),
            Err(e) => ValueError::into(format!("loads failed: {}", e)),
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------

/// The abstract syntax tree corresponding to an OBO document.
#[pyclass(subclass)]
#[derive(Debug)]
pub struct OboDoc {
    header: Py<HeaderFrame>,
    entities: Vec<EntityFrame>
}

impl ClonePy for OboDoc {
    fn clone_py(&self, py: Python) -> Self {
        Self {
            header: self.header.clone_py(py),
            entities: self.entities.clone_py(py)
        }
    }
}

impl Display for OboDoc {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let gil = Python::acquire_gil();
        let py = gil.python();
        fastobo::ast::OboDoc::from_py(self.clone_py(py), py).fmt(f)
    }
}

impl FromPy<obo::OboDoc> for OboDoc {
    fn from_py(doc: fastobo::ast::OboDoc, py: Python) -> Self {
        let header = HeaderFrame::from_py(doc.header, py);
        Self {
            header: Py::new(py, header)
                .expect("could not move header to Python heap"),
            entities: doc.entities
                .into_iter()
                .map(|frame| EntityFrame::from_py(frame, py))
                .collect(),
        }
    }
}

impl FromPy<OboDoc> for fastobo::ast::OboDoc {
    fn from_py(doc: OboDoc, py: Python) -> Self {
        let header: HeaderFrame = doc.header.as_ref(py).clone_py(py);
        Self::with_entities(
            fastobo::ast::HeaderFrame::from_py(header, py),
            doc.entities.iter().map(|frame| frame.into_py(py))
        )
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
        self.header = Py::new(py, header.clone_py(py))?;
        Ok(())
    }
}

#[pyproto]
impl PyObjectProtocol for OboDoc {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
}

#[pyproto]
impl PySequenceProtocol for OboDoc {
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.entities.len())
    }

    fn __getitem__(&self, index: isize) -> PyResult<PyObject> {
        let py = unsafe { Python::assume_gil_acquired() };
        if index < self.entities.len() as isize {
            let item = &self.entities[index as usize];
            Ok(item.to_object(py))
        } else {
            IndexError::into("list index out of range")
        }
    }
}

// ---------------------------------------------------------------------------



// --- Conversion Wrapper ----------------------------------------------------

#[derive(ClonePy, Debug, PartialEq, PyWrapper)]
#[wraps(BaseEntityFrame)]
pub enum EntityFrame {
    Term(Py<TermFrame>),
    Typedef(Py<TypedefFrame>),
}

impl FromPy<fastobo::ast::EntityFrame> for EntityFrame {
    fn from_py(frame: fastobo::ast::EntityFrame, py: Python) -> Self {
        match frame {
            fastobo::ast::EntityFrame::Term(frame) =>
                Py::new(py, TermFrame::from_py(frame, py))
                    .map(EntityFrame::Term),
            fastobo::ast::EntityFrame::Typedef(frame) =>
                Py::new(py, TypedefFrame::from_py(frame, py))
                    .map(EntityFrame::Typedef),
            _ => unimplemented!(),
        }.expect("could not allocate on Python heap")
    }
}

// ---

#[pyclass(subclass)]
pub struct BaseEntityFrame {}
