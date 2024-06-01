use std::collections::HashMap;

use pyo3::{
    types::{PyAnyMethods, PyDict},
    Bound,
};

use crate::value::Value;

pub type PyAttributes<'a> = Bound<'a, PyDict>;

pub trait PyAttributesExt {
    fn to_hashmap(&self) -> HashMap<String, Value>;
}

impl<'a> PyAttributesExt for PyAttributes<'a> {
    fn to_hashmap(&self) -> HashMap<String, Value> {
        let mut extracted = HashMap::new();
        for (k, v) in self {
            let key = k.extract::<String>().unwrap();
            let value = v.extract::<Value>().unwrap();
            extracted.insert(key, value);
        }
        extracted
    }
}
