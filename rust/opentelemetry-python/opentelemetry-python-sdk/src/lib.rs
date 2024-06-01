pub mod trace;

use pyo3::prelude::*;

#[pymodule]
fn _native(module: &Bound<'_, PyModule>) -> PyResult<()> {
    trace::trace(module)?;
    Ok(())
}
