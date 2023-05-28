"""
File: gain.py
Project: gain
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""

from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.autd import Body


class Gain(Body):
    _disposed: bool

    def __init__(self):
        super().__init__()
        self._disposed = False

    def __del__(self):
        if not self._disposed:
            Base().delete_gain(self.ptr)
        self._disposed = True
