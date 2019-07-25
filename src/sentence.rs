use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use conllx::graph::{Node, Sentence};
use conllx::token::{Token, TokenBuilder};
use pyo3::class::basic::PyObjectProtocol;
use pyo3::class::iter::PyIterProtocol;
use pyo3::class::sequence::PySequenceProtocol;
use pyo3::exceptions;
use pyo3::prelude::*;

use crate::util::ListVec;

/// Sentence that can be annotated.
#[pyclass(name=Sentence)]
pub struct PySentence {
    inner: Rc<RefCell<Sentence>>,
}

#[pymethods]
impl PySentence {
    /// Construct a new sentence from forms and (optionally) POS tags.
    ///
    /// The constructor will throw a `ValueError` if POS tags are
    /// provided, but the number or tags is not equal to the number of
    /// tokens.
    #[new]
    fn __new__(
        obj: &PyRawObject,
        forms: ListVec<&str>,
        pos_tags: Option<ListVec<&str>>,
    ) -> PyResult<()> {
        let sent = match pos_tags {
            Some(pos_tags) => {
                if forms.len() != pos_tags.len() {
                    return Err(exceptions::ValueError::py_err(format!(
                        "token and part-of-speech lists have inequal lengths: {} {}",
                        forms.len(),
                        pos_tags.len()
                    )));
                }

                forms
                    .into_inner()
                    .into_iter()
                    .zip(pos_tags.into_inner())
                    .map(|(form, pos_tag)| TokenBuilder::new(form).pos(pos_tag).into())
                    .collect::<Sentence>()
            }
            None => forms
                .into_inner()
                .into_iter()
                .map(Token::new)
                .collect::<Sentence>(),
        };

        obj.init(PySentence {
            inner: Rc::new(sent.into()),
        });

        Ok(())
    }
}

impl PySentence {
    pub fn inner(&self) -> RefMut<Sentence> {
        self.inner.borrow_mut()
    }
}

#[pyproto]
impl PyIterProtocol for PySentence {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<PySentenceIterator> {
        Ok(PySentenceIterator {
            sent: slf.inner.clone(),
            idx: 0,
        })
    }
}

#[pyproto]
impl PyObjectProtocol for PySentence {
    fn __repr__(&self) -> PyResult<String> {
        let token_reprs = (0..self.inner.borrow().len())
            .map(|token_idx| {
                PyToken {
                    token_idx,
                    sent: self.inner.clone(),
                }
                .__repr__()
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(format!("Sentence([{}])", token_reprs.join(", ")))
    }

    fn __str__(&self) -> PyResult<String> {
        Ok(format!("{}", self.inner.borrow()))
    }
}

#[pyproto]
impl PySequenceProtocol for PySentence {
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.inner.borrow().len())
    }

    fn __getitem__(&self, idx: isize) -> PyResult<PyToken> {
        if idx >= self.inner.borrow().len() as isize || idx < 0 {
            Err(exceptions::IndexError::py_err("token index out of range"))
        } else {
            Ok(PyToken {
                sent: self.inner.clone(),
                token_idx: idx as usize,
            })
        }
    }
}

/// Iterator over the nodes in a dependency graph.
///
/// The nodes are returned in sentence-linear order.
#[pyclass(name=SentenceIterator)]
pub struct PySentenceIterator {
    sent: Rc<RefCell<Sentence>>,
    idx: usize,
}

#[pyproto]
impl PyIterProtocol for PySentenceIterator {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<Py<PySentenceIterator>> {
        Ok(slf.into())
    }

    fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<PyToken>> {
        let slf = &mut *slf;

        if slf.idx < slf.sent.borrow().len() {
            let token = PyToken {
                sent: slf.sent.clone(),
                token_idx: slf.idx,
            };

            slf.idx += 1;

            Ok(Some(token))
        } else {
            Ok(None)
        }
    }
}

/// Token that can be annotated.
#[pyclass(name=Token)]
pub struct PyToken {
    sent: Rc<RefCell<Sentence>>,
    token_idx: usize,
}

#[pymethods]
impl PyToken {
    /// Get the form of the token.
    #[getter]
    fn get_form(&self) -> Option<String> {
        match self.sent.borrow()[self.token_idx] {
            Node::Token(ref token) => Some(token.form().to_owned()),
            Node::Root => None,
        }
    }

    /// Get the (dependency) head.
    #[getter]
    fn get_head(&self) -> Option<usize> {
        self.sent
            .borrow()
            .dep_graph()
            .head(self.token_idx)
            .map(|triple| triple.head())
    }

    /// Get the (dependency) head relation.
    #[getter]
    fn get_head_rel(&self) -> Option<String> {
        self.sent
            .borrow()
            .dep_graph()
            .head(self.token_idx)
            .and_then(|triple| triple.relation().map(ToOwned::to_owned))
    }

    /// Get the part-of-speech.
    #[getter]
    fn get_pos(&self) -> Option<String> {
        match self.sent.borrow()[self.token_idx] {
            Node::Token(ref token) => token.pos().map(ToOwned::to_owned),
            Node::Root => None,
        }
    }
}

#[pyproto]
impl PyObjectProtocol for PyToken {
    fn __repr__(&self) -> PyResult<String> {
        match self.sent.borrow()[self.token_idx] {
            Node::Root => Ok("Root".to_string()),
            Node::Token(ref token) => {
                let mut attrs = Vec::new();
                attrs.push(format!("form = '{}'", token.form()));

                if let Some(pos) = token.pos() {
                    attrs.push(format!("pos = '{}'", pos));
                };

                if let Some(triple) = self.sent.borrow().dep_graph().head(self.token_idx) {
                    attrs.push(format!("head = {}", triple.head()));

                    if let Some(relation) = triple.relation() {
                        attrs.push(format!("relation = {}", relation));
                    }
                }

                Ok(format!("Token({})", attrs.join(", ")))
            }
        }
    }
}
