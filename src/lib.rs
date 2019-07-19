use pyo3::prelude::*;

mod util;
pub use util::ListVec;

mod sentence;
pub use sentence::{PySentence, PyToken};

/// This is a Python module for wrapping the sticker sequence labeler.
#[pymodule]
fn sticker(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PySentence>()?;
    m.add_class::<PyToken>()?;

    Ok(())
}
