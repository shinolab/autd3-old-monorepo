"""
File: sine.py
Project: modulation
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""

from pyautd3.emit_intensity import EmitIntensity
from pyautd3.internal.modulation import IModulationWithSamplingConfig
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi import SamplingMode
from pyautd3.native_methods.autd3capi_def import ModulationPtr


class Sine(IModulationWithSamplingConfig):
    """Sine wave modulation."""

    _freq: float
    _intensity: EmitIntensity | None
    _offset: EmitIntensity | None
    _phase: float | None
    _mode: SamplingMode | None

    def __init__(self: "Sine", freq: float) -> None:
        """Constructor.

        The sine wave is defined as `amp/2 * sin(2π * freq * t + phase) + offset`,
        where `t` is time, and `amp = EmitIntensity.maximum()`, `phase = 0`, `offset = EmitIntensity.maximum()/2` by default.

        Arguments:
        ---------
            freq: Frequency (Hz)
        """
        super().__init__()
        self._freq = freq
        self._intensity = None
        self._offset = None
        self._phase = None
        self._mode = None

    def with_intensity(self: "Sine", intensity: int | EmitIntensity) -> "Sine":
        """Set intensity.

        Arguments:
        ---------
            intensity: Intensity
        """
        self._intensity = EmitIntensity._cast(intensity)
        return self

    def with_offset(self: "Sine", offset: int | EmitIntensity) -> "Sine":
        """Set offset.

        Arguments:
        ---------
            offset: Offset
        """
        self._offset = EmitIntensity._cast(offset)
        return self

    def with_phase(self: "Sine", phase: float) -> "Sine":
        """Set phase.

        Arguments:
        ---------
            phase: Phase (from 0 to 2π)
        """
        self._phase = phase
        return self

    def with_mode(self: "Sine", mode: SamplingMode) -> "Sine":
        """Set sampling mode.

        Arguments:
        ---------
            mode: Sampling mode
        """
        self._mode = mode
        return self

    def _modulation_ptr(self: "Sine") -> ModulationPtr:
        ptr = Base().modulation_sine(self._freq)
        if self._intensity is not None:
            ptr = Base().modulation_sine_with_intensity(ptr, self._intensity.value)
        if self._offset is not None:
            ptr = Base().modulation_sine_with_offset(ptr, self._offset.value)
        if self._phase is not None:
            ptr = Base().modulation_sine_with_phase(ptr, self._phase)
        if self._config is not None:
            ptr = Base().modulation_sine_with_sampling_config(ptr, self._config._internal)
        if self._mode is not None:
            ptr = Base().modulation_sine_with_mode(ptr, self._mode)
        return ptr
