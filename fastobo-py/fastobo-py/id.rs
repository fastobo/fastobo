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
use fastobo::borrow::Borrow;
use fastobo::borrow::ToOwned;

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
        impl FromPy<$crate::fastobo::ast::$base> for $cls {
            fn from_py(id: $crate::fastobo::ast::$base, py: Python) -> $cls {
                let ident: ast::Ident = id.into();
                $cls::from_py(ident, py)
            }
        }

        impl FromPy<$cls> for $crate::fastobo::ast::$base {
            fn from_py(id: $cls, py: Python) -> $crate::fastobo::ast::$base {
                let ident: ast::Ident = id.into_py(py);
                $crate::fastobo::ast::$base::from(ident)
            }
        }
    };
}

#[derive(Debug, PartialEq, PyWrapper)]
#[wraps(BaseIdent)]
pub enum Ident {
    Unprefixed(Py<UnprefixedIdent>),
    Prefixed(Py<PrefixedIdent>),
    Url(Py<Url>),
}

impl FromPy<fastobo::ast::Ident> for Ident {
    fn from_py(ident: fastobo::ast::Ident, py: Python) -> Self {
        match ident {
            ast::Ident::Unprefixed(id) => Py::new(py, UnprefixedIdent::from(id))
                .map(Ident::Unprefixed),
            ast::Ident::Prefixed(id) => Py::new(py, PrefixedIdent::from(id))
                .map(Ident::Prefixed),
            ast::Ident::Url(id) => Py::new(py, Url::from(id))
                .map(Ident::Url)
        }
        .expect("could not allocate on Python heap")
    }
}

impl FromPy<Ident> for fastobo::ast::Ident {
    fn from_py(ident: Ident, py: Python) -> Self {
        match ident {
            Ident::Unprefixed(id) => {
                let i: UnprefixedIdent = id.as_ref(py).clone();
                ast::Ident::Unprefixed(i.into())
            }
            Ident::Prefixed(id) => {
                let i: PrefixedIdent = id.as_ref(py).clone();
                ast::Ident::Prefixed(i.into())
            }
            Ident::Url(id) => {
                let url: Url = id.as_ref(py).clone();
                ast::Ident::Url(url.into())
            }
        }
    }
}

impl_convert!(ClassIdent, Ident);
impl_convert!(RelationIdent, Ident);
impl_convert!(InstanceIdent, Ident);
impl_convert!(SubsetIdent, Ident);
impl_convert!(SynonymTypeIdent, Ident);
impl_convert!(NamespaceIdent, Ident);

// --- Base -------------------------------------------------------------------

#[pyclass(subclass)]
pub struct BaseIdent {}

// --- PrefixedIdent ----------------------------------------------------------

/// An identifier with a prefix.
#[pyclass(extends=BaseIdent)]
#[derive(Debug)]
pub struct PrefixedIdent {
    prefix: Py<IdentPrefix>,
    local: Py<IdentLocal>,
}

impl Clone for PrefixedIdent {
    fn clone(&self) -> Self {
        let gil = Python::acquire_gil();
        let py = gil.python();
        Self {
            prefix: self.prefix.clone_ref(py),
            local: self.local.clone_ref(py),
        }
    }
}

impl PartialEq for PrefixedIdent {
    fn eq(&self, other: &Self) -> bool {
        let gil = Python::acquire_gil();
        let py = gil.python();

        *self.prefix.as_ref(py) == *other.prefix.as_ref(py)
        && *self.local.as_ref(py) == *other.local.as_ref(py)

    }
}

impl Eq for PrefixedIdent {}

impl PrefixedIdent {
    fn new(prefix: Py<IdentPrefix>, local: Py<IdentLocal>) -> Self {
        PrefixedIdent { prefix, local }
    }
}

impl From<PrefixedIdent> for ast::PrefixedIdent {
    fn from(ident: PrefixedIdent) -> Self {

        let gil = Python::acquire_gil();
        let py = gil.python();

        ast::PrefixedIdent::new(
            ident.prefix.as_ref(py).clone().into(),
            ident.local.as_ref(py).clone().into(),
        )

    }
}

impl From<ast::PrefixedIdent> for PrefixedIdent {
    fn from(id: ast::PrefixedIdent) -> Self {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let prefix = fastobo::borrow::ToOwned::to_owned(&id.prefix());
        let local = fastobo::borrow::ToOwned::to_owned(&id.local());

        Self::new(
            Py::new(py, prefix.into())
                .expect("could not allocate on Python heap"),
            Py::new(py, local.into())
                .expect("could not allocate on Python heap"),
        )
    }
}

#[pymethods]
impl PrefixedIdent {

    #[new]
    fn __init__(obj: &PyRawObject, prefix: &PyAny, local: &PyAny) -> PyResult<()> {

        let py = prefix.py();

        let p = if prefix.downcast_ref::<IdentPrefix>().is_ok() {
            unsafe { Py::from_borrowed_ptr(prefix.as_ptr()) }
        } else if let Ok(ref s) = PyString::try_from(prefix) {
            let string = s.to_string();
            Py::new(py, IdentPrefix::new(ast::IdentPrefix::new(string)))?
        } else {
            let ty = prefix.get_type().name();
            let msg = format!("expected IdentPrefix or str, found {}", ty);
            return TypeError::into(msg);
        };

        let l = if local.downcast_ref::<IdentLocal>().is_ok() {
            unsafe { Py::from_borrowed_ptr(local.as_ptr()) }
        } else if let Ok(ref s) = PyString::try_from(local) {
            let string = s.to_string();
            Py::new(py, IdentLocal::new(ast::IdentLocal::new(string)))?
        } else {
            let ty = local.get_type().name();
            let msg = format!("expected IdentLocal or str, found {}", ty);
            return TypeError::into(msg);
        };

        Ok(obj.init(Self::new(p, l)))
    }

    /// `str`: the IDspace of the identifier.
    #[getter]
    fn get_prefix(&self) -> PyResult<Py<IdentPrefix>> {
        let py = unsafe { Python::assume_gil_acquired() };
        Ok(self.prefix.clone_ref(py))
    }

    #[setter]
    fn set_prefix(&mut self, prefix: &PyAny) -> PyResult<()> {
        let py = prefix.py();
        self.prefix = if prefix.downcast_ref::<IdentPrefix>().is_ok() {
            unsafe { Py::from_borrowed_ptr(prefix.as_ptr()) }
        } else if let Ok(ref s) = PyString::try_from(prefix) {
            let string = s.to_string();
            Py::new(py, IdentPrefix::new(ast::IdentPrefix::new(string)))?
        } else {
            let ty = prefix.get_type().name();
            let msg = format!("expected IdentPrefix or str, found {}", ty);
            return TypeError::into(msg);
        };
        Ok(())
    }

    /// `str`: the local part of the identifier.
    #[getter]
    fn get_local(&self) -> PyResult<Py<IdentLocal>> {
        let py = unsafe { Python::assume_gil_acquired() };
        Ok(self.local.clone_ref(py))
    }

    #[setter]
    fn set_local(&mut self, local: &PyAny) -> PyResult<()> {
        let py = local.py();
        self.local = if local.downcast_ref::<IdentLocal>().is_ok() {
            unsafe { Py::from_borrowed_ptr(local.as_ptr()) }
        } else if let Ok(ref s) = PyString::try_from(local) {
            let string = s.to_string();
            Py::new(py, IdentLocal::new(ast::IdentLocal::new(string)))?
        } else {
            let ty = local.get_type().name();
            let msg = format!("expected IdentLocal or str, found {}", ty);
            return TypeError::into(msg);
        };
        Ok(())
    }
}

#[pyproto]
impl PyObjectProtocol for PrefixedIdent {
    fn __repr__(&self) -> PyResult<PyObject> {
        // acquire the GIL
        let gil = Python::acquire_gil();
        let py = gil.python();
        // extract inner references
        let pref = self.prefix.as_ref(py);
        let lref = self.local.as_ref(py);
        // extract string slices
        let p = pref.inner.as_str();
        let l = lref.inner.as_str();
        // return the formatted `repr` string
        let fmt = PyString::new(py, "PrefixedIdent({!r}, {!r})").to_object(py);
        fmt.call_method1(py, "format", (p, l))
    }

    fn __str__(&self) -> PyResult<String> {
        let id: PrefixedIdent = self.clone();
        Ok(ast::PrefixedIdent::from(id).to_string())
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
#[derive(Clone, Debug, Eq, Hash, OpaqueTypedef, PartialEq)]
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
#[opaque_typedef(derive(FromInner, IntoInner, AsRefInner))]
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
    fn get_escaped(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }

    /// `str`: the unescaped representation of the identifier.
    #[getter]
    fn get_unescaped(&self) -> PyResult<&str> {
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
