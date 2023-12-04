"""
File: configure.py
Project: tests
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 23/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


import pytest


def pytest_addoption(parser):
    parser.addoption("--soem", action="store_true", default=False, help="run soem tests")


def pytest_configure(config):
    config.addinivalue_line("markers", "soem: soem tests")


def pytest_collection_modifyitems(session, config, items):
    option_lists = [
        ("--soem", "soem"),
    ]
    for option, marker in option_lists:
        if config.getoption(option):
            continue
        skip = pytest.mark.skip()
        for item in items:
            if marker in item.keywords:
                item.add_marker(skip)
