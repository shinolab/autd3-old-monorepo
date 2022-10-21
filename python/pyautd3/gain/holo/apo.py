'''
File: apo.py
Project: holo
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 21/10/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''

from .holo import Holo
from .backend import Backend

from ctypes import byref

from pyautd3.native_methods.autd3capi_gain_holo import NativeMethods as GainHolo


class APO(Holo):
    def __init__(self, backend: Backend, eps: float = 1e-8, lambda_: float = 1, k_max: int = 200, line_search_max: int = 100):
        super().__init__()
        GainHolo().dll.AUTDGainHoloAPO(
            byref(self.ptr),
            backend.ptr,
            eps,
            lambda_,
            k_max,
            line_search_max)

    def __del__(self):
        super().__del__()
