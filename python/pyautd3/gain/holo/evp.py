'''
File: evp.py
Project: holo
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 02/03/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''


from .holo import Holo
from .backend import Backend

from ctypes import byref

from pyautd3.native_methods.autd3capi_gain_holo import NativeMethods as GainHolo


class EVP(Holo):
    def __init__(self, backend: Backend, gamma: float = 1.0):
        super().__init__()
        GainHolo().dll.AUTDGainHoloEVP(byref(self.ptr), backend.ptr, gamma)

    def __del__(self):
        super().__del__()
