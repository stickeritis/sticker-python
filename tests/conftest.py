import os

import pytest
from sticker import Config, Pipeline, Tagger

def pytest_addoption(parser):
    parser.addoption("--tagger_model", action="store", default=None)
    parser.addoption("--topo_model", action="store", default=None)

def pytest_generate_tests(metafunc):
    opt_tagger_model = metafunc.config.option.tagger_model
    opt_topo_model = metafunc.config.option.topo_model

    if 'tagger_model_file' in metafunc.fixturenames:
        if opt_tagger_model is not None:
            metafunc.parametrize("tagger_model_file", [opt_tagger_model])
        else:
            metafunc.parametrize("tagger_model_file", [])

    if 'topo_model_file' in metafunc.fixturenames:
        if opt_topo_model is not None:
            metafunc.parametrize("topo_model_file", [opt_topo_model])
        else:
            metafunc.parametrize("topo_model_file", [])

@pytest.fixture
def pipeline_model(tagger_model_file, topo_model_file):
    tagger_config = Config(tagger_model_file)
    topo_config = Config(topo_model_file)
    yield Pipeline([tagger_config, topo_config])

@pytest.fixture
def tagger_model(tagger_model_file):
    config = Config(tagger_model_file)
    yield Tagger(config)

@pytest.fixture
def tests_root():
    yield os.path.dirname(__file__)
