use pyo3::{
    exceptions::PyTypeError, types::PyAnyMethods as _, Bound, FromPyObject, PyAny, PyResult,
};

pub enum Value {
    String(String),
    Bool(bool),
    Int(i64),
    Float(f64),
    StringArray(Vec<String>),
    BoolArray(Vec<bool>),
    IntArray(Vec<i64>),
    FloatArray(Vec<f64>),
}

impl<'py> FromPyObject<'py> for Value {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        if let Ok(s) = ob.extract::<String>() {
            return Ok(Value::String(s));
        }
        if let Ok(b) = ob.extract::<bool>() {
            return Ok(Value::Bool(b));
        }
        if let Ok(i) = ob.extract::<i64>() {
            return Ok(Value::Int(i));
        }
        if let Ok(f) = ob.extract::<f64>() {
            return Ok(Value::Float(f));
        }
        if let Ok(s) = ob.extract::<Vec<String>>() {
            return Ok(Value::StringArray(s));
        }
        if let Ok(b) = ob.extract::<Vec<bool>>() {
            return Ok(Value::BoolArray(b));
        }
        if let Ok(i) = ob.extract::<Vec<i64>>() {
            return Ok(Value::IntArray(i));
        }
        if let Ok(f) = ob.extract::<Vec<f64>>() {
            return Ok(Value::FloatArray(f));
        }
        Err(PyTypeError::new_err(format!(
            "Invalid type: {:?}",
            ob.get_type()
        )))
    }
}
