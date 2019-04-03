use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::string::ToString;

use fastobo::ast as obo;
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

        match &self.inner {
            FormatVersion(v) => {
                FormatVersionClause::new(py, v.as_str()).into_object(py)
            }
            DataVersion(v) => {
                DataVersionClause::new(v.as_str()).into_object(py)
            }
            Date(dt) => {
                DateClause::new(dt).into_object(py)
            }
            SavedBy(s) => SavedByClause::new(s.as_str()).into_object(py),
            // _ => unimplemented!("HeaderClause.to_object()")
            _ => py.NotImplemented(),
        }
    }
}

// --- FormatVersion ---------------------------------------------------------

#[pyclass(extends=HeaderClause)]
pub struct FormatVersionClause {
    version: Py<PyString>,
}

impl Clone for FormatVersionClause {
    fn clone(&self) -> Self {
        let gil = Python::acquire_gil();
        FormatVersionClause {
            version: self.version.clone_ref(gil.python())
        }
    }
}

impl FormatVersionClause {
    pub fn new<S>(py: Python, version: S) -> Self
    where
        S: AsRef<str>,
    {
        Self {version: PyString::new(py, version.as_ref())}
    }
}

impl From<FormatVersionClause> for obo::HeaderClause {
    fn from(clause: FormatVersionClause) -> obo::HeaderClause {
        let gil = Python::acquire_gil();
        let py = gil.python();
        <obo::HeaderClauseRef as ToOwned>::to_owned(&clause.to_ref(py))
    }
}

impl FormatVersionClause {
    fn to_ref<'p, 's: 'p>(&'s self, py: Python<'p>) -> obo::HeaderClauseRef<'p> {
        let v = self.version.as_ref(py);

        let buf = extend_slice_lifetime!(v.as_bytes());
        let s = match std::str::from_utf8(buf) {
            Ok(s) => Ok(s),
            Err(e) => Err(pyo3::PyErr::from_instance(
                pyo3::exceptions::UnicodeDecodeError::new_utf8(py, buf, e).unwrap(),
            )),
        }.unwrap();
        obo::HeaderClauseRef::FormatVersion(Cow::Borrowed(obo::UnquotedStr::new(s)))
    }
}

#[pymethods]
impl FormatVersionClause {
    #[new]
    fn __init__(obj: &PyRawObject, version: String) {
        obj.init(Self::new(obj.py(), version));
    }

    #[getter]
    fn get_version(&self) -> PyResult<Py<PyString>> {
        Ok(self.version.clone_ref(Python::acquire_gil().python()))
    }

    #[setter]
    fn set_version(&mut self, version: &str) -> PyResult<()> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.version = PyString::new(py, version);
        Ok(())
    }
}

#[pyproto]
impl PyObjectProtocol for FormatVersionClause {
    fn __repr__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        // let v = self.version.as_ref(py);

        let s = PyString::new(py, "FormatVersionClause({})").to_object(py);
        s.call_method1(py, "format", (self.version.as_ref(py).repr()?,))

    }

    fn __str__(&self) -> PyResult<String> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        println!("{:?}", self.to_ref(py));

        let s = self.to_ref(py).to_string();


        Ok(s)
    }
}

// --- DataVersion -----------------------------------------------------------

#[pyclass(extends=HeaderClause)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct DataVersionClause {
    #[pyo3(get, set)]
    version: String
}

impl DataVersionClause {
    pub fn new<S>(version: S) -> Self
    where
        S: Into<String>,
    {
        Self {version: version.into()}
    }
}

impl From<DataVersionClause> for obo::HeaderClause {
    fn from(clause: DataVersionClause) -> obo::HeaderClause {
        obo::HeaderClause::DataVersion(obo::UnquotedString::new(clause.version))
    }
}

impl Display for DataVersionClause {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        obo::HeaderClause::DataVersion(obo::UnquotedString::new(self.version.to_string()))
            .fmt(f)
    }
}

#[pymethods]
impl DataVersionClause {
    #[new]
    fn __init__(obj: &PyRawObject, version: String) {
        obj.init(Self::new(version));
    }
}

#[pyproto]
impl PyObjectProtocol for DataVersionClause {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("DataVersionClause({:?})", self.version))
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
        // WARNING(@althonos): possible black magic going on here, not sure
        // how dangerous it is to transform a Py<PyDateTime> into a PyObject
        // and back into a PyDateTime.
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
    #[pyo3(get, set)]
    name: String
}

impl SavedByClause {
    pub fn new<S>(version: S) -> Self
    where
        S: Into<String>,
    {
        Self {name: version.into()}
    }
}

impl From<SavedByClause> for obo::HeaderClause {
    fn from(clause: SavedByClause) -> obo::HeaderClause {
        obo::HeaderClause::SavedBy(obo::UnquotedString::new(clause.name))
    }
}

impl Display for SavedByClause {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        obo::HeaderClause::FormatVersion(obo::UnquotedString::new(self.name.to_string()))
            .fmt(f)
    }
}

#[pymethods]
impl SavedByClause {
    #[new]
    fn __init__(obj: &PyRawObject, version: String) {
        obj.init(Self::new(version));
    }
}

#[pyproto]
impl PyObjectProtocol for SavedByClause {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("SavedByClause({:?})", self.name))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.to_string())
    }
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

// impl<'a> FromPy<&'a TreatXrefsAsIsAClause> for obo::HeaderClauseRef<'_> {
//     fn from_py(clause: &'a TreatXrefsAsIsAClause, py: Python) -> Self {
//         let v = clause.idspace.as_ref(py);
//         let ptr = v.as_bytes().as_ptr();
//         let s = unsafe { std::slice::from_raw_parts(ptr, v.as_bytes().len()) } ;
//         let s = unsafe { std::str::from_utf8_unchecked(s) };
//         obo::HeaderClauseRef::TreatXrefsAsIsA(
//             Cow::Borrowed(fastobo::ast::IdPrf::new(s, false)) // FIXME
//         )
//     }
// }

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
