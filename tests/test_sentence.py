import pytest
from sticker import Sentence


def test_form_type():
    with pytest.raises(TypeError):
        Sentence("blah")


def test_incorrect_tags_length():
    with pytest.raises(ValueError):
        Sentence(["Der", "Hund", "lauft"], ["DET-ART", "NOUN-NN"])


def test_pos_type():
    with pytest.raises(TypeError):
        Sentence(["a", "b", "c"], "ABC")


def test_sentence():
    sent = Sentence(["Der", "Hund", "lauft"])

    assert len(sent) == 4

    forms = list(map(lambda node: node.form, sent))
    assert forms == [None, "Der", "Hund", "lauft"]


def test_sentence_index():
    sent = Sentence(["Der", "Hund", "lauft"])

    assert sent[0].form is None
    assert sent[3].form == "lauft"

    with pytest.raises(IndexError):
        sent[4]


def test_sentence_with_tags():
    sent = Sentence(["Der", "Hund", "lauft"], [
                    "DET-ART", "NOUN-NN", "VERB-VVFIN"])

    assert len(sent) == 4

    forms = list(map(lambda node: node.form, sent))
    assert forms == [None, "Der", "Hund", "lauft"]

    postags = list(map(lambda node: node.pos, sent))
    assert postags == [None, "DET-ART", "NOUN-NN", "VERB-VVFIN"]

    with pytest.raises(IndexError):
        sent[4]
