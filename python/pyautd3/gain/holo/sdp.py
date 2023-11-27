"""
File: sdp.py
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


class SDP(HoloWithBackend):
    """Gain to produce multiple foci by solving Semi-Denfinite Programming.

    References
    ----------
    - Inoue, Seki, Yasutoshi Makino, and Hiroyuki Shinoda. "Active touch perception produced by airborne ultrasonic haptic hologram,"
        2015 IEEE World Haptics Conference (WHC). IEEE, 2015.
    """

    _alpha: float | None
    _lambda: float | None
    _repeat: int | None

    def __init__(self: "SDP", backend: Backend) -> None:
        super().__init__(backend)
        self._alpha = None
        self._lambda = None
        self._repeat = None

    def with_alpha(self: "SDP", alpha: float) -> "SDP":
        """Set parameter.

        Arguments:
        ---------
            alpha: parameter
        """
        self._alpha = alpha
        return self

    def with_lambda(self: "SDP", lambda_: float) -> "SDP":
        """Set parameter.

        Arguments:
        ---------
            lambda_: parameter
        """
        self._lambda = lambda_
        return self

    def with_repeat(self: "SDP", repeat: int) -> "SDP":
        """Set parameter.

        Arguments:
        ---------
            repeat: parameter
        """
        self._repeat = repeat
        return self

    def _gain_ptr(self: "SDP", _: Geometry) -> GainPtr:
        size = len(self._amps)
        foci_ = np.ctypeslib.as_ctypes(np.array(self._foci).astype(ctypes.c_double))
        amps = np.ctypeslib.as_ctypes(np.fromiter((a.pascal for a in self._amps), dtype=float).astype(ctypes.c_double))
        ptr = self._backend._sdp(foci_, amps, size)
        if self._alpha is not None:
            ptr = self._backend._sdp_with_alpha(ptr, self._alpha)
        if self._lambda is not None:
            ptr = self._backend._sdp_with_lambda(ptr, self._lambda)
        if self._repeat is not None:
            ptr = self._backend._sdp_with_repeat(ptr, self._repeat)
        if self._constraint is not None:
            ptr = self._backend._sdp_with_constraint(ptr, self._constraint)
        return ptr
