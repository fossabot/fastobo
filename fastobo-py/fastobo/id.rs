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

pub fn module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<self::BaseIdent>()?;
    m.add_class::<self::PrefixedIdent>()?;
    m.add_class::<self::UnprefixedIdent>()?;
    m.add_class::<self::Url>()?;
    Ok(())
}

// --- Conversion Wrapper -----------------------------------------------------

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

impl From<ast::Ident> for Ident {
    fn from(id: ast::Ident) -> Ident {
        Ident(id)
    }
}

impl From<ast::SubsetIdent> for Ident {
    fn from(id: ast::SubsetIdent) -> Self {
        let id: ast::Ident = id.into();
        Self(id)
    }
}

impl From<Ident> for ast::Ident {
    fn from(id: Ident) -> Self {
        id.0
    }
}

impl From<Ident> for ast::SubsetIdent {
    fn from(id: Ident) -> Self {
        ast::SubsetIdent::from(id.0)
    }
}

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
    #[new]
    fn __init__(obj: &PyRawObject, value: &str) -> PyResult<()> {
        match ast::UnprefixedIdent::from_str(value) {
            Ok(id) => Ok(obj.init(UnprefixedIdent::new(id))),
            // ERROR FIXME: add source
            Err(e) => ValueError::into(format!("invalid ident: {}", e))
        }
    }
}

#[pyproto]
impl PyObjectProtocol for UnprefixedIdent {
    fn __repr__(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let fmt = PyString::new(py, "UnprefixedIdent({!r})").to_object(py);
        fmt.call_method1(py, "format", (self.inner.to_string(),))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }
}

// --- UrlIdent ---------------------------------------------------------------

#[pyclass(extends=BaseIdent)]
#[derive(OpaqueTypedef)]
#[opaque_typedef(derive(FromInner, IntoInner))]
pub struct Url{
    inner: url::Url
}

impl Url {
    pub fn new(url: url::Url) -> Self {
        Url { inner: url }
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
        fmt.call_method1(py, "format", (self.inner.to_string(),))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(self.inner.to_string())
    }
}