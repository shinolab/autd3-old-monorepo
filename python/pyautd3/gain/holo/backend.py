"""
File: backend.py
Project: holo
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""


from ctypes import c_void_p

from pyautd3.native_methods.autd3capi_gain_holo import NativeMethods as GainHolo


class Backend:
    def __init__(self):
        self.ptr = c_void_p()


class DefaultBackend(Backend):
    def __init__(self):
        super().__init__()
        self.ptr = GainHolo().default_backend()
