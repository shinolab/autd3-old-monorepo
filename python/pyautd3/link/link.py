"""
File: link.py
Project: link
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""

import ctypes
from pyautd3.native_methods.autd3capi_def import LinkPtr

LogOutputFunc = ctypes.CFUNCTYPE(None, ctypes.c_char_p)
LogFlushFunc = ctypes.CFUNCTYPE(None)


class Link:
    _ptr: LinkPtr

    def __init__(self, ptr: LinkPtr):
        self._ptr = ptr

    def ptr(self) -> LinkPtr:
        return self._ptr
