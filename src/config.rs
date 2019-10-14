use std::cell::{Ref, RefCell};
use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;

use pyo3::class::basic::PyObjectProtocol;
use pyo3::exceptions;
use pyo3::prelude::*;

use sticker_utils::{Config, TomlRead};

/// Config(file)
/// --
///
/// Tagger configuration.
#[pyclass(name=Config)]
pub struct PyConfig {
    inner: Rc<RefCell<Config>>,
}

impl PyConfig {
    pub fn as_ref(&self) -> Ref<Config> {
        self.inner.borrow()
    }
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

        obj.init(PyConfig {
            inner: Rc::new(RefCell::new(config)),
        });

        Ok(())
    }

    #[getter]
    fn get_labeler(&self) -> PyLabeler {
        PyLabeler {
            config: self.inner.clone(),
        }
    }

    #[getter]
    fn get_model(&self) -> PyModel {
        PyModel {
            config: self.inner.clone(),
        }
    }
}

#[pyclass(name=Model)]
pub struct PyModel {
    config: Rc<RefCell<Config>>,
}

#[pymethods]
impl PyModel {
    #[getter]
    fn get_graph(&self) -> String {
        self.config.borrow().model.graph.to_owned()
    }

    #[getter]
    fn get_inter_op_parallelism_threads(&self) -> usize {
        self.config.borrow().model.inter_op_parallelism_threads
    }

    #[getter]
    fn get_intra_op_parallelism_threads(&self) -> usize {
        self.config.borrow().model.intra_op_parallelism_threads
    }

    #[getter]
    fn get_parameters(&self) -> String {
        self.config.borrow().model.parameters.to_owned()
    }

    #[setter]
    fn set_graph(&self, graph: &str) {
        self.config.borrow_mut().model.graph = graph.to_owned()
    }

    #[setter]
    fn set_inter_op_parallelism_threads(&self, threads: usize) {
        self.config.borrow_mut().model.inter_op_parallelism_threads = threads
    }

    #[setter]
    fn set_intra_op_parallelism_threads(&self, threads: usize) {
        self.config.borrow_mut().model.intra_op_parallelism_threads = threads
    }

    #[setter]
    fn set_parameters(&self, parameters: &str) {
        self.config.borrow_mut().model.parameters = parameters.to_owned()
    }
}

#[pyproto]
impl PyObjectProtocol for PyModel {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.config.borrow().model))
    }
}

#[pyclass(name=Labeler)]
pub struct PyLabeler {
    config: Rc<RefCell<Config>>,
}

#[pymethods]
impl PyLabeler {
    #[getter]
    fn get_labels(&self) -> String {
        self.config.borrow().labeler.labels.clone()
    }

    #[setter]
    fn set_labels(&self, labels: &str) {
        self.config.borrow_mut().labeler.labels = labels.to_owned()
    }
}

#[pyproto]
impl PyObjectProtocol for PyLabeler {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.config.borrow().labeler))
    }
}
