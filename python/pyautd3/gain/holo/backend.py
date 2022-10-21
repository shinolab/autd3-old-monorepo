'''
File: backend.py
Project: holo
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 21/10/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''


from ctypes import c_void_p, byref

from pyautd3.native_methods.autd3capi_gain_holo import NativeMethods as GainHolo


class Backend():
    def __init__(self):
        self.ptr = c_void_p()

    def __del__(self):
        GainHolo().dll.AUTDDeleteBackend(self.ptr)


class EigenBackend(Backend):
    def __init__(self):
        super().__init__()
        GainHolo().dll.AUTDEigenBackend(byref(self.ptr))

    def __del__(self):
        super().__del__()
