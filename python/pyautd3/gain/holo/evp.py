'''
File: evp.py
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

from .backend import Backend
from .holo import Holo

from pyautd3.native_methods.autd3capi_def import GainPtr
from pyautd3.geometry import Geometry


class EVP(Holo):
    """Gain to produce multiple foci by solving Eigen Value Problem

    - Reference
        * Long, Benjamin, et al. "Rendering volumetric haptic shapes in mid-air using ultrasound,"
          ACM Transactions on Graphics (TOG) 33.6 (2014): 1-10.
    """

    _gamma: Optional[float]

    def __init__(self, backend: Backend):
        super().__init__(backend)
        self._gamma = None

    def with_gamma(self, gamma: float) -> "EVP":
        self._gamma = gamma
        return self

    def gain_ptr(self, _: Geometry) -> GainPtr:
        size = len(self._amps)
        foci_ = np.ctypeslib.as_ctypes(np.array(self._foci).astype(ctypes.c_double))
        amps = np.ctypeslib.as_ctypes(np.array(self._amps).astype(ctypes.c_double))
        assert self._backend is not None
        ptr = self._backend.evp(foci_, amps, size)
        if self._gamma is not None:
            ptr = self._backend.evp_with_gamma(ptr, self._gamma)
        if self._constraint is not None:
            ptr = self._backend.evp_with_constraint(ptr, self._constraint)
        return ptr
