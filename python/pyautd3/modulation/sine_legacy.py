'''
File: sine_legacy.py
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


class SineLegacy(IModulationWithFreqDiv):
    """Sine wave modulation

    """

    _freq: float
    _amp: Optional[float]
    _offset: Optional[float]

    def __init__(self, freq: float):
        """Constructor

        The sine wave is defined as `amp/2 * sin(2π * freq * t) + offset`, where `t` is time, and `amp = 1`, `offset = 0.5` by default.

        Arguments:
        - `freq` - Frequency (Hz)
        """

        super().__init__()
        self._freq = freq
        self._amp = None
        self._offset = None

    def with_amp(self, amp: float) -> "SineLegacy":
        """Set amplitude

        Arguments:
        - `amp` - Peek to peek amplitude
        """

        self._amp = amp
        return self

    def with_offset(self, offset: float) -> "SineLegacy":
        """Set offset

        Arguments:
        - `offset` - Offset
        """

        self._offset = offset
        return self

    def modulation_ptr(self) -> ModulationPtr:
        ptr = Base().modulation_sine_legacy(self._freq)
        if self._amp is not None:
            ptr = Base().modulation_sine_legacy_with_amp(ptr, self._amp)
        if self._offset is not None:
            ptr = Base().modulation_sine_legacy_with_offset(ptr, self._offset)
        if self._freq_div is not None:
            ptr = Base().modulation_sine_legacy_with_sampling_frequency_division(
                ptr, self._freq_div
            )
        return ptr
