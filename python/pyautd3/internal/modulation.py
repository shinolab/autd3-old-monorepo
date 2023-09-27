'''
File: modulation.py
Project: modulation
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 26/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

'''


from abc import ABCMeta, abstractmethod
import ctypes
import numpy as np
from ctypes import create_string_buffer
from typing import Callable, Iterator

from pyautd3.native_methods.autd3capi import ModulationCachePtr
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import DatagramPtr, ModulationPtr

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
    _cache: ModulationCachePtr
    _buffer: np.ndarray

    def __init__(self, m: IModulation):
        err = create_string_buffer(256)
        cache = Base().modulation_with_cache(m.modulation_ptr(), err)
        if cache._0 is None:
            raise AUTDError(err)
        self._cache = cache

        n = int(Base().modulation_cache_get_buffer_size(self._cache))
        self._buffer = np.zeros(n, dtype=float)
        Base().modulation_cache_get_buffer(self._cache, np.ctypeslib.as_ctypes(self._buffer))

    @property
    def buffer(self) -> np.ndarray:
        return self._buffer

    def __getitem__(self, key: int) -> float:
        return self._buffer[key]

    def __iter__(self) -> Iterator[float]:
        return iter(self._buffer)

    def modulation_ptr(self) -> ModulationPtr:
        return Base().modulation_cache_into_modulation(self._cache)

    def __del__(self):
        Base().modulation_cache_delete(self._cache)


class Transform(IModulation):
    _m: IModulation

    def __init__(self, m: IModulation, f: Callable[[int, float], float]):
        self._m = m
        self._f_native = ctypes.CFUNCTYPE(ctypes.c_double, ctypes.c_void_p, ctypes.c_uint32, ctypes.c_double)(lambda _, i, d: f(int(i), float(d)))

    def modulation_ptr(self) -> ModulationPtr:
        return Base().modulation_with_transform(self._m.modulation_ptr(), self._f_native, None)  # type: ignore


class RadiationPressure(IModulation):
    _m: IModulation

    def __init__(self, m: IModulation):
        self._m = m

    def modulation_ptr(self) -> ModulationPtr:
        return Base().modulation_with_radiation_pressure(self._m.modulation_ptr())
