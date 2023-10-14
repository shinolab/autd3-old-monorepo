'''
File: greedy.py
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
from typing import Optional
import ctypes

from .holo import Holo

from pyautd3.native_methods.autd3capi_gain_holo import NativeMethods as GainHolo
from pyautd3.native_methods.autd3capi_def import GainPtr
from pyautd3.geometry import Geometry


class Greedy(Holo):
    """Gain to produce multiple foci with greedy algorithm

    - Reference
        * Suzuki, Shun, et al. "Radiation pressure field reconstruction for ultrasound midair haptics by Greedy algorithm with brute-force search,"
          IEEE Transactions on Haptics 14.4 (2021): 914-921.
    """

    _div: Optional[int]

    def __init__(self):
        super().__init__(None)
        self._div = None

    def with_phase_div(self, div: int) -> "Greedy":
        self._div = div
        return self

    def gain_ptr(self, _: Geometry) -> GainPtr:
        size = len(self._amps)
        foci_ = np.ctypeslib.as_ctypes(np.array(self._foci).astype(ctypes.c_double))
        amps = np.ctypeslib.as_ctypes(np.array(self._amps).astype(ctypes.c_double))
        ptr = GainHolo().gain_holo_greedy(foci_, amps, size)
        if self._div is not None:
            ptr = GainHolo().gain_holo_greedy_with_phase_div(ptr, self._div)
        if self._constraint is not None:
            ptr = GainHolo().gain_holo_greedy_with_constraint(
                ptr, self._constraint.ptr()
            )
        return ptr
