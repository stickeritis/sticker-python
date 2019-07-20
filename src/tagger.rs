use pyo3::exceptions;
use pyo3::prelude::*;

use sticker_utils::TaggerWrapper;

use crate::PyConfig;
use crate::PySentence;

/// Tagger(config)
/// --
///
/// Instances of this class represent taggers. A tagger is constructed
/// from a configuration.
#[pyclass(name=Tagger)]
pub struct PyTagger {
    inner: TaggerWrapper,
}

#[pymethods]
impl PyTagger {
    #[new]
    fn __new__(obj: &PyRawObject, config: &PyConfig) -> PyResult<()> {
        let tagger = TaggerWrapper::new(&config)
            .map_err(|err| exceptions::IOError::py_err(err.to_string()))?;

        obj.init(PyTagger { inner: tagger });

        Ok(())
    }

    /// tag_sentence(sentence)
    /// --
    ///
    /// Tag a sentence. The sentences is modified in-place, adding the
    /// annotations.
    ///
    /// Parameters
    /// ----------
    /// sentence : Sentence
    ///     Sentence object to annotate.
    fn tag_sentence(&self, sentence: &PySentence) -> PyResult<()> {
        self.tag_sentences(vec![sentence])
    }

    /// tag_sentences(sentences)
    /// --
    ///
    /// Tag a list of sentences. The sentences are modified in-place, adding
    /// the annotations.
    ///
    /// Parameters
    /// ----------
    /// sentences : list
    ///     List of Sentence objects to annotate.
    fn tag_sentences(&self, sentences: Vec<&PySentence>) -> PyResult<()> {
        // We need borrowed_sents to exist as long as sents, since the
        // lifetimes of sentences are bound to RefMut returned by
        // PySentence::inner.
        let mut borrowed_sents = sentences
            .into_iter()
            .map(|sent| sent.inner())
            .collect::<Vec<_>>();
        let mut sents = borrowed_sents
            .iter_mut()
            .map(|sent| &mut **sent)
            .collect::<Vec<_>>();
        self.inner
            .tag_sentences(&mut sents)
            .map_err(|err| exceptions::RuntimeError::py_err(err.to_string()))
    }
}
