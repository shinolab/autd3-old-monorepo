"""
File: lm.py
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


class LM(HoloWithBackend):
    """Gain to produce multiple foci with Levenberg-Marquardt algorithm.

    References
    ----------
    - Levenberg, Kenneth. "A method for the solution of certain non-linear problems in least squares,"
        Quarterly of applied mathematics 2.2 (1944): 164-168.
    - Marquardt, Donald W. "An algorithm for least-squares estimation of nonlinear parameters,"
        Journal of the society for Industrial and Applied Mathematics 11.2 (1963): 431-441.
    - K.Madsen, H.Nielsen, and O.Tingleff, “Methods for non-linear least squares problems (2nd ed.),” 2004.
    """

    _eps1: float | None
    _eps2: float | None
    _tau: float | None
    _kmax: int | None
    _initial: list[float]

    def __init__(self: "LM", backend: Backend) -> None:
        super().__init__(backend)
        self._eps1 = None
        self._eps2 = None
        self._tau = None
        self._kmax = None
        self._initial = []

    def with_eps1(self: "LM", eps1: float) -> "LM":
        """Set parameter.

        Arguments:
        ---------
            eps1: parameter
        """
        self._eps1 = eps1
        return self

    def with_eps2(self: "LM", eps2: float) -> "LM":
        """Set parameter.

        Arguments:
        ---------
            eps2: parameter
        """
        self._eps2 = eps2
        return self

    def with_tau(self: "LM", tau: float) -> "LM":
        """Set parameter.

        Arguments:
        ---------
            tau: parameter
        """
        self._tau = tau
        return self

    def with_kmax(self: "LM", kmax: int) -> "LM":
        """Set parameter.

        Arguments:
        ---------
            kmax: parameter
        """
        self._kmax = kmax
        return self

    def with_initial(self: "LM", initial: list[float]) -> "LM":
        """Set parameter.

        Arguments:
        ---------
            initial: parameter
        """
        self._initial = initial
        return self

    def _gain_ptr(self: "LM", _: Geometry) -> GainPtr:
        size = len(self._amps)
        foci_ = np.ctypeslib.as_ctypes(np.array(self._foci).astype(ctypes.c_double))
        amps = np.ctypeslib.as_ctypes(np.fromiter((a.pascal for a in self._amps), dtype=float).astype(ctypes.c_double))
        ptr = self._backend._lm(foci_, amps, size)
        if self._eps1 is not None:
            ptr = self._backend._lm_with_eps1(ptr, self._eps1)
        if self._eps2 is not None:
            ptr = self._backend._lm_with_eps2(ptr, self._eps2)
        if self._tau is not None:
            ptr = self._backend._lm_with_tau(ptr, self._tau)
        if self._kmax is not None:
            ptr = self._backend._lm_with_kmax(ptr, self._kmax)
        if len(self._initial) > 0:
            initial_ = np.ctypeslib.as_ctypes(np.array(self._initial).astype(ctypes.c_double))
            ptr = self._backend._lm_with_initial(ptr, initial_, len(self._initial))
        if self._constraint is not None:
            ptr = self._backend._lm_with_constraint(ptr, self._constraint)
        return ptr
