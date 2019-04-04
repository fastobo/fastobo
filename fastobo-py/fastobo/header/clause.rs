use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::str::FromStr;
use std::string::ToString;

use fastobo::ast as obo;
use fastobo::ast::UnquotedString;
use fastobo::ast::QuotedString;
use fastobo::borrow::Cow;
use fastobo::borrow::Borrow;
use fastobo::borrow::ToOwned;

use pyo3::prelude::*;
use pyo3::PyTypeInfo;
use pyo3::PyNativeType;
use pyo3::types::PyTimeAccess;
use pyo3::types::PyDateAccess;
use pyo3::types::PyAny;
use pyo3::types::PyList;
use pyo3::types::PyDateTime;
use pyo3::types::PyString;
use pyo3::exceptions::RuntimeError;
use pyo3::exceptions::IndexError;
use pyo3::exceptions::TypeError;
use pyo3::PySequenceProtocol;
use pyo3::PyGCProtocol;
use pyo3::PyObjectProtocol;
use pyo3::gc::PyTraverseError;
use pyo3::class::gc::PyVisit;
use pyo3::type_object::PyTypeCreate;

use crate::id::Ident;
use crate::id::BaseIdent;

// --- Base ------------------------------------------------------------------

#[pyclass(subclass)]
pub struct HeaderClause {
    inner: obo::HeaderClause
}

impl From<obo::HeaderClause> for HeaderClause {
    fn from(clause: obo::HeaderClause) -> Self {
        Self { inner: clause }
    }
}

impl ToPyObject for HeaderClause {
    fn to_object(&self, py: Python) -> PyObject {
        use obo::HeaderClause::*;
        match self.inner.clone() {
            FormatVersion(v) => {
                FormatVersionClause::new(v).into_object(py)
            }
            DataVersion(v) => {
                DataVersionClause::new(v).into_object(py)
            }
            Date(dt) => {
                DateClause::new(&dt).into_object(py)
            }
            SavedBy(name) => {
                SavedByClause::new(name).into_object(py)
            }
            AutoGeneratedBy(name) => {
                AutoGeneratedByClause::new(name).into_object(py)
            }
            Import(i) => {
                ImportClause::new(i.to_string()).into_object(py)
            }
            Subsetdef(s, q) => {
                SubsetdefClause::new(s, q).into_object(py)
            }
            SynonymTypedef(ty, desc, scope) => {
                match scope {
                    None => SynonymTypedefClause::new(
                        ty.to_string(),
                        desc.as_str().to_string()
                    ),
                    Some(s) => SynonymTypedefClause::with_scope(
                        ty.to_string(),
                        desc.as_str().to_string(),
                        s.to_string(),
                    ),
                }.into_object(py)
            }
            DefaultNamespace(ns) => {
                DefaultNamespaceClause::new(ns.to_string()).into_object(py)
            }
            // _ => unimplemented!("HeaderClause.to_object()")
            _ => py.NotImplemented(),
        }
    }
}

// --- FormatVersion ---------------------------------------------------------

#[pyclass(extends=HeaderClause)]
#[derive(Clone)]
pub struct FormatVersionClause {
    version: obo::UnquotedString,
}

impl FormatVersionClause {
    pub fn new(version: obo::UnquotedString) -> Self {
        Self { version }
    }
}

impl Display for FormatVersionClause {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        obo::HeaderClause::from(self.clone())
            .fmt(f)
    }
}

impl From<FormatVersionClause> for obo::HeaderClause {
    fn from(clause: FormatVersionClause) -> obo::HeaderClause {
        <obo::HeaderClauseRef as ToOwned>::to_owned(&clause.to_ref())
    }
}

impl FormatVersionClause {
    fn to_ref<'s>(&'s self) -> obo::HeaderClauseRef<'s> {
        let s: &'s str = self.version.as_ref();
        obo::HeaderClauseRef::FormatVersion(Cow::Borrowed(obo::UnquotedStr::new(s)))
    }
}

#[pymethods]
impl FormatVersionClause {
    #[new]
    fn __init__(obj: &PyRawObject, version: String) {
        obj.init(Self::new(fastobo::ast::UnquotedString::new(version)));
    }

    #[getter]
    fn get_version(&self) -> PyResult<&str> {
        Ok(self.version.as_str())
    }

    #[setter]
    fn set_version(&mut self, version: String) -> PyResult<()> {
        self.version = obo::UnquotedString::new(version);
        Ok(())
    }
}

#[pyproto]
impl PyObjectProtocol for FormatVersionClause {
    fn __repr__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let fmt = PyString::new(py, "FormatVersionClause({!r})").to_object(py);
        fmt.call_method1(py, "format", (self.version.as_str(),))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
}

// --- DataVersion -----------------------------------------------------------

#[pyclass(extends=HeaderClause)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DataVersionClause {
    version: UnquotedString
}

impl DataVersionClause {
    pub fn new(version: UnquotedString) -> Self {
        Self {version}
    }
}

impl From<DataVersionClause> for obo::HeaderClause {
    fn from(clause: DataVersionClause) -> obo::HeaderClause {
        obo::HeaderClause::DataVersion(clause.version)
    }
}

impl Display for DataVersionClause {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        obo::HeaderClause::DataVersion(self.version.clone())
            .fmt(f)
    }
}

#[pymethods]
impl DataVersionClause {
    #[new]
    fn __init__(obj: &PyRawObject, version: String) {
        obj.init(Self::new(UnquotedString::new(version)));
    }

    #[getter]
    fn get_version(&self) -> PyResult<&str> {
        Ok(self.version.as_str())
    }

    #[setter]
    fn set_version(&mut self, version: String) -> PyResult<()> {
        self.version = UnquotedString::new(version);
        Ok(())
    }
}

#[pyproto]
impl PyObjectProtocol for DataVersionClause {
    fn __repr__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let fmt = PyString::new(py, "DataVersionClause({!r})").to_object(py);
        fmt.call_method1(py, "format", (self.version.as_str(),))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
}

// --- Date ------------------------------------------------------------------

#[pyclass(extends=HeaderClause)]
#[derive(Debug)]
pub struct DateClause {
    #[pyo3(get, set)]
    date: Py<PyDateTime>,
}

impl Clone for DateClause {
    fn clone(&self) -> Self {
        Self {
            date: self.date.clone()
        }
    }
}

impl DateClause {
    pub fn new(dt: &obo::NaiveDateTime) -> Self {
        let gil = Python::acquire_gil();
        let pydt = PyDateTime::new(
            gil.python(),
            dt.year() as i32,
            dt.month(),
            dt.day(),
            dt.hour(),
            dt.minute(),
            0, 0, None).expect("cannot fail ?");
        Self { date: pydt }
    }
}

impl From<DateClause> for obo::HeaderClause {
    fn from(clause: DateClause) -> obo::HeaderClause {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let dt = obo::NaiveDateTime::new(
            clause.date.as_ref(py).get_day(),
            clause.date.as_ref(py).get_month(),
            clause.date.as_ref(py).get_year() as u16,
            clause.date.as_ref(py).get_hour(),
            clause.date.as_ref(py).get_minute(),
        );
        obo::HeaderClause::Date(dt)
    }
}

impl Display for DateClause {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let clause: obo::HeaderClause = self.clone().into();
        clause.fmt(f)
    }
}

#[pymethods]
impl DateClause {
    #[new]
    fn __init__(obj: &PyRawObject, date: PyDateTime) {

        obj.init(Self { date: Py::new(obj.py(), date).unwrap() });
    }
}

#[pyproto]
impl PyObjectProtocol for DateClause {
    fn __repr__(&self) -> PyResult<String> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        Ok(format!("DateClause({})", self.date.as_ref(py).repr()?))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
}

// --- SavedBy ---------------------------------------------------------------

#[pyclass(extends=HeaderClause)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SavedByClause {
    name: UnquotedString
}

impl SavedByClause {
    pub fn new(name: UnquotedString) -> Self  {
        Self {name}
    }
}

impl From<SavedByClause> for obo::HeaderClause {
    fn from(clause: SavedByClause) -> obo::HeaderClause {
        obo::HeaderClause::SavedBy(clause.name)
    }
}

impl Display for SavedByClause {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        obo::HeaderClause::from(self.clone()).fmt(f)
    }
}

#[pymethods]
impl SavedByClause {
    #[new]
    fn __init__(obj: &PyRawObject, version: String) {
        obj.init(Self::new(UnquotedString::new(version)));
    }

    #[getter]
    fn get_name(&self) -> PyResult<&str> {
        Ok(self.name.as_str())
    }

    #[setter]
    fn set_name(&mut self, name: String) -> PyResult<()> {
        self.name = UnquotedString::new(name);
        Ok(())
    }
}

#[pyproto]
impl PyObjectProtocol for SavedByClause {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
}

// --- AutoGeneratedBy -------------------------------------------------------

#[pyclass(extends=HeaderClause)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AutoGeneratedByClause {
    name: UnquotedString
}

impl AutoGeneratedByClause {
    pub fn new(name: UnquotedString) -> Self {
        Self { name }
    }
}

impl From<AutoGeneratedByClause> for obo::HeaderClause {
    fn from(clause: AutoGeneratedByClause) -> obo::HeaderClause {
        obo::HeaderClause::AutoGeneratedBy(clause.name)
    }
}

impl Display for AutoGeneratedByClause {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        obo::HeaderClause::from(self.clone()).fmt(f)
    }
}

#[pymethods]
impl AutoGeneratedByClause {
    #[new]
    fn __init__(obj: &PyRawObject, version: String) {
        obj.init(Self::new(UnquotedString::new(version)));
    }

    #[getter]
    fn get_name(&self) -> PyResult<&str> {
        Ok(self.name.as_ref())
    }

    #[setter]
    fn set_name(&mut self, name: String) -> PyResult<()> {
        self.name = UnquotedString::new(name);
        Ok(())
    }
}

// --- Import ----------------------------------------------------------------

#[pyclass(extends=HeaderClause)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ImportClause {
    #[pyo3(get, set)]
    reference: String, // should be `Import`
}

impl ImportClause {
    pub fn new(reference: String) -> Self {
        Self { reference }
    }
}

impl From<ImportClause> for obo::HeaderClause {
    fn from(clause: ImportClause) -> Self {
        obo::HeaderClause::Import(
            obo::Import::from_str(&clause.reference).unwrap()
        )
    }
}

#[pymethods]
impl ImportClause {
    #[new]
    fn __init__(obj: &PyRawObject, reference: String) {
        obj.init(Self::new(reference));
    }
}

// --- Subsetdef -------------------------------------------------------------

#[pyclass(extends=HeaderClause)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SubsetdefClause {
    subset: Ident,
    description: QuotedString,
}

impl SubsetdefClause {
    pub fn new<I>(subset: I, description: QuotedString) -> Self
    where
        I: Into<Ident>
    {
        Self {
            subset: subset.into(),
            description
        }
    }
}

impl From<SubsetdefClause> for obo::HeaderClause {
    fn from(clause: SubsetdefClause) -> Self {
        obo::HeaderClause::Subsetdef(clause.subset.into(), clause.description)
    }
}

impl Display for SubsetdefClause {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        obo::HeaderClause::from(self.clone()).fmt(f)
    }
}

#[pymethods]
impl SubsetdefClause {
    #[new]
    fn __init__(obj: &PyRawObject, subset: &PyAny, description: String) -> PyResult<()> {
        let py = obj.py();
        let ident = if py.is_instance::<BaseIdent, PyAny>(subset)? {
            Ident::extract(subset)?
        } else if py.is_instance::<PyString, PyAny>(subset)? {
            let s: PyString = FromPyObject::extract(subset)?;
            Ident::from_str(&s.to_string()?)?
        } else {
            return TypeError::into("expected str or Ident for 'subset'");
        };
        obj.init(Self::new(ident, QuotedString::new(description)));
        Ok(())
    }
}

#[pyproto]
impl PyObjectProtocol for SubsetdefClause {
    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
}

// --- SynonymTypedef --------------------------------------------------------

#[pyclass(extends=HeaderClause)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct SynonymTypedefClause {
    #[pyo3(get, set)]
    typedef: String,        // should be `SynonymTypeId`
    #[pyo3(get, set)]
    description: String,
    #[pyo3(get, set)]
    scope: Option<String>,  // should be `Option<SynonymScope>`
}

impl SynonymTypedefClause {
    pub fn new(typedef: String, description: String) -> Self {
        Self { typedef, description, scope: None }
    }

    pub fn with_scope(typedef: String, description: String, scope: String) -> Self {
        Self { typedef, description, scope: Some(scope) }
    }
}

impl From<SynonymTypedefClause> for obo::HeaderClause {
    fn from(clause: SynonymTypedefClause) -> Self {
        obo::HeaderClause::SynonymTypedef(
            obo::SynonymTypeId::from_str(&clause.typedef).unwrap(),
            obo::QuotedString::new(clause.description),
            clause.scope.map(|s| obo::SynonymScope::from_str(&s).unwrap()),
        )
    }
}

#[pymethods]
impl SynonymTypedefClause {
    #[new]
    // #[args(scope=None)]
    fn __init__(obj: &PyRawObject, typedef: String, description: String, scope: Option<String>) {
        obj.init(Self { typedef, description, scope });
    }
}

// --- DefaultNamespace ------------------------------------------------------

#[pyclass(extends=HeaderClause)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DefaultNamespaceClause {
    #[pyo3(get, set)]
    namespace: String,    // should be `NamespaceId`
}

impl DefaultNamespaceClause {
    pub fn new(namespace: String) -> Self {
        Self { namespace }
    }
}

impl From<DefaultNamespaceClause> for obo::HeaderClause {
    fn from(clause: DefaultNamespaceClause) -> Self {
        obo::HeaderClause::DefaultNamespace(
            obo::NamespaceId::from_str(&clause.namespace).unwrap()
        )
    }
}

#[pymethods]
impl DefaultNamespaceClause {
    #[new]
    fn __init__(obj: &PyRawObject, namespace: String) {
        obj.init(Self { namespace })
    }
}

// --- IdspaceClause ---------------------------------------------------------

#[pyclass(extends=HeaderClause)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IdspaceClause {
    #[pyo3(get, set)]
    prefix: String,         // should be `IdPrefix`
    #[pyo3(get, set)]
    url: String,            // should be `Url`
    #[pyo3(get, set)]
    description: Option<String>,
}

impl IdspaceClause {
    pub fn new(prefix: String, url: String) -> Self {
        Self { prefix, url, description: None }
    }

    pub fn with_description(prefix: String, url: String, description: String) -> Self {
        Self { prefix, url, description: Some(description) }
    }
}

impl From<IdspaceClause> for obo::HeaderClause {
    fn from(clause: IdspaceClause) -> Self {
        obo::HeaderClause::Idspace(
            obo::IdPrefix::from_str(&clause.prefix).unwrap(),
            FromStr::from_str(&clause.url).unwrap(),
            clause.description.map(|s| obo::QuotedString::from_str(&s).unwrap())
        )
    }
}

// --- TreatXrefsAsEquivalentClause ------------------------------------------

#[pyclass(extends=HeaderClause)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TreatXrefsAsEquivalentClause {
    #[pyo3(get, set)]
    idspace: String,   // Should be `IdPrefix`
}

// --- TreatXrefsAsGenusDifferentiaClause -------------------------------------

#[pyclass(extends=HeaderClause)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TreatXrefsAsGenusDifferentiaClause {
    #[pyo3(get, set)]
    idspace: String,   // Should be `IdPrefix`
    #[pyo3(get, set)]
    relation: String,  // Should be `RelationId`
    #[pyo3(get, set)]
    filler: String,    // Should be `ClassId`
}

// --- TreatXrefsAsReverseGenusDifferentiaClause ------------------------------

#[pyclass(extends=HeaderClause)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TreatXrefsAsReverseGenusDifferentiaClause {
    #[pyo3(get, set)]
    idspace: String,   // Should be `IdPrefix`
    #[pyo3(get, set)]
    relation: String,  // Should be `RelationId`
    #[pyo3(get, set)]
    filler: String,    // Should be `ClassId`
}

// --- TreatXrefsAsRelationshipClause -----------------------------------------

#[pyclass(extends=HeaderClause)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TreatXrefsAsRelationshipClause {
    #[pyo3(get, set)]
    idspace: String,
    #[pyo3(get, set)]
    relation: String,
}

// --- TreatXrefsAsIsA -------------------------------------------------------

#[pyclass(extends=HeaderClause)]
#[derive(Debug)]
pub struct TreatXrefsAsIsAClause {
    idspace: Py<PyString>,
    canonical: bool,
}

impl Clone for TreatXrefsAsIsAClause {
    fn clone(&self) -> Self {
        let gil = Python::acquire_gil();
        Self {
            idspace: self.idspace.clone_ref(gil.python()),
            canonical: self.canonical,
        }
    }
}

impl TreatXrefsAsIsAClause {
    pub fn new<S>(py: Python, idspace: S) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            idspace: PyString::new(py, idspace.as_ref()),
            canonical: false // FIXME(@althonos)
        }
    }
}

impl From<TreatXrefsAsIsAClause> for obo::HeaderClause {
    fn from(clause: TreatXrefsAsIsAClause) -> obo::HeaderClause {
        let gil = Python::acquire_gil();
        let py = gil.python();
        <obo::HeaderClauseRef as ToOwned>::to_owned(&clause.to_ref(py))
    }
}

impl TreatXrefsAsIsAClause {
    fn to_ref<'p, 's: 'p>(&'s self, py: Python<'p>) -> obo::HeaderClauseRef<'p> {
        let v = self.idspace.as_ref(py);
        let ptr = v.as_bytes().as_ptr();
        let s = unsafe { std::slice::from_raw_parts(ptr, v.as_bytes().len()) } ;
        let s = unsafe { std::str::from_utf8_unchecked(s) };
        obo::HeaderClauseRef::FormatVersion(Cow::Borrowed(obo::UnquotedStr::new(s)))
    }
}

#[pymethods]
impl TreatXrefsAsIsAClause {
    #[new]
    fn __init__(obj: &PyRawObject, idspace: String) {
        obj.init(Self::new(obj.py(), idspace));
    }

    #[getter]
    fn get_idspace(&self) -> PyResult<Py<PyString>> {
        Ok(self.idspace.clone_ref(Python::acquire_gil().python()))
    }

    #[setter]
    fn set_idspace(&mut self, idspace: &str) -> PyResult<()> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        // FIXME(@althonos): Check if canonical
        self.idspace = PyString::new(py, idspace);
        Ok(())
    }

    fn is_canonical(&self) -> PyResult<bool> {
        Ok(self.canonical)
    }
}

#[pyproto]
impl PyObjectProtocol for TreatXrefsAsIsAClause {
    fn __repr__(&self) -> PyResult<String> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let v = self.idspace.as_ref(py);
        Ok(format!("TreatXrefsAsIsAClause({})", v.repr()?.to_string()?))
    }

    fn __str__(&self) -> PyResult<String> {
        let gil = Python::acquire_gil();
        Ok(self.to_ref(gil.python()).to_string())
    }
}

// --- TreatXrefsAsHasSubclassClause -----------------------------------------

#[pyclass(extends=HeaderClause)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TreatXrefsAsHasSubclassClause {
    #[pyo3(get, set)]
    idspace: String,
}
