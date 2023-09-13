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


import functools
import numpy as np
from typing import Iterable, Optional, List, Tuple
import ctypes

from pyautd3.geometry import Geometry

from .backend import Backend
from .constraint import AmplitudeConstraint

from pyautd3.native_methods.autd3capi_def import GainPtr

from pyautd3.gain.gain import IGain


class LM(IGain):
    _foci: List[float]
    _amps: List[float]
    _backend: Backend
    _eps1: Optional[float]
    _eps2: Optional[float]
    _tau: Optional[float]
    _kmax: Optional[int]
    _initial: List[float]
    _constraint: Optional[AmplitudeConstraint]

    def __init__(self, backend: Backend):
        self._foci = []
        self._amps = []
        self._backend = backend
        self._eps1 = None
        self._eps2 = None
        self._tau = None
        self._kmax = None
        self._initial = []
        self._constraint = None

    def add_focus(self, focus: np.ndarray, amp: float) -> "LM":
        self._foci.append(focus[0])
        self._foci.append(focus[1])
        self._foci.append(focus[2])
        self._amps.append(amp)
        return self

    def add_foci_from_iter(
        self, iterable: Iterable[Tuple[np.ndarray, float]]
    ) -> "LM":
        return functools.reduce(
            lambda acc, x: acc.add_focus(x[0], x[1]),
            iterable,
            self,
        )

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
