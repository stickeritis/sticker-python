import os

import pytest
from sticker import Config, Sentence

def test_config(tests_root):
    Config(os.path.join(tests_root, "sticker.conf"))

def test_bogus_config(tests_root):
    with pytest.raises(ValueError):
        Config(os.path.join(tests_root, "bogus.conf"))

def test_missing_config(tests_root):
    with pytest.raises(IOError):
        Config(os.path.join(tests_root, "nonexistant.conf"))


def test_model(tests_root):
    config = Config(os.path.join(tests_root, "sticker.conf"))

    assert config.model.batch_size == 256
    assert config.model.graph.endswith("sticker.graph")
    assert config.model.intra_op_parallelism_threads == 2
    assert config.model.inter_op_parallelism_threads == 4
    assert config.model.parameters.endswith("epoch-37")

    config.model.batch_size = 128
    config.model.graph = "another.graph"
    config.model.intra_op_parallelism_threads = 1
    config.model.inter_op_parallelism_threads = 3
    config.model.parameters = "epoch-42"

    assert config.model.batch_size == 128
    assert config.model.graph == "another.graph"
    assert config.model.intra_op_parallelism_threads == 1
    assert config.model.inter_op_parallelism_threads == 3
    assert config.model.parameters == "epoch-42"
