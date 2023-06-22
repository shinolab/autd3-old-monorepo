"""
File: primitive.py
Project: gain
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""

from abc import ABCMeta, abstractmethod
import numpy as np
from ctypes import byref, c_void_p, c_uint64, POINTER, memmove, sizeof, c_double
from typing import Optional

from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import ModulationPtr, FPGA_SUB_CLK_FREQ
from .modulation import IModulation


class Static(IModulation):
    _amp: Optional[float]

    def __init__(self):
        super().__init__()
        self._amp = None

    def with_amp(self, amp: float) -> "Static":
        self._amp = amp
        return self

    def modulation_ptr(self) -> ModulationPtr:
        ptr = Base().modulation_static()
        if self._amp is not None:
            ptr = Base().modulation_static_with_amp(ptr, self._amp)
        return ptr


class Sine(IModulation):
    _freq: int
    _amp: Optional[float]
    _offset: Optional[float]
    _freq_div: Optional[int]

    def __init__(self, freq: int):
        super().__init__()
        self._freq = freq
        self._amp = None
        self._offset = None
        self._freq_div = None

    def with_amp(self, amp: float) -> "Sine":
        self._amp = amp
        return self

    def with_offset(self, offset: float) -> "Sine":
        self._offset = offset
        return self

    def with_sampling_frequency_division(self, div: int) -> "Sine":
        self._freq_div = div
        return self

    def with_sampling_frequency(self, freq: float) -> "Sine":
        div = int(FPGA_SUB_CLK_FREQ / freq)
        return self.with_sampling_frequency_division(div)

    def modulation_ptr(self) -> ModulationPtr:
        ptr = Base().modulation_sine(self._freq)
        if self._amp is not None:
            ptr = Base().modulation_sine_with_amp(ptr, self._amp)
        if self._offset is not None:
            ptr = Base().modulation_sine_with_offset(ptr, self._offset)
        if self._freq_div is not None:
            ptr = Base().modulation_sine_with_sampling_frequency_division(
                ptr, self._freq_div
            )
        return ptr


class SinePressure(IModulation):
    _freq: int
    _amp: Optional[float]
    _offset: Optional[float]
    _freq_div: Optional[int]

    def __init__(self, freq: int):
        super().__init__()
        self._freq = freq
        self._amp = None
        self._offset = None
        self._freq_div = None

    def with_amp(self, amp: float) -> "SinePressure":
        self._amp = amp
        return self

    def with_offset(self, offset: float) -> "SinePressure":
        self._offset = offset
        return self

    def with_sampling_frequency_division(self, div: int) -> "SinePressure":
        self._freq_div = div
        return self

    def with_sampling_frequency(self, freq: float) -> "SinePressure":
        div = int(FPGA_SUB_CLK_FREQ / freq)
        return self.with_sampling_frequency_division(div)

    def modulation_ptr(self) -> ModulationPtr:
        ptr = Base().modulation_sine_pressure(self._freq)
        if self._amp is not None:
            ptr = Base().modulation_sine_pressure_with_amp(ptr, self._amp)
        if self._offset is not None:
            ptr = Base().modulation_sine_pressure_with_offset(ptr, self._offset)
        if self._freq_div is not None:
            ptr = Base().modulation_sine_pressure_with_sampling_frequency_division(
                ptr, self._freq_div
            )
        return ptr


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


class Modulation(IModulation, metaclass=ABCMeta):
    _freq_div: int

    def __init__(self, freq_div: int):
        super().__init__()
        self._freq_div = freq_div

    @abstractmethod
    def calc(self) -> np.ndarray:
        pass

    def modulation_ptr(self) -> ModulationPtr:
        data = self.calc()
        size = len(data)
        ptr = c_void_p()
        len_ = c_uint64()
        cap = c_uint64()
        Base().alloc_mod_buf(size, byref(ptr), byref(len_), byref(cap))
        memmove(
            ptr,
            data.ctypes.data_as(POINTER(c_double)),
            int(len_.value) * sizeof(c_double),
        )
        return Base().modulation_custom(
            self._freq_div, ptr, int(len_.value), int(cap.value)
        )
