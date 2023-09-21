'''
File: configure.py
Project: tests
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 20/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


import pytest


def pytest_addoption(parser):
    parser.addoption(
        '--test_simulator',
        action='store_true',
        default=False,
        help='run simulator tests'
    )
    parser.addoption(
        '--test_twincat',
        action='store_true',
        default=False,
        help='run twincat tests'
    )
    parser.addoption(
        '--test_remote_twincat',
        action='store_true',
        default=False,
        help='run remote twincat tests'
    )
    parser.addoption(
        '--test_soem',
        action='store_true',
        default=False,
        help='run soem tests'
    )
    parser.addoption(
        '--test_remote_soem',
        action='store_true',
        default=False,
        help='run remote soem tests'
    )

    parser.addoption(
        '--test_geometry_viewer',
        action='store_true',
        default=False,
        help='run geometry viewer tests'
    )

    parser.addoption(
        '--test_cuda',
        action='store_true',
        default=False,
        help='run cuda tests'
    )


def pytest_configure(config):
    config.addinivalue_line('markers', 'simulator: mark test as simulator test')
    config.addinivalue_line('markers', 'twincat: mark test as twincat test')
    config.addinivalue_line('markers', 'remote_twincat: mark test as remote twincat test')
    config.addinivalue_line('markers', 'soem: mark test as soem test')
    config.addinivalue_line('markers', 'remote_soem: mark test as remote soem test')
    config.addinivalue_line('markers', 'geometry_viewer: mark test as geometry viewer test')
    config.addinivalue_line('markers', 'cuda: mark test as cuda test')


def pytest_collection_modifyitems(session, config, items):
    option_lists = [
        ('--test_simulator', 'simulator'),
        ('--test_twincat', 'twincat'),
        ('--test_remote_twincat', 'remote_twincat'),
        ('--test_soem', 'soem'),
        ('--test_remote_soem', 'remote_soem'),
        ('--test_geometry_viewer', 'geometry_viewer'),
        ('--test_cuda', 'cuda'),
    ]
    for option, marker in option_lists:
        if config.getoption(option):
            continue
        skip = pytest.mark.skip()
        for item in items:
            if marker in item.keywords:
                item.add_marker(skip)
