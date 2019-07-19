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

    assert sent[0].form == None
    assert sent[1].form == "Der"
    assert sent[2].form == "Hund"
    assert sent[3].form == "lauft"

    with pytest.raises(IndexError):
        sent[4]

def test_sentence_with_tags():
    sent = Sentence(["Der", "Hund", "lauft"], ["DET-ART", "NOUN-NN", "VERB-VVFIN"])

    assert len(sent) == 4

    assert sent[0].form == None
    assert sent[1].form == "Der"
    assert sent[2].form == "Hund"
    assert sent[3].form == "lauft"
    assert sent[0].pos == None
    assert sent[1].pos == "DET-ART"
    assert sent[2].pos == "NOUN-NN"
    assert sent[3].pos == "VERB-VVFIN"

    with pytest.raises(IndexError):
        sent[4]
