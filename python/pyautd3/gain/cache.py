"""
File: cache.py
Project: gain
Created Date: 10/10/2023
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""

from ctypes import POINTER
from functools import reduce
from typing import Generic, TypeVar

import numpy as np

from pyautd3.geometry import Geometry
from pyautd3.internal.gain import IGain
from pyautd3.internal.utils import _validate_ptr
from pyautd3.native_methods.autd3capi import Drive
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import GainPtr

G = TypeVar("G", bound=IGain)


class Cache(IGain, Generic[G]):
    """Gain to cache the result of calculation."""

    _g: G
    _cache: dict[int, np.ndarray]

    def __init__(self: "Cache", g: G) -> None:
        super().__init__()
        self._g = g
        self._cache = {}

    def _gain_ptr(self: "Cache", geometry: Geometry) -> GainPtr:
        device_indices = [dev.idx for dev in geometry.devices()]

        if len(self._cache) != len(device_indices) or any(idx not in self._cache for idx in device_indices):
            res = _validate_ptr(Base().gain_calc(self._g._gain_ptr(geometry), geometry._geometry_ptr()))
            for dev in geometry.devices():
                drives = np.zeros(dev.num_transducers, dtype=Drive)
                Base().gain_calc_get_result(res, drives.ctypes.data_as(POINTER(Drive)), dev.idx)  # type: ignore[arg-type]
                self._cache[dev.idx] = drives
            Base().gain_calc_free_result(res)

        return reduce(
            lambda acc, dev: Base().gain_custom_set(
                acc,
                dev.idx,
                self._cache[dev.idx].ctypes.data_as(POINTER(Drive)),  # type: ignore[arg-type]
                len(self._cache[dev.idx]),
            ),
            geometry.devices(),
            Base().gain_custom(),
        )

    def drives(self: "Cache") -> dict[int, np.ndarray]:
        """Get cached drives."""
        return self._cache


def __with_cache(self: G) -> Cache:
    """Cache the result of calculation."""
    return Cache(self)


IGain.with_cache = __with_cache  # type: ignore[method-assign]
