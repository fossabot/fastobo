//! Definition of the Python classes exported in the `fastobo` module.

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
use crate::error::Error;
use crate::pyfile::PyFile;

// ---------------------------------------------------------------------------

pub mod entity;
pub mod doc;
pub mod header;
pub mod id;
pub mod term;
pub mod typedef;
pub mod pv;
pub mod syn;
pub mod xref;

use self::header::PyInit_header;
use self::typedef::PyInit_typedef;
use self::term::PyInit_term;
use self::id::PyInit_id;
use self::syn::PyInit_syn;
use self::pv::PyInit_pv;
use self::xref::PyInit_xref;

use self::doc::OboDoc;

// --- Module export ---------------------------------------------------------

#[pymodule]
fn fastobo(py: Python, m: &PyModule) -> PyResult<()> {

    m.add_class::<self::entity::BaseEntityFrame>()?;
    m.add_class::<self::doc::OboDoc>()?;

    m.add_wrapped(pyo3::wrap_pymodule!(header))?;
    m.add_wrapped(pyo3::wrap_pymodule!(id))?;
    m.add_wrapped(pyo3::wrap_pymodule!(pv))?;
    m.add_wrapped(pyo3::wrap_pymodule!(syn))?;
    m.add_wrapped(pyo3::wrap_pymodule!(term))?;
    m.add_wrapped(pyo3::wrap_pymodule!(typedef))?;
    m.add_wrapped(pyo3::wrap_pymodule!(xref))?;

    #[pyfn(m, "load")]
    fn load(py: Python, fh: &PyAny) -> PyResult<OboDoc> {
        if let Ok(s) = fh.downcast_ref::<PyString>() {
            let path = s.to_string()?;
            match obo::OboDoc::from_file(path.as_ref()) {
                Ok(doc) => Ok(doc.into_py(py)),
                Err(e) => Error::from(e).into(),
            }

        } else if let Ok(f) = PyFile::from_object(fh.py(), fh) {
            let mut bufreader = std::io::BufReader::new(f);
            match obo::OboDoc::from_stream(&mut bufreader) {
                Ok(doc) => Ok(doc.into_py(py)),
                Err(e) => Error::from(e).into(),
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
            Err(e) => Error::from(e).into(),
        }
    }

    Ok(())
}
