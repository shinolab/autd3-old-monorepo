"""
File: square.py
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


class Square(IModulationWithSamplingConfig):
    """Square wave modulation."""

    _freq: float
    _low: EmitIntensity | None
    _high: EmitIntensity | None
    _duty: float | None
    _mode: SamplingMode | None

    def __init__(self: "Square", freq: float) -> None:
        """Constructor.

        Arguments:
        ---------
            freq: Frequency (Hz)
        """
        super().__init__()
        self._freq = freq
        self._low = None
        self._high = None
        self._duty = None
        self._mode = None

    def with_low(self: "Square", low: int | EmitIntensity) -> "Square":
        """Set low level intensity.

        Arguments:
        ---------
            low: Low level intensity
        """
        self._low = EmitIntensity._cast(low)
        return self

    def with_high(self: "Square", high: int | EmitIntensity) -> "Square":
        """Set high level intensity.

        Arguments:
        ---------
            high: High level intensity
        """
        self._high = EmitIntensity._cast(high)
        return self

    def with_duty(self: "Square", duty: float) -> "Square":
        """Set duty ratio which is defined as `Th / (Th + Tl)`, where `Th` is high level duration and `Tl` is low level duration.

        Arguments:
        ---------
            duty: Duty ratio (from 0 to 1)
        """
        self._duty = duty
        return self

    def with_mode(self: "Square", mode: SamplingMode) -> "Square":
        """Set sampling mode.

        Arguments:
        ---------
            mode: Sampling mode
        """
        self._mode = mode
        return self

    def _modulation_ptr(self: "Square") -> ModulationPtr:
        ptr = Base().modulation_square(self._freq)
        if self._low is not None:
            ptr = Base().modulation_square_with_low(ptr, self._low.value)
        if self._high is not None:
            ptr = Base().modulation_square_with_high(ptr, self._high.value)
        if self._duty is not None:
            ptr = Base().modulation_square_with_duty(ptr, self._duty)
        if self._config is not None:
            ptr = Base().modulation_square_with_sampling_config(ptr, self._config._internal)
        if self._mode is not None:
            ptr = Base().modulation_square_with_mode(ptr, self._mode)
        return ptr
