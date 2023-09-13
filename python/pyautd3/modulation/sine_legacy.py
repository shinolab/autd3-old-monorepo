'''
File: sine_legacy.py
Project: modulation
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 14/09/2023
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
    _freq: float
    _amp: Optional[float]
    _offset: Optional[float]
    _freq_div: Optional[int]

    def __init__(self, freq: float):
        super().__init__()
        self._freq = freq
        self._amp = None
        self._offset = None
        self._freq_div = None

    def with_amp(self, amp: float) -> "SineLegacy":
        self._amp = amp
        return self

    def with_offset(self, offset: float) -> "SineLegacy":
        self._offset = offset
        return self

    def with_sampling_frequency_division(self, div: int) -> "SineLegacy":
        self._freq_div = div
        return self

    def with_sampling_frequency(self, freq: float) -> "SineLegacy":
        div = int(FPGA_SUB_CLK_FREQ / freq)
        return self.with_sampling_frequency_division(div)

    def with_sampling_period(self, period: timedelta) -> "SineLegacy":
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
