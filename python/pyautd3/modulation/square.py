'''
File: square.py
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


class Square(IModulation):
    """Square wave modulation

    """

    _freq: int
    _low: Optional[float]
    _high: Optional[float]
    _duty: Optional[float]
    _freq_div: Optional[int]

    def __init__(self, freq: int):
        """Constructor

        Arguments:
        - `freq` - Frequency (Hz)
        """

        super().__init__()
        self._freq = freq
        self._low = None
        self._high = None
        self._duty = None
        self._freq_div = None

    def with_low(self, low: float) -> "Square":
        """Set low level amplitude

        Arguments:
        - `low` - Low level amplitude (from 0 to 1)
        """

        self._low = low
        return self

    def with_high(self, high: float) -> "Square":
        """Set high level amplitude

        Arguments:
        - `high` - High level amplitude (from 0 to 1)
        """

        self._high = high
        return self

    def with_duty(self, duty: float) -> "Square":
        """Set duty ratio which is defined as `Th / (Th + Tl)`, where `Th` is high level duration and `Tl` is low level duration.

        Arguments:
        - `duty` - Duty ratio (from 0 to 1)
        """

        self._duty = duty
        return self

    def with_sampling_frequency_division(self, div: int) -> "Square":
        """Set sampling frequency division

        Arguments:
        - `div` - Sampling frequency division.
                The sampling frequency will be `pyautd3.AUTD3.fpga_sub_clk_freq()` / `div`.
        """

        self._freq_div = div
        return self

    def with_sampling_frequency(self, freq: float) -> "Square":
        """Set sampling frequency

        Arguments:
        - `freq` - Sampling frequency.
                The sampling frequency closest to `freq` from the possible sampling frequencies is set.
        """

        div = int(FPGA_SUB_CLK_FREQ / freq)
        return self.with_sampling_frequency_division(div)

    def with_sampling_period(self, period: timedelta) -> "Square":
        """Set sampling period

        Arguments:
        - `period` - Sampling period.
                The sampling period closest to `period` from the possible sampling periods is set.
        """

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
