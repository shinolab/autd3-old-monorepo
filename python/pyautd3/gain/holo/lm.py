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
from typing import Optional, List
import ctypes

from .backend import Backend, DefaultBackend
from .constraint import AmplitudeConstraint

from pyautd3.native_methods.autd3capi_gain_holo import NativeMethods as GainHolo
from pyautd3.native_methods.autd3capi_def import GainPtr
from pyautd3.geometry import Geometry

from .holo import Holo


class LM(Holo):
    _eps1: Optional[float]
    _eps2: Optional[float]
    _tau: Optional[float]
    _kmax: Optional[int]
    _initial: List[float]
    _constraint: Optional[AmplitudeConstraint]

    def __init__(self, backend: Backend = DefaultBackend()):
        super().__init__(backend)
        self._eps1 = None
        self._eps2 = None
        self._tau = None
        self._kmax = None
        self._initial = []
        self._constraint = None

    def with_eps1(self, eps1: float) -> "LM":
        self._eps1 = eps1
        return self

    def with_eps2(self, eps2: float) -> "LM":
        self._eps2 = eps2
        return self

    def with_tau(self, tau: float) -> "LM":
        self._tau = tau
        return self

    def with_kmax(self, kmax: int) -> "LM":
        self._kmax = kmax
        return self

    def with_initial(self, initial: List[float]) -> "LM":
        self._initial = initial
        return self

    def with_constraint(self, constraint: AmplitudeConstraint) -> "LM":
        self._constraint = constraint
        return self

    def gain_ptr(self, _: Geometry) -> GainPtr:
        size = len(self._amps)
        foci_ = np.ctypeslib.as_ctypes(np.array(self._foci).astype(ctypes.c_double))
        amps = np.ctypeslib.as_ctypes(np.array(self._amps).astype(ctypes.c_double))
        ptr = GainHolo().gain_holo_lm(self._backend.ptr(), foci_, amps, size)
        if self._eps1 is not None:
            ptr = GainHolo().gain_holo_lm_with_eps_1(ptr, self._eps1)
        if self._eps2 is not None:
            ptr = GainHolo().gain_holo_lm_with_eps_2(ptr, self._eps2)
        if self._tau is not None:
            ptr = GainHolo().gain_holo_lm_with_tau(ptr, self._tau)
        if self._kmax is not None:
            ptr = GainHolo().gain_holo_lm_with_k_max(ptr, self._kmax)
        if len(self._initial) > 0:
            initial_ = np.ctypeslib.as_ctypes(
                np.array(self._initial).astype(ctypes.c_double)
            )
            ptr = GainHolo().gain_holo_lm_with_initial(
                ptr, initial_, len(self._initial)
            )
        if self._constraint is not None:
            ptr = GainHolo().gain_holo_lm_with_constraint(ptr, self._constraint.ptr())
        return ptr
