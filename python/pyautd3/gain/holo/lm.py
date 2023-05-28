"""
File: lm.py
Project: holo
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""


import numpy as np
from .holo import Holo
from .backend import Backend


from pyautd3.native_methods.autd3capi_gain_holo import NativeMethods as GainHolo


class LM(Holo):
    def __init__(self, backend: Backend):
        super().__init__()
        self.ptr = GainHolo().gain_holo_lm(
            backend.ptr,
        )

    def eps1(self, eps: float):
        GainHolo().gain_holo_lm_eps_1(self.ptr, eps)

    def eps2(self, eps: float):
        GainHolo().gain_holo_lm_eps_2(self.ptr, eps)

    def tau(self, tau: float):
        GainHolo().gain_holo_lm_tau(self.ptr, tau)

    def k_max(self, k_max: int):
        GainHolo().gain_holo_lmk_max(self.ptr, k_max)

    def initial(self, value: np.ndarray):
        value_ = np.ctypeslib.as_ctypes(value.astype(np.double))
        size = len(value)
        GainHolo().gain_holo_lm_initial(self.ptr, value_, size)

    def __del__(self):
        super().__del__()
