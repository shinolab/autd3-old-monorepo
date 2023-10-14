'''
File: square.py
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


class Square(IModulationWithFreqDiv):
    """Square wave modulation

    """

    _freq: int
    _low: Optional[float]
    _high: Optional[float]
    _duty: Optional[float]

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
