use pyo3::{
    pyclass,
    types::{PyModule, PyModuleMethods as _},
    Bound, PyResult,
};

#[pyclass(module = "opentelemetry.sdk._native.trace")]
pub struct Span {
    name: String,
}

#[pyclass(module = "opentelemetry.sdk._native.trace")]
pub struct Tracer {
    inner: i32,
}

pub fn trace(parent: &Bound<'_, PyModule>) -> PyResult<()> {
    let py = parent.py();
    let submodule = PyModule::new_bound(py, "trace")?;
    submodule.add_class::<Span>()?;
    submodule.add_class::<Tracer>()?;
    parent.add_submodule(&submodule)?;
    Ok(())
}
