use std::ops::{Deref, DerefMut};

use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyList};

/// A `Vec` that that can only be converted from a Python list.
pub struct ListVec<T> {
    inner: Vec<T>,
}

impl<T> ListVec<T> {
    /// Get wrapped `Vec`.
    pub fn into_inner(self) -> Vec<T> {
        self.inner
    }
}

impl<T> Deref for ListVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for ListVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a, T> FromPyObject<'a> for ListVec<T>
where
    T: FromPyObject<'a>,
{
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let py_list = ob
            .downcast_ref::<PyList>()
            .map_err(|_| exceptions::TypeError::py_err("argument of type 'list' expected"))?;
        let inner: Vec<T> = py_list.as_ref().extract()?;
        Ok(ListVec { inner })
    }
}
