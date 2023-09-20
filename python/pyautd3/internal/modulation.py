"""
File: modulation.py
Project: modulation
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
from typing import Callable, Iterator

from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import DatagramPtr, ModulationPtr
from pyautd3.native_methods.autd3capi_def import AUTD3_ERR

from pyautd3.autd_error import AUTDError
from pyautd3.autd import Datagram
from pyautd3.geometry import Geometry, AUTD3


class IModulation(Datagram, metaclass=ABCMeta):
    def __init__(self):
        super().__init__()

    def ptr(self, _: Geometry) -> DatagramPtr:
        return Base().modulation_into_datagram(self.modulation_ptr())

    @property
    def sampling_frequency_division(self) -> int:
        return int(Base().modulation_sampling_frequency_division(self.modulation_ptr()))

    @property
    def sampling_frequency(self) -> float:
        return AUTD3.fpga_sub_clk_freq() / self.sampling_frequency_division

    def __len__(self):
        err = create_string_buffer(256)
        n = Base().modulation_size(self.modulation_ptr(), err)
        if n < 0:
            raise AUTDError(err)
        return int(n)

    @abstractmethod
    def modulation_ptr(self) -> ModulationPtr:
        pass

    def with_cache(self):
        return Cache(self)

    def with_radiation_pressure(self):
        return RadiationPressure(self)

    def with_transform(self, f: Callable[[int, float], float]):
        return Transform(self, f)


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


class Transform(IModulation):
    _freq_div: int
    _buffer: np.ndarray

    def __init__(self, m: IModulation, f: Callable[[int, float], float]):
        self._freq_div = m.sampling_frequency_division

        err = create_string_buffer(256)
        size = Base().modulation_size(m.modulation_ptr(), err)
        if size == AUTD3_ERR:
            raise AUTDError(err)
        self._buffer = np.zeros(int(size), dtype=c_double)
        bufp = np.ctypeslib.as_ctypes(self._buffer)
        if Base().modulation_calc(m.modulation_ptr(), bufp, err) == AUTD3_ERR:
            raise AUTDError(err)

        for i in range(len(self._buffer)):
            self._buffer[i] = f(i, self._buffer[i])

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
