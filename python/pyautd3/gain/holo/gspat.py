"""
File: gspat.py
Project: holo
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""


import ctypes

import numpy as np

from pyautd3.geometry import Geometry
from pyautd3.native_methods.autd3capi_def import GainPtr

from .backend import Backend
from .holo import HoloWithBackend


class GSPAT(HoloWithBackend):
    """Gain to produce multiple foci with GS-PAT algorithm.

    Reference
    ---------
    - Plasencia, Diego Martinez, et al. "GS-PAT: high-speed multi-point sound-fields for phased arrays of transducers,"
        ACM Transactions on Graphics (TOG) 39.4 (2020): 138-1.
    """

    _repeat: int | None

    def __init__(self: "GSPAT", backend: Backend) -> None:
        super().__init__(backend)
        self._repeat = None

    def with_repeat(self: "GSPAT", value: int) -> "GSPAT":
        """Set parameter.

        Arguments:
        ---------
            value: parameter
        """
        self._repeat = value
        return self

    def _gain_ptr(self: "GSPAT", _: Geometry) -> GainPtr:
        size = len(self._amps)
        foci_ = np.ctypeslib.as_ctypes(np.array(self._foci).astype(ctypes.c_double))
        amps = np.ctypeslib.as_ctypes(np.fromiter((a.pascal for a in self._amps), dtype=float).astype(ctypes.c_double))
        ptr = self._backend._gspat(foci_, amps, size)
        if self._repeat is not None:
            ptr = self._backend._gspat_with_repeat(ptr, self._repeat)
        if self._constraint is not None:
            ptr = self._backend._gspat_with_constraint(ptr, self._constraint)
        return ptr
