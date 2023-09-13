'''
File: square.py
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


class Square(IModulation):
    _freq: int
    _low: Optional[float]
    _high: Optional[float]
    _duty: Optional[float]
    _freq_div: Optional[int]

    def __init__(self, freq: int):
        super().__init__()
        self._freq = freq
        self._low = None
        self._high = None
        self._duty = None
        self._freq_div = None

    def with_low(self, low: float) -> "Square":
        self._low = low
        return self

    def with_high(self, high: float) -> "Square":
        self._high = high
        return self

    def with_duty(self, duty: float) -> "Square":
        self._duty = duty
        return self

    def with_sampling_frequency_division(self, div: int) -> "Square":
        self._freq_div = div
        return self

    def with_sampling_frequency(self, freq: float) -> "Square":
        div = int(FPGA_SUB_CLK_FREQ / freq)
        return self.with_sampling_frequency_division(div)

    def with_sampling_period(self, period: timedelta) -> "Square":
        return self.with_sampling_frequency_division(int(FPGA_SUB_CLK_FREQ / 1000000000. * (period.total_seconds() * 1000. * 1000. * 1000.)))

    def modulation_ptr(self) -> ModulationPtr:
        ptr = Base().modulation_square(self._freq)
        if self._low is not None:
            ptr = Base().modulation_square_with_low(ptr, self._low)
        if self._high is not None:
            ptr = Base().modulation_square_with_high(ptr, self._high)
        if self._duty is not None:
            ptr = Base().modulation_square_with_duty(ptr, self._duty)
        if self._freq_div is not None:
            ptr = Base().modulation_square_with_sampling_frequency_division(
                ptr, self._freq_div
            )
        return ptr
