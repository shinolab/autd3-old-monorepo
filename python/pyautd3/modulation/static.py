"""
File: static.py
Project: modulation
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 29/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""

from pyautd3.emit_intensity import EmitIntensity
from pyautd3.internal.modulation import IModulation
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import ModulationPtr


class Static(IModulation):
    """Without modulation."""

    _intensity: EmitIntensity | None

    def __init__(self: "Static") -> None:
        super().__init__()
        self._intensity = None

    def with_intensity(self: "Static", intensity: int | EmitIntensity) -> "Static":
        """Set intensity.

        Arguments:
        ---------
            intensity: Emission intensity
        """
        self._intensity = EmitIntensity._cast(intensity)
        return self

    def _modulation_ptr(self: "Static") -> ModulationPtr:
        ptr = Base().modulation_static()
        if self._intensity is not None:
            ptr = Base().modulation_static_with_intensity(ptr, self._intensity.value)
        return ptr
