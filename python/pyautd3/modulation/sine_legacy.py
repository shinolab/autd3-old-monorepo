'''
File: sine_legacy.py
Project: modulation
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 29/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from datetime import timedelta
from typing import Optional
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import ModulationPtr, FPGA_SUB_CLK_FREQ
from pyautd3.internal.modulation import IModulation


class SineLegacy(IModulation):
    """Sine wave modulation

    """

    _freq: float
    _amp: Optional[float]
    _offset: Optional[float]
    _freq_div: Optional[int]

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
        self._freq_div = None

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

    def with_sampling_frequency_division(self, div: int) -> "SineLegacy":
        """Set sampling frequency division

        Arguments:
        - `div` - Sampling frequency division.
                The sampling frequency will be `pyautd3.AUTD3.fpga_sub_clk_freq()` / `div`.
        """

        self._freq_div = div
        return self

    def with_sampling_frequency(self, freq: float) -> "SineLegacy":
        """Set sampling frequency

        Arguments:
        - `freq` - Sampling frequency.
                The sampling frequency closest to `freq` from the possible sampling frequencies is set.
        """

        div = int(FPGA_SUB_CLK_FREQ / freq)
        return self.with_sampling_frequency_division(div)

    def with_sampling_period(self, period: timedelta) -> "SineLegacy":
        """Set sampling period

        Arguments:
        - `period` - Sampling period.
                The sampling period closest to `period` from the possible sampling periods is set.
        """

        return self.with_sampling_frequency_division(int(FPGA_SUB_CLK_FREQ / 1000000000. * (period.total_seconds() * 1000. * 1000. * 1000.)))

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
