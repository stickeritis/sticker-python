use pyo3::prelude::*;

/// This is a Python module for wrapping the sticker sequence labeler.
#[pymodule]
fn sticker(_py: Python, _m: &PyModule) -> PyResult<()> {
    Ok(())
}
