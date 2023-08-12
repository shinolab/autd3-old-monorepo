"""
File: evp.py
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

from .backend import Backend
from .constraint import AmplitudeConstraint

from pyautd3.native_methods.autd3capi_def import GainPtr
from pyautd3.geometry import Geometry

from pyautd3.gain.gain import IGain


class EVP(IGain):
    _foci: List[float]
    _amps: List[float]
    _backend: Backend
    _gamma: Optional[float]
    _constraint: Optional[AmplitudeConstraint]

    def __init__(self, backend: Backend):
        self._foci = []
        self._amps = []
        self._backend = backend
        self._gamma = None
        self._constraint = None

    def add_focus(self, focus: np.ndarray, amp: float) -> "EVP":
        self._foci.append(focus[0])
        self._foci.append(focus[1])
        self._foci.append(focus[2])
        self._amps.append(amp)
        return self

    def with_gamma(self, gamma: float) -> "EVP":
        self._gamma = gamma
        return self

    def with_backend(self, backend: Backend) -> "EVP":
        self._backend = backend
        return self

    def with_constraint(self, constraint: AmplitudeConstraint) -> "EVP":
        self._constraint = constraint
        return self

    def gain_ptr(self, _: Geometry) -> GainPtr:
        size = len(self._amps)
        foci_ = np.ctypeslib.as_ctypes(np.array(self._foci).astype(ctypes.c_double))
        amps = np.ctypeslib.as_ctypes(np.array(self._amps).astype(ctypes.c_double))
        ptr = self._backend.evp(foci_, amps, size)
        if self._gamma is not None:
            ptr = self._backend.evp_with_gamma(ptr, self._gamma)
        if self._constraint is not None:
            ptr = self._backend.evp_with_constraint(ptr, self._constraint)
        return ptr
