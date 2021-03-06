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
use crate::utils::ClonePy;

// --- Module export ---------------------------------------------------------

#[pymodule(xref)]
fn module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<self::Xref>()?;
    Ok(())
}

// --- Xref ------------------------------------------------------------------

/// A cross-reference to another entity or an external resource.
///
/// Xrefs can be used in a `~fastobo.term.DefClause` to indicate the provenance
/// of the definition, or in a `~fastobo.syn.Synonym` to add evidence from
/// literature supporting the origin of the synonym.
///
/// Example:
///     >>> xref = fastobo.xref.Xref(
///     ...     fastobo.id.PrefixedIdent('ISBN', '978-0-321-84268-8'),
///     ... )
#[pyclass]
#[derive(Debug)]
pub struct Xref {
    #[pyo3(set)]
    id: Ident,
    desc: Option<fastobo::ast::QuotedString>
}

impl Xref {
    pub fn new(_py: Python, id: Ident) -> Self {
        Self { id, desc: None }
    }

    pub fn with_desc<D>(py: Python, id: Ident, desc: Option<D>) -> Self
    where
        D: IntoPy<fastobo::ast::QuotedString>,
    {
        Self {
            id,
            desc: desc.map(|d| d.into_py(py)),
        }
    }

    pub fn from_object(py: Python, obj: &PyAny) -> PyResult<Py<Self>> {
        if Xref::is_instance(&obj) {
            unsafe {
                let ptr = obj.as_ptr();
                Ok(Py::from_borrowed_ptr(ptr))
            }
        } else {
            let ty = obj.get_type().name();
            TypeError::into(format!("expected Xref, found {}", ty))
        }
    }

}

impl ClonePy for Xref {
    fn clone_py(&self, py: Python) -> Self {
        Xref {
            id: self.id.clone_py(py),
            desc: self.desc.clone(),
        }
    }
}

impl FromPy<fastobo::ast::Xref> for Xref {
    fn from_py(xref: fastobo::ast::Xref, py: Python) -> Self {
        Self::with_desc(py, xref.id.into_py(py), xref.desc)
    }
}

impl FromPy<Xref> for fastobo::ast::Xref {
    fn from_py(xref: Xref, py: Python) -> Self {
        let id: fastobo::ast::Ident = xref.id.into_py(py);
        Self::with_desc(id, xref.desc)
    }
}

#[pymethods]
impl Xref {

    /// Create a new `Xref` instance from an ID and an optional description.
    ///
    /// Arguments:
    ///     id (~fastobo.id.Ident): the identifier of the reference.
    ///     desc (str, optional): an optional description for the reference.
    #[new]
    fn __init__(obj: &PyRawObject, id: Ident, desc: Option<String>) -> PyResult<()> {
        Ok(obj.init(
            Self::with_desc(obj.py(), id, desc.map(fastobo::ast::QuotedString::new))
        ))
    }

    /// Ident: the identifier of the reference.
    #[getter]
    fn get_id(&self) -> PyResult<&Ident> {
        Ok(&self.id)
    }

    /// str or None: the description of the reference, if any.
    #[getter]
    fn get_desc(&self) -> PyResult<Option<&str>> {
        match &self.desc {
            Some(d) => Ok(Some(d.as_str())),
            None => Ok(None)
        }
    }

    #[setter]
    fn set_desc(&mut self, desc: Option<String>) -> PyResult<()> {
        self.desc = desc.map(fastobo::ast::QuotedString::new);
        Ok(())
    }
}

#[pyproto]
impl PyObjectProtocol for Xref {
    fn __repr__(&self) -> PyResult<PyObject> {
        let py = unsafe { Python::assume_gil_acquired() };
        if let Some(ref d) = self.desc {
            PyString::new(py, "Xref({!r}, {!r})")
                .to_object(py)
                .call_method1(py, "format", (&self.id, d.as_str()))
        } else {
            PyString::new(py, "Xref({!r}, {!r})")
                .to_object(py)
                .call_method1(py, "format", (&self.id,))
        }
    }

    fn __str__(&self) -> PyResult<String> {
        let py = unsafe { Python::assume_gil_acquired() };
        Ok(fastobo::ast::Xref::from_py(self.clone_py(py), py).to_string())
    }
}

// --- XrefList --------------------------------------------------------------

#[pyclass]
#[derive(Debug)]
pub struct XrefList {
    xrefs: Vec<Py<Xref>>
}

impl XrefList {
    fn new(_py: Python, xrefs: Vec<Py<Xref>>) -> Self {
        Self { xrefs }
    }
}

impl ClonePy for XrefList {
    fn clone_py(&self, py: Python) -> Self {
        XrefList {
            xrefs: self.xrefs.clone_py(py)
        }
    }
}

impl FromPy<fastobo::ast::XrefList> for XrefList {
    fn from_py(list: fastobo::ast::XrefList, py: Python) -> Self {
        let mut xrefs = Vec::with_capacity((&list).len());
        for xref in list.into_iter() {
            xrefs.push(Py::new(py, xref.into_py(py)).unwrap())
        }
        Self::new(py, xrefs)
    }
}

impl FromPy<XrefList> for fastobo::ast::XrefList {
    fn from_py(list: XrefList, py: Python) -> Self {
        list
            .xrefs
            .into_iter()
            .map(|xref| xref.as_ref(py).clone_py(py).into_py(py))
            .collect()
    }
}

impl ToPyObject for XrefList {
    fn to_object(&self, py: Python) -> PyObject {
        PyList::new(py, &self.xrefs).into_object(py)
    }
}

#[pymethods]
impl XrefList {
    #[new]
    fn __init__(obj: &PyRawObject, xrefs: Option<&PyAny>) -> PyResult<()> {
        if let Some(x) = xrefs {
            let mut vec = Vec::new();
            for item in PyIterator::from_object(x.py(), x)? {
                let i = item?;
                if Xref::is_exact_instance(i) {
                    unsafe {
                        vec.push(Py::from_borrowed_ptr(i.as_ptr()));
                    }
                }
            }
            Ok(obj.init(Self::new(obj.py(), vec)))
        } else {
            Ok(obj.init(Self::new(obj.py(), Vec::new())))
        }
    }
}

#[pyproto]
impl PyObjectProtocol for XrefList {
    fn __repr__(&self) -> PyResult<PyObject> {
        let py = unsafe { Python::assume_gil_acquired() };
        let fmt = PyString::new(py, "XrefList({!r})").to_object(py);
        fmt.call_method1(py, "format", (self.to_object(py),))
    }

    fn __str__(&self) -> PyResult<String> {
        let py = unsafe { Python::assume_gil_acquired() };
        let frame: fastobo::ast::XrefList = self.clone_py(py).into_py(py);
        Ok(frame.to_string())
    }
}

#[pyproto]
impl PySequenceProtocol for XrefList {
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.xrefs.len())
    }

    fn __getitem__(&self, index: isize) -> PyResult<Py<Xref>> {
        let py = unsafe { Python::assume_gil_acquired() };
        if index < self.xrefs.len() as isize {
            Ok(self.xrefs[index as usize].clone_ref(py))
        } else {
            IndexError::into("list index out of range")
        }
    }
}
