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
from ctypes import c_double, create_string_buffer
from typing import Optional, Iterator

from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import ModulationPtr, FPGA_SUB_CLK_FREQ
from .modulation import IModulation
from pyautd3.autd_error import AUTDError
from pyautd3.native_methods.autd3capi_def import AUTD3_ERR


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
        return Base().modulation_custom(
            self._freq_div, np.ctypeslib.as_ctypes(data.astype(c_double)), size
        )


class Cache(IModulation):
    _freq_div: int
    _buffer: np.ndarray

    def __init__(self, m: IModulation):
        self._freq_div = m.sampling_frequency_division

        err = create_string_buffer(256)
        size = Base().modulation_size(m.modulation_ptr(), err)
        if size == AUTD3_ERR:
            raise AUTDError(err)
        self._buffer = np.zeros(int(size), dtype=c_double)
        bufp = np.ctypeslib.as_ctypes(self._buffer)
        if Base().modulation_calc(m.modulation_ptr(), bufp, err) == AUTD3_ERR:
            raise AUTDError(err)

    @property
    def buffer(self) -> np.ndarray:
        return self._buffer

    def __getitem__(self, key: int) -> float:
        return self._buffer[key]

    def __iter__(self) -> Iterator[float]:
        return iter(self._buffer)

    def modulation_ptr(self) -> ModulationPtr:
        bufp = np.ctypeslib.as_ctypes(self._buffer)
        return Base().modulation_custom(self._freq_div, bufp, len(self._buffer))


class RadiationPressure(IModulation):
    _freq_div: int
    _buffer: np.ndarray

    def __init__(self, m: IModulation):
        self._freq_div = m.sampling_frequency_division

        err = create_string_buffer(256)
        size = Base().modulation_size(m.modulation_ptr(), err)
        if size == AUTD3_ERR:
            raise AUTDError(err)
        buf = np.zeros(int(size), dtype=c_double)
        bufp = np.ctypeslib.as_ctypes(buf)
        if Base().modulation_calc(m.modulation_ptr(), bufp, err) == AUTD3_ERR:
            raise AUTDError(err)
        self._buffer = np.sqrt(buf)

    def modulation_ptr(self) -> ModulationPtr:
        bufp = np.ctypeslib.as_ctypes(self._buffer)
        return Base().modulation_custom(self._freq_div, bufp, len(self._buffer))
