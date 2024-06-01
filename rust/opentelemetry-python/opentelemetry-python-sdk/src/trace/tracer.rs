use std::collections::HashMap;

use pyo3::{pyclass, pymethods, Bound, PyAny, PyResult, Python};

use crate::{
    attributes::{self, PyAttributes, PyAttributesExt},
    value::Value,
};

use super::span::Span;

#[pyclass(module = "opentelemetry.sdk._native.trace")]
pub struct Tracer {}

#[pymethods]
impl Tracer {
    #[new]
    fn new() -> Self {
        Tracer {}
    }

    #[allow(clippy::too_many_arguments)]
    pub fn start_span(
        &self,
        py: Python<'_>,
        name: &str,
        context: Option<Bound<'_, PyAny>>,
        kind: Option<Bound<'_, PyAny>>,
        attributes: Option<PyAttributes<'_>>,
        links: Option<Bound<'_, PyAny>>,
        start_time: Option<i64>,
        record_exception: Option<bool>,
        set_status_on_exception: Option<bool>,
    ) -> PyResult<Span> {
        let attributes = attributes.map(|x| x.to_hashmap()).unwrap_or_default();
        py.allow_threads(|| Ok(Span::new(name.to_string(), attributes)))
    }
}
