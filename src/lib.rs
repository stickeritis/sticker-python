use pyo3::prelude::*;

mod util;
pub use util::ListVec;

mod config;
use config::{PyConfig, PyLabeler, PyModel};

mod tagger;
use tagger::{PyPipeline, PyTagger};

mod sentence;
pub use sentence::{PySentence, PySentenceIterator, PyToken};

/// This is a Python module for wrapping the sticker sequence labeler.
#[pymodule]
fn sticker(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyConfig>()?;
    m.add_class::<PyLabeler>()?;
    m.add_class::<PyModel>()?;
    m.add_class::<PyPipeline>()?;
    m.add_class::<PyTagger>()?;
    m.add_class::<PySentence>()?;
    m.add_class::<PySentenceIterator>()?;
    m.add_class::<PyToken>()?;

    Ok(())
}
