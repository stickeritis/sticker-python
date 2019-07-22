from sticker import Config, Sentence, Tagger

def test_tag_sentence(tagger_model):
    sent = Sentence("Chandrayaan-2 ist eine Mondsonde .".split(" "))
    tagger_model.tag_sentence(sent)

    # Fixme: after Sentence supports iteration.
    assert sent[0].pos is None
    assert sent[1].pos == "PROPN-NE"
    assert sent[2].pos == "AUX-VAFIN"
    assert sent[3].pos == "DET-ART"
    assert sent[4].pos == "NOUN-NN"
    assert sent[5].pos == "PUNCT-$."
