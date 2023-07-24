"""
File: setup.py
Project: python
Created Date: 24/07/2023
Author: Shun Suzuki
-----
Last Modified: 24/07/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""

from setuptools import Extension, setup

setup(
    ext_modules=[
        Extension(
            name="pyautd3.dummy",
            sources=["empty.cpp"],
        ),
    ]
)
