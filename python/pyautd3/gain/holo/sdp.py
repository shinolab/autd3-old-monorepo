'''
File: sdp.py
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

from pyautd3.geometry import Geometry

from .backend import Backend
from .holo import Holo

from pyautd3.native_methods.autd3capi_def import GainPtr


class SDP(Holo):
    """Gain to produce multiple foci by solving Semi-Denfinite Programming

    - Reference
        * Inoue, Seki, Yasutoshi Makino, and Hiroyuki Shinoda. "Active touch perception produced by airborne ultrasonic haptic hologram,"
          2015 IEEE World Haptics Conference (WHC). IEEE, 2015.
    """

    _alpha: Optional[float]
    _lambda: Optional[float]
    _repeat: Optional[int]

    def __init__(self, backend: Backend):
        super().__init__(backend)
        self._alpha = None
        self._lambda = None
        self._repeat = None

    def with_alpha(self, alpha: float) -> "SDP":
        self._alpha = alpha
        return self

    def with_lambda(self, lambda_: float) -> "SDP":
        self._lambda = lambda_
        return self

    def with_repeat(self, repeat: int) -> "SDP":
        self._repeat = repeat
        return self

    def gain_ptr(self, _: Geometry) -> GainPtr:
        size = len(self._amps)
        foci_ = np.ctypeslib.as_ctypes(np.array(self._foci).astype(ctypes.c_double))
        amps = np.ctypeslib.as_ctypes(np.array(self._amps).astype(ctypes.c_double))
        ptr = self._backend.sdp(foci_, amps, size)
        if self._alpha is not None:
            ptr = self._backend.sdp_with_alpha(ptr, self._alpha)
        if self._lambda is not None:
            ptr = self._backend.sdp_with_lambda(ptr, self._lambda)
        if self._repeat is not None:
            ptr = self._backend.sdp_with_repeat(ptr, self._repeat)
        if self._constraint is not None:
            ptr = self._backend.sdp_with_constraint(ptr, self._constraint)
        return ptr
