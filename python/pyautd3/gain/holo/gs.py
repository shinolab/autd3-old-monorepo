"""
File: gs.py
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


class GS(HoloWithBackend):
    """Gain to produce multiple foci with GS algorithm.

    References
    ----------
    - Marzo, Asier, and Bruce W. Drinkwater. "Holographic acoustic tweezers,"
        Proceedings of the National Academy of Sciences 116.1 (2019): 84-89.
    """

    _repeat: int | None

    def __init__(self: "GS", backend: Backend) -> None:
        super().__init__(backend)
        self._repeat = None

    def with_repeat(self: "GS", value: int) -> "GS":
        """Set parameter.

        Arguments:
        ---------
            value: parameter
        """
        self._repeat = value
        return self

    def _gain_ptr(self: "GS", _: Geometry) -> GainPtr:
        size = len(self._amps)
        foci_ = np.ctypeslib.as_ctypes(np.array(self._foci).astype(ctypes.c_double))
        amps = np.ctypeslib.as_ctypes(np.fromiter((a.pascal for a in self._amps), dtype=float).astype(ctypes.c_double))
        ptr = self._backend._gs(foci_, amps, size)
        if self._repeat is not None:
            ptr = self._backend._gs_with_repeat(ptr, self._repeat)
        if self._constraint is not None:
            ptr = self._backend._gs_with_constraint(ptr, self._constraint)
        return ptr
