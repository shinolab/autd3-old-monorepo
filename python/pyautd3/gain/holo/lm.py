'''
File: lm.py
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


class LM(Holo):
    def __init__(self, backend: Backend, eps1: float = 1e-8, eps2: float = 1e-8, tau: float = 1e-3,
                 k_max: int = 5, initial=None):
        super().__init__()
        GainHolo().dll.AUTDGainHoloLM(
            byref(self.ptr),
            backend.ptr,
            eps1,
            eps2,
            tau,
            k_max,
            initial,
            0 if initial is None else len(initial))

    def __del__(self):
        super().__del__()
