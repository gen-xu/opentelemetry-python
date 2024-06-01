mod span;
mod tracer;
mod event;

use pyo3::{
    types::{PyModule, PyModuleMethods as _},
    Bound, PyResult,
};
use span::Span;
use tracer::Tracer;

pub fn trace(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = parent.py();

    let submodule = PyModule::new_bound(py, "trace")?;
    submodule.add_class::<Span>()?;
    submodule.add_class::<Tracer>()?;
    parent.add_submodule(&submodule)?;
    Ok(())
}
