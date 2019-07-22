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
