"""
File: bessel.py
Project: gain
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 02/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


import numpy as np

from pyautd3.geometry import Geometry
from pyautd3.internal.gain import IGain
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import GainPtr


class Bessel(IGain):
    """Gain to produce a Bessel beam."""

    _p: np.ndarray
    _d: np.ndarray
    _theta: float
    _amp: float | None

    def __init__(self: "Bessel", pos: np.ndarray, direction: np.ndarray, theta_z: float) -> None:
        """Constructor.

        Arguments:
        ---------
            pos: Start point of the beam (the apex of the conical wavefront of the beam)
            direction: Direction of the beam
            theta_z: Angle between the conical wavefront of the beam and the plane normal to `dir`
        """
        super().__init__()
        self._p = pos
        self._d = direction
        self._theta = theta_z
        self._amp = None

    def with_amp(self: "Bessel", amp: float) -> "Bessel":
        """Set amplitude.

        Arguments:
        ---------
            amp: Normalized amplitude (from 0 to 1)
        """
        self._amp = amp
        return self

    def _gain_ptr(self: "Bessel", _: Geometry) -> GainPtr:
        ptr = Base().gain_bessel(
            self._p[0],
            self._p[1],
            self._p[2],
            self._d[0],
            self._d[1],
            self._d[2],
            self._theta,
        )
        if self._amp is not None:
            ptr = Base().gain_bessel_with_amp(ptr, self._amp)
        return ptr
