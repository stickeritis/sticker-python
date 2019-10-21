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

def test_top_k_sentence(tagger_model):
    sent = Sentence("Chandrayaan-2 ist eine Mondsonde .".split(" "))
    top_k = tagger_model.top_k_sentence(sent)

    assert list(map(lambda l: l[0][0], top_k)) == \
        ["PROPN-NE", "AUX-VAFIN", "DET-ART", "NOUN-NN", "PUNCT-$."]

    # The other labels are probably to model-dependent to check, but
    # we should at least check that we get three labels in a top-3
    # model.
    assert list(map(lambda l: len(l), top_k)) == \
        [3, 3, 3, 3, 3]

