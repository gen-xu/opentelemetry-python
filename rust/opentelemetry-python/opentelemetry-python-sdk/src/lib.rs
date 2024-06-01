pub mod trace;
pub(crate) mod value;
pub(crate) mod attributes;

use pyo3::prelude::*;

#[pymodule]
#[pyo3(name = "_native")]
fn _native(module: &Bound<'_, PyModule>) -> PyResult<()> {
    trace::trace(module)?;
    Ok(())
}
