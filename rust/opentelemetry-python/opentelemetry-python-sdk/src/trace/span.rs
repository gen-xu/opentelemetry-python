use std::collections::HashMap;

use pyo3::{pyclass, pymethods, Bound, PyAny, Python};

use crate::{
    attributes::{PyAttributes, PyAttributesExt},
    value::Value,
};

use super::event::Event;

#[pyclass(module = "opentelemetry.sdk._native.trace")]
pub struct Span {
    pub(super) name: String,
    pub(super) attributes: HashMap<String, Value>,
    pub(super) events: Vec<Event>,
    pub(super) start_time: i64,
    pub(super) end_time: i64,
}

impl Span {
    pub fn new(name: String, attributes: HashMap<String, Value>) -> Self {
        Span {
            name,
            attributes,
            events: Vec::new(),
            start_time: 0,
            end_time: 0,
        }
    }
}

#[pymethods]
impl Span {
    #[new]
    fn py_new(py: Python<'_>, name: &str, attributes: Option<PyAttributes<'_>>) -> Self {
        let attributes = attributes.map(|x| x.to_hashmap()).unwrap_or_default();
        py.allow_threads(|| Span::new(name.to_string(), attributes))
    }

    pub fn add_event(
        &mut self,
        py: Python<'_>,
        name: &str,
        attributes: Option<PyAttributes<'_>>,
        timestamp: Option<i64>,
    ) {
        let attributes = attributes.map(|x| x.to_hashmap()).unwrap_or_default();
        py.allow_threads(|| {
            let event = Event::new(name.to_string(), timestamp, attributes);
            self.events.push(event);
        });
    }

    pub fn start(
        &mut self,
        py: Python<'_>,
        timestamp: Option<i64>,
        parent_context: Option<Bound<'_, PyAny>>,
    ) {
        py.allow_threads(|| {
            self.start_time = timestamp
                .unwrap_or_else(|| std::time::UNIX_EPOCH.elapsed().unwrap().as_nanos() as i64);
        });
    }

    pub fn end(&mut self, py: Python<'_>, timestamp: Option<i64>) {
        py.allow_threads(|| {
            self.end_time = timestamp
                .unwrap_or_else(|| std::time::UNIX_EPOCH.elapsed().unwrap().as_nanos() as i64);
        });
    }
}
