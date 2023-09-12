"""
File: gs.py
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

from .backend import Backend
from .constraint import AmplitudeConstraint

from pyautd3.native_methods.autd3capi_def import GainPtr
from pyautd3.geometry import Geometry


from pyautd3.gain.gain import IGain


class GS(IGain):
    _foci: List[float]
    _amps: List[float]
    _backend: Backend
    _repeat: Optional[int]
    _constraint: Optional[AmplitudeConstraint]

    def __init__(self, backend: Backend):
        self._foci = []
        self._amps = []
        self._backend = backend
        self._repeat = None
        self._constraint = None

    def add_focus(self, focus: np.ndarray, amp: float) -> "GS":
        self._foci.append(focus[0])
        self._foci.append(focus[1])
        self._foci.append(focus[2])
        self._amps.append(amp)
        return self

    def add_foci_from_iter(
        self, iterable: Iterable[Tuple[np.ndarray, float]]
    ) -> "GS":
        return functools.reduce(
            lambda acc, x: acc.add_focus(x[0], x[1]),
            iterable,
            self,
        )

    def with_repeat(self, value: int) -> "GS":
        self._repeat = value
        return self

    def with_backend(self, backend: Backend) -> "GS":
        self._backend = backend
        return self

    def with_constraint(self, constraint: AmplitudeConstraint) -> "GS":
        self._constraint = constraint
        return self

    def gain_ptr(self, _: Geometry) -> GainPtr:
        size = len(self._amps)
        foci_ = np.ctypeslib.as_ctypes(np.array(self._foci).astype(ctypes.c_double))
        amps = np.ctypeslib.as_ctypes(np.array(self._amps).astype(ctypes.c_double))
        ptr = self._backend.gs(foci_, amps, size)
        if self._repeat is not None:
            ptr = self._backend.gs_with_repeat(ptr, self._repeat)
        if self._constraint is not None:
            ptr = self._backend.gs_with_constraint(ptr, self._constraint)
        return ptr
