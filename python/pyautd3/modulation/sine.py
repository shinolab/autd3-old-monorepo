'''
File: sine.py
Project: modulation
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from typing import Optional
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import ModulationPtr
from pyautd3.internal.modulation import IModulationWithFreqDiv


class Sine(IModulationWithFreqDiv):
    """Sine wave modulation

    """

    _freq: int
    _amp: Optional[float]
    _offset: Optional[float]
    _phase: Optional[float]

    def __init__(self, freq: int):
        """Constructor

        The sine wave is defined as `amp/2 * sin(2π * freq * t + phase) + offset`,
        where `t` is time, and `amp = 1`, `phase = 0`, `offset = 0.5` by default.

        Arguments:
        - `freq` - Frequency (Hz)
        """

        super().__init__()
        self._freq = freq
        self._amp = None
        self._offset = None
        self._phase = None

    def with_amp(self, amp: float) -> "Sine":
        """Set amplitude

        Arguments:
        - `amp` - Peek to peek amplitude
        """

        self._amp = amp
        return self

    def with_offset(self, offset: float) -> "Sine":
        """Set offset

        Arguments:
        - `offset` - Offset
        """

        self._offset = offset
        return self

    def with_phase(self, phase: float) -> "Sine":
        """Set phase

        Arguments:
        - `phase` - Phase (from 0 to 2π)
        """

        self._phase = phase
        return self

    def modulation_ptr(self) -> ModulationPtr:
        ptr = Base().modulation_sine(self._freq)
        if self._amp is not None:
            ptr = Base().modulation_sine_with_amp(ptr, self._amp)
        if self._offset is not None:
            ptr = Base().modulation_sine_with_offset(ptr, self._offset)
        if self._phase is not None:
            ptr = Base().modulation_sine_with_phase(ptr, self._phase)
        if self._freq_div is not None:
            ptr = Base().modulation_sine_with_sampling_frequency_division(
                ptr, self._freq_div
            )
        return ptr
