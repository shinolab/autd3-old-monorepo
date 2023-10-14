'''
File: cache.py
Project: modulation
Created Date: 10/10/2023
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


import numpy as np
from ctypes import create_string_buffer
from typing import Iterator

from pyautd3.native_methods.autd3capi import ModulationCachePtr
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import ModulationPtr

from pyautd3.autd_error import AUTDError
from pyautd3.internal.modulation import IModulation


class Cache(IModulation):
    """Modulation to cache the result of calculation

    """

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
        """get cached modulation data

        """

        return self._buffer

    def __getitem__(self, key: int) -> float:
        return self._buffer[key]

    def __iter__(self) -> Iterator[float]:
        return iter(self._buffer)

    def modulation_ptr(self) -> ModulationPtr:
        return Base().modulation_cache_into_modulation(self._cache)

    def __del__(self):
        Base().modulation_cache_delete(self._cache)


def __with_cache(self):
    """Cache the result of calculation

    """

    return Cache(self)


IModulation.with_cache = __with_cache  # type: ignore
