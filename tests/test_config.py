import pytest
from sticker import Config, Sentence

def test_config():
    Config("tests/sticker.conf")

def test_bogus_config():
    with pytest.raises(ValueError):
        Config("tests/bogus.conf")

def test_missing_config():
    with pytest.raises(IOError):
        Config("tests/nonexistant.conf")
