import pytest
from sticker import Config, Tagger

def pytest_addoption(parser):
    parser.addoption("--tagger_model", action="store", default=None)

def pytest_generate_tests(metafunc):
    opt_tagger_model = metafunc.config.option.tagger_model

    if 'tagger_model_file' in metafunc.fixturenames:
        if opt_tagger_model is not None:
            metafunc.parametrize("tagger_model_file", [opt_tagger_model])
        else:
            metafunc.parametrize("tagger_model_file", [])

@pytest.fixture
def tagger_model(tagger_model_file):
    config = Config(tagger_model_file)
    yield Tagger(config)
