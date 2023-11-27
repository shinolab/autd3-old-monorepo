"""
File: greedy.py
Project: holo
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 27/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""


import ctypes

import numpy as np

from pyautd3.geometry import Geometry
from pyautd3.native_methods.autd3capi_def import GainPtr
from pyautd3.native_methods.autd3capi_gain_holo import NativeMethods as GainHolo

from .holo import Holo


class Greedy(Holo):
    """Gain to produce multiple foci with greedy algorithm.

    References
    ----------
    - Suzuki, Shun, et al. "Radiation pressure field reconstruction for ultrasound midair haptics by Greedy algorithm with brute-force search,"
        IEEE Transactions on Haptics 14.4 (2021): 914-921.
    """

    _div: int | None

    def __init__(self: "Greedy") -> None:
        super().__init__()
        self._div = None

    def with_phase_div(self: "Greedy", div: int) -> "Greedy":
        """Set parameter.

        Arguments:
        ---------
            div: parameter
        """
        self._div = div
        return self

    def _gain_ptr(self: "Greedy", _: Geometry) -> GainPtr:
        size = len(self._amps)
        foci_ = np.ctypeslib.as_ctypes(np.array(self._foci).astype(ctypes.c_double))
        amps = np.ctypeslib.as_ctypes(np.fromiter((a.pascal for a in self._amps), dtype=float).astype(ctypes.c_double))
        ptr = GainHolo().gain_holo_greedy(foci_, amps, size)
        if self._div is not None:
            ptr = GainHolo().gain_holo_greedy_with_phase_div(ptr, self._div)
        if self._constraint is not None:
            ptr = GainHolo().gain_holo_greedy_with_constraint(ptr, self._constraint._constraint_ptr())
        return ptr
