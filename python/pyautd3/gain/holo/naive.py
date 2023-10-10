'''
File: naive.py
Project: holo
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

'''


import numpy as np
import ctypes

from pyautd3.geometry import Geometry

from .backend import Backend
from .holo import Holo

from pyautd3.native_methods.autd3capi_def import GainPtr


class Naive(Holo):
    """Gain to produce multiple foci with naive linear synthesis

    """

    def __init__(self, backend: Backend):
        super().__init__(backend)

    def gain_ptr(self, _: Geometry) -> GainPtr:
        size = len(self._amps)
        foci_ = np.ctypeslib.as_ctypes(np.array(self._foci).astype(ctypes.c_double))
        amps = np.ctypeslib.as_ctypes(np.array(self._amps).astype(ctypes.c_double))
        assert self._backend is not None
        ptr = self._backend.naive(foci_, amps, size)
        if self._constraint is not None:
            ptr = self._backend.naive_with_constraint(ptr, self._constraint)
        return ptr
