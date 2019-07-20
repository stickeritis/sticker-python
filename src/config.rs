use std::fs::File;
use std::io::BufReader;
use std::ops::Deref;

use pyo3::exceptions;
use pyo3::prelude::*;

use sticker_utils::{Config, TomlRead};

/// Tagger configuration.
#[pyclass(name=Config)]
pub struct PyConfig {
    inner: Config,
}

#[pymethods]
impl PyConfig {
    #[new]
    fn __new__(obj: &PyRawObject, path: &str) -> PyResult<()> {
        let reader = BufReader::new(File::open(path).map_err(|err| {
            exceptions::IOError::py_err(format!(
                "cannot read sticker configuration: {}",
                err.to_string()
            ))
        })?);
        let mut config = Config::from_toml_read(reader).map_err(|err| {
            exceptions::ValueError::py_err(format!(
                "cannot read parse configuration: {}",
                err.to_string()
            ))
        })?;

        config.relativize_paths(path).map_err(|err| {
            exceptions::IOError::py_err(format!("cannot relativize paths: {}", err.to_string()))
        })?;

        obj.init(PyConfig { inner: config });

        Ok(())
    }
}

impl Deref for PyConfig {
    type Target = Config;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
