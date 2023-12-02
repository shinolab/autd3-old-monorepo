"""
File: drive.py
Project: pyautd3
Created Date: 23/11/2023
Author: Shun Suzuki
-----
Last Modified: 23/11/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from .emit_intensity import EmitIntensity
from .phase import Phase


class Drive:
    """Phase and intensity."""

    _phase: Phase
    _intensity: EmitIntensity

    def __init__(self: "Drive", phase: Phase, intensity: int | EmitIntensity) -> None:
        self._phase = phase
        self._intensity = EmitIntensity._cast(intensity)

    @property
    def phase(self: "Drive") -> Phase:
        """Phase."""
        return self._phase

    @property
    def intensity(self: "Drive") -> EmitIntensity:
        """Emission intensity."""
        return self._intensity
