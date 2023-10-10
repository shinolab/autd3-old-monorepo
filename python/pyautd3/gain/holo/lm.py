'''
File: lm.py
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
from typing import Optional, List
import ctypes

from pyautd3.geometry import Geometry

from .backend import Backend
from .holo import Holo

from pyautd3.native_methods.autd3capi_def import GainPtr


class LM(Holo):
    """Gain to produce multiple foci with Levenberg-Marquardt algorithm

    - Reference
        * Levenberg, Kenneth. "A method for the solution of certain non-linear problems in least squares,"
          Quarterly of applied mathematics 2.2 (1944): 164-168.
        * Marquardt, Donald W. "An algorithm for least-squares estimation of nonlinear parameters,"
          Journal of the society for Industrial and Applied Mathematics 11.2 (1963): 431-441.
        * K.Madsen, H.Nielsen, and O.Tingleff, “Methods for non-linear least squares problems (2nd ed.),” 2004.
    """

    _eps1: Optional[float]
    _eps2: Optional[float]
    _tau: Optional[float]
    _kmax: Optional[int]
    _initial: List[float]

    def __init__(self, backend: Backend):
        super().__init__(backend)
        self._eps1 = None
        self._eps2 = None
        self._tau = None
        self._kmax = None
        self._initial = []

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

    def gain_ptr(self, _: Geometry) -> GainPtr:
        size = len(self._amps)
        foci_ = np.ctypeslib.as_ctypes(np.array(self._foci).astype(ctypes.c_double))
        amps = np.ctypeslib.as_ctypes(np.array(self._amps).astype(ctypes.c_double))
        assert self._backend is not None
        ptr = self._backend.lm(foci_, amps, size)
        if self._eps1 is not None:
            ptr = self._backend.lm_with_eps1(ptr, self._eps1)
        if self._eps2 is not None:
            ptr = self._backend.lm_with_eps2(ptr, self._eps2)
        if self._tau is not None:
            ptr = self._backend.lm_with_tau(ptr, self._tau)
        if self._kmax is not None:
            ptr = self._backend.lm_with_kmax(ptr, self._kmax)
        if len(self._initial) > 0:
            initial_ = np.ctypeslib.as_ctypes(
                np.array(self._initial).astype(ctypes.c_double)
            )
            ptr = self._backend.lm_with_initial(ptr, initial_, len(self._initial))
        if self._constraint is not None:
            ptr = self._backend.lm_with_constraint(ptr, self._constraint)
        return ptr
