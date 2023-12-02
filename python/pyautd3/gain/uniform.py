"""
File: uniform.py
Project: gain
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 29/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from pyautd3.emit_intensity import EmitIntensity
from pyautd3.geometry import Geometry
from pyautd3.internal.gain import IGain
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import GainPtr
from pyautd3.phase import Phase


class Uniform(IGain):
    """Gain with uniform amplitude and phase."""

    _intensity: EmitIntensity
    _phase: Phase | None

    def __init__(self: "Uniform", intensity: int | EmitIntensity) -> None:
        """Constructor.

        Arguments:
        ---------
            intensity: Emission intensity
        """
        super().__init__()
        self._intensity = EmitIntensity._cast(intensity)
        self._phase = None

    def with_phase(self: "Uniform", phase: Phase) -> "Uniform":
        """Set phase.

        Arguments:
        ---------
            phase: Phase
        """
        self._phase = phase
        return self

    def _gain_ptr(self: "Uniform", _: Geometry) -> GainPtr:
        ptr = Base().gain_uniform(self._intensity.value)
        if self._phase is not None:
            ptr = Base().gain_uniform_with_phase(ptr, self._phase.value)
        return ptr
