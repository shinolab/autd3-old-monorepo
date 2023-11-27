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


from pyautd3.internal.modulation import IModulationWithFreqDiv
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import ModulationPtr


class Sine(IModulationWithFreqDiv):
    """Sine wave modulation."""

    _freq: int
    _amp: float | None
    _offset: float | None
    _phase: float | None

    def __init__(self: "Sine", freq: int) -> None:
        """Constructor.

        The sine wave is defined as `amp/2 * sin(2π * freq * t + phase) + offset`,
        where `t` is time, and `amp = 1`, `phase = 0`, `offset = 0.5` by default.

        Arguments:
        ---------
            freq: Frequency (Hz)
        """
        super().__init__()
        self._freq = freq
        self._amp = None
        self._offset = None
        self._phase = None

    def with_amp(self: "Sine", amp: float) -> "Sine":
        """Set amplitude.

        Arguments:
        ---------
            amp: Amplitude
        """
        self._amp = amp
        return self

    def with_offset(self: "Sine", offset: float) -> "Sine":
        """Set offset.

        Arguments:
        ---------
            offset: Offset
        """
        self._offset = offset
        return self

    def with_phase(self: "Sine", phase: float) -> "Sine":
        """Set phase.

        Arguments:
        ---------
            phase: Phase (from 0 to 2π)
        """
        self._phase = phase
        return self

    def _modulation_ptr(self: "Sine") -> ModulationPtr:
        ptr = Base().modulation_sine(self._freq)
        if self._amp is not None:
            ptr = Base().modulation_sine_with_amp(ptr, self._amp)
        if self._offset is not None:
            ptr = Base().modulation_sine_with_offset(ptr, self._offset)
        if self._phase is not None:
            ptr = Base().modulation_sine_with_phase(ptr, self._phase)
        if self._config is not None:
            ptr = Base().modulation_sine_with_sampling_config(ptr, self._config._internal)
        return ptr
