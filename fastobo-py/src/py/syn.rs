
use std::rc::Rc;
use std::str::FromStr;
use std::string::ToString;

use pyo3::prelude::*;
use pyo3::PyTypeInfo;
use pyo3::PyNativeType;
use pyo3::AsPyPointer;
use pyo3::types::PyAny;
use pyo3::types::PyList;
use pyo3::types::PyString;
use pyo3::types::PyIterator;
use pyo3::exceptions::RuntimeError;
use pyo3::exceptions::IndexError;
use pyo3::exceptions::TypeError;
use pyo3::exceptions::ValueError;
use pyo3::PySequenceProtocol;
use pyo3::PyGCProtocol;
use pyo3::PyObjectProtocol;
use pyo3::gc::PyTraverseError;
use pyo3::class::gc::PyVisit;

use super::id::Ident;
use super::xref::XrefList;
use crate::utils::ClonePy;

// --- Module export ---------------------------------------------------------

#[pymodule(syn)]
fn module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<self::Synonym>()?;
    m.add_class::<self::SynonymScope>()?;
    Ok(())
}

// --- SynonymScope ----------------------------------------------------------

#[pyclass]
#[derive(Clone, ClonePy, Debug)]
pub struct SynonymScope {
    inner: fastobo::ast::SynonymScope
}

impl SynonymScope {
    pub fn new(scope: fastobo::ast::SynonymScope) -> Self {
        Self { inner: scope }
    }
}

// --

#[pyclass]
#[derive(Debug)]
pub struct Synonym {
    desc: fastobo::ast::QuotedString,
    scope: SynonymScope,
    ty: Option<Ident>,
    xrefs: XrefList,
}

impl ClonePy for Synonym {
    fn clone_py(&self, py: Python) -> Self {
        Self {
            desc: self.desc.clone(),
            scope: self.scope.clone_py(py),
            ty: self.ty.clone_py(py),
            xrefs: self.xrefs.clone_py(py),
        }
    }
}

impl FromPy<fastobo::ast::Synonym> for Synonym {
    fn from_py(syn: fastobo::ast::Synonym, py: Python) -> Self {
        Self {
            desc: syn.desc,
            scope: SynonymScope::new(syn.scope),
            ty: syn.ty.map(|id| id.into_py(py)),
            xrefs: syn.xrefs.into_py(py),
        }
    }
}

impl FromPy<Synonym> for fastobo::ast::Synonym {
    fn from_py(syn: Synonym, py: Python) -> Self {
        Self::with_type_and_xrefs(
            syn.desc,
            syn.scope.inner,
            syn.ty.map(|ty| ty.into_py(py)),
            syn.xrefs.into_py(py),
        )
    }
}