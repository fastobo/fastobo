use std::str::FromStr;

use pyo3::PyObjectProtocol;
use pyo3::PyTypeInfo;
use pyo3::prelude::*;
use pyo3::exceptions::TypeError;
use pyo3::exceptions::ValueError;
use pyo3::types::PyAny;
use pyo3::types::PyString;

use fastobo::ast;

// --- Module export ----------------------------------------------------------

#[pymodule(id)]
fn module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<self::BaseIdent>()?;
    m.add_class::<self::PrefixedIdent>()?;
    m.add_class::<self::UnprefixedIdent>()?;
    m.add_class::<self::IdentPrefix>()?;
    m.add_class::<self::IdentLocal>()?;
    m.add_class::<self::Url>()?;
    Ok(())
}

// --- Conversion Wrapper -----------------------------------------------------

macro_rules! impl_convert {
    ($base:ident, $cls:ident) => {
        impl From<$crate::fastobo::ast::$base> for $cls {
            fn from(id: $crate::fastobo::ast::$base) -> $cls {
                let ident: ast::Ident = id.into();
                $cls::from(ident)
            }
        }

        impl From<$cls> for $crate::fastobo::ast::$base {
            fn from(id: $cls) -> $crate::fastobo::ast::$base {
                let ident: ast::Ident = id.into();
                $crate::fastobo::ast::$base::from(ident)
            }
        }
    };
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, OpaqueTypedef)]
#[opaque_typedef(derive(FromInner, IntoInner))]
pub struct Ident(ast::Ident);

impl FromStr for Ident {
    type Err = PyErr;
    fn from_str(s: &str) -> PyResult<Self> {
        match ast::Ident::from_str(s) {
            Ok(id) => Ok(Ident(id)),
            Err(e) => unimplemented!(),
        }
    }
}

impl_convert!(ClassIdent, Ident);
impl_convert!(RelationIdent, Ident);
impl_convert!(SubsetIdent, Ident);
impl_convert!(SynonymTypeIdent, Ident);
impl_convert!(NamespaceIdent, Ident);

impl IntoPyObject for Ident {
    fn into_object(self, py: Python) -> PyObject {
        use fastobo::ast::Ident::*;
        match self.0 {
            Unprefixed(id) => UnprefixedIdent::from(id).into_object(py),
            Prefixed(id) => PrefixedIdent::from(id).into_object(py),
            Url(_) => unimplemented!("Ident.into_object for Ident::Url")
        }
    }
}

impl<'source> FromPyObject<'source> for Ident {
    fn extract(ob: &'source PyAny) -> PyResult<Self> {    
        if let Ok(id) = ob.downcast_ref::<PrefixedIdent>() {
            Ok(Ident(id.inner.clone().into()))
        } else if let Ok(id) = ob.downcast_ref::<UnprefixedIdent>() {
            Ok(Ident(id.inner.clone().into()))
        } else if let Ok(url) = ob.downcast_ref::<Url>() {
            Ok(Ident(url.inner.clone().into()))
        } else {
            TypeError::into("expected PrefixedIdent or UnprefixedIdent")
        }
    }
}

// --- Base -------------------------------------------------------------------

#[pyclass(subclass)]
pub struct BaseIdent {}

// --- PrefixedIdent ----------------------------------------------------------

/// An identifier with a prefix.
#[pyclass(extends=BaseIdent)]
pub struct PrefixedIdent {
    inner: ast::PrefixedIdent,
}

impl PrefixedIdent {
    fn new(id: ast::PrefixedIdent) -> Self {
        PrefixedIdent { inner: id }
    }
}

impl From<PrefixedIdent> for ast::PrefixedIdent {
    fn from(ident: PrefixedIdent) -> Self {
        ident.inner
    }
}

impl From<ast::PrefixedIdent> for PrefixedIdent {
    fn from(id: ast::PrefixedIdent) -> Self {
        Self::new(id)
    }
}

#[pymethods]
impl PrefixedIdent {
    #[new]
    fn __init__(obj: &PyRawObject, value: &str) -> PyResult<()> {
        match ast::PrefixedIdent::from_str(value) {
            Ok(id) => Ok(obj.init(PrefixedIdent::new(id))),
            // ERROR FIXME: add source
            Err(e) => ValueError::into(format!("invalid ident: {}", e)),
        }
    }

    /// `str`: the IDspace of the identifier.
    #[getter]
    fn prefix(&self) -> PyResult<&str> {
        Ok(self.inner.prefix().as_str())
    }
}

#[pyproto]
impl PyObjectProtocol for PrefixedIdent {
    fn __repr__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let fmt = PyString::new(py, "PrefixedIdent({!r})").to_object(py);
        fmt.call_method1(py, "format", (self.inner.to_string(),))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }
}

// --- UnprefixedIdent --------------------------------------------------------

/// An identifier without a prefix.
///
/// Example:
///
///     .. code::
///
///         >>> from fastobo import UnprefixedIdent
///         >>> ident = UnprefixedIdent(\"hello world\")
///         >>> print(ident.escaped)
///         hello\\ world
///         >>> print(ident.unescaped)
///         hello world
///
#[pyclass(extends=BaseIdent)]
pub struct UnprefixedIdent {
    inner: ast::UnprefixedIdent,
}

impl UnprefixedIdent {
    fn new(id: ast::UnprefixedIdent) -> Self {
        UnprefixedIdent { inner: id }
    }
}

impl From<UnprefixedIdent> for ast::UnprefixedIdent {
    fn from(id: UnprefixedIdent) -> Self {
        id.inner
    }
}

impl From<ast::UnprefixedIdent> for UnprefixedIdent {
    fn from(id: ast::UnprefixedIdent) -> Self {
        Self::new(id)
    }
}

#[pymethods]
impl UnprefixedIdent {

    /// Create a new `UnprefixedIdent` instance.
    ///
    /// Arguments:
    ///     value (`str`): the unescaped representation of the identifier.
    #[new]
    fn __init__(obj: &PyRawObject, value: &str) -> PyResult<()> {
        let id = ast::UnprefixedIdent::new(value.to_string());
        Ok(obj.init(UnprefixedIdent::new(id)))
    }

    /// `str`: the escaped representation of the identifier.
    #[getter]
    fn escaped(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }

    /// `str`: the unescaped representation of the identifier.
    #[getter]
    fn unescaped(&self) -> PyResult<String> {
        Ok(self.inner.as_str().to_string())
    }
}

#[pyproto]
impl PyObjectProtocol for UnprefixedIdent {
    fn __repr__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let fmt = PyString::new(py, "UnprefixedIdent({!r})").to_object(py);
        fmt.call_method1(py, "format", (self.inner.as_str().to_string(),))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }
}

// --- UrlIdent ---------------------------------------------------------------

/// A URL used as an identifier.
#[pyclass(extends=BaseIdent)]
#[derive(Clone, Debug, Eq, Hash, OpaqueTypedef, PartialEq)]
#[opaque_typedef(derive(FromInner, IntoInner))]
pub struct Url{
    inner: url::Url
}

impl Url {
    pub fn new(url: url::Url) -> Self {
        Self { inner: url }
    }
}

#[pymethods]
impl Url {
    #[new]
    fn __init__(obj: &PyRawObject, value: &str) -> PyResult<()> {
        match url::Url::from_str(value) {
            Ok(url) => Ok(obj.init(Url::new(url))),
            Err(e) => ValueError::into(format!("invalid url: {}", e)),
        }
    }
}

#[pyproto]
impl PyObjectProtocol for Url {
    fn __repr__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let fmt = PyString::new(py, "Url({!r})").to_object(py);
        fmt.call_method1(py, "format", (self.inner.as_str(),))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }
}

/// --- IdentPrefix ----------------------------------------------------------

/// The prefix of a prefixed identifier.
#[pyclass]
#[derive(Clone, Debug, Eq, Hash, OpaqueTypedef, PartialEq)]
#[opaque_typedef(derive(FromInner, IntoInner))]
pub struct IdentPrefix {
    inner: ast::IdentPrefix
}

impl IdentPrefix {
    pub fn new(prefix: ast::IdentPrefix) -> Self {
        Self { inner: prefix }
    }
}

#[pymethods]
impl IdentPrefix {

    /// Create a new `IdentPrefix` instance.
    ///
    /// Arguments:
    ///     value (`str`): the unescaped representation of the prefix.
    #[new]
    fn __init__(obj: &PyRawObject, value: String) -> PyResult<()> {
        Ok(obj.init(Self::new(ast::IdentPrefix::new(value))))
    }

    /// `str`: the escaped representation of the identifier.
    #[getter]
    fn escaped(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }

    /// `str`: the unescaped representation of the identifier.
    #[getter]
    fn unescaped(&self) -> PyResult<&str> {
        Ok(self.inner.as_str())
    }
}

#[pyproto]
impl PyObjectProtocol for IdentPrefix {
    fn __repr__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let fmt = PyString::new(py, "IdentPrefix({!r})").to_object(py);
        fmt.call_method1(py, "format", (self.inner.as_str(),))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }
}

/// --- IdentLocal -----------------------------------------------------------

#[pyclass]
#[derive(Clone, Debug, Eq, Hash, OpaqueTypedef, PartialEq)]
#[opaque_typedef(derive(FromInner, IntoInner))]
pub struct IdentLocal {
    inner: ast::IdentLocal,
}

impl IdentLocal {
    pub fn new(local: ast::IdentLocal) -> Self {
        Self { inner: local }
    }
}

#[pymethods]
impl IdentLocal {

    /// Create a new `IdentLocal` instance.
    #[new]
    fn __init__(obj: &PyRawObject, value: String) -> PyResult<()> {
        Ok(obj.init(Self::new(ast::IdentLocal::new(value))))
    }

    /// `str`: the escaped representation of the identifier.
    #[getter]
    fn escaped(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }

    /// `str`: the unescaped representation of the identifier.
    #[getter]
    fn unescaped(&self) -> PyResult<&str> {
        Ok(self.inner.as_str())
    }
}

#[pyproto]
impl PyObjectProtocol for IdentLocal {
    fn __repr__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let fmt = PyString::new(py, "IdentLocal({!r})").to_object(py);
        fmt.call_method1(py, "format", (self.inner.as_str(),))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }
}
