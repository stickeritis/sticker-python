from sticker import Config, Sentence, Tagger

def test_tag_sentence(tagger_model):
    sent = Sentence("Chandrayaan-2 ist eine Mondsonde .".split(" "))
    tagger_model.tag_sentence(sent)

    assert list(map(lambda t: t.pos, sent)) == \
        [None, "PROPN-NE", "AUX-VAFIN", "DET-ART", "NOUN-NN", "PUNCT-$."]

def test_tag_sentence_pipeline(pipeline_model):
    sent = Sentence("Chandrayaan-2 ist eine Mondsonde .".split(" "))
    pipeline_model.tag_sentence(sent)

    assert list(map(lambda t: t.pos, sent)) == \
        [None, "PROPN-NE", "AUX-VAFIN", "DET-ART", "NOUN-NN", "PUNCT-$."]

    assert list(map(lambda t: t.features["tf"], filter(lambda t: t.form != None, sent) )) == \
        ["VF", "LK", "MF", "MF", "UNK"]
