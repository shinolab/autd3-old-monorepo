"""
File: focus.py
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

from pyautd3.emit_intensity import EmitIntensity
from pyautd3.geometry import Geometry
from pyautd3.internal.gain import IGain
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import GainPtr


class Focus(IGain):
    """Gain to produce a focal point."""

    _p: np.ndarray
    _amp: EmitIntensity | None

    def __init__(self: "Focus", pos: np.ndarray) -> None:
        """Constructor.

        Arguments:
        ---------
            pos: Position of the focal point
        """
        super().__init__()
        self._p = pos
        self._amp = None

    def with_amp(self: "Focus", amp: int | float | EmitIntensity) -> "Focus":  # noqa: PYI041
        """Set amplitude.

        Arguments:
        ---------
            amp: pulse width (int) | normalized amplitude (float) | EmitIntensity
        """
        self._amp = EmitIntensity._cast(amp)
        return self

    def _gain_ptr(self: "Focus", _: Geometry) -> GainPtr:
        ptr = Base().gain_focus(self._p[0], self._p[1], self._p[2])
        if self._amp is not None:
            ptr = Base().gain_focus_with_amp(ptr, self._amp.pulse_width)
        return ptr
