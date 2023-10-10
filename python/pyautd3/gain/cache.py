'''
File: cache.py
Project: gain
Created Date: 10/10/2023
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''

from functools import reduce
import numpy as np
from typing import Dict
from ctypes import create_string_buffer, POINTER

from pyautd3.geometry import Geometry
from pyautd3.autd_error import AUTDError
from pyautd3.internal.gain import IGain

from pyautd3.native_methods.autd3capi import Drive
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import GainPtr


class Cache(IGain):
    """Gain to cache the result of calculation

    """

    _g: IGain
    _cache: Dict[int, np.ndarray]

    def __init__(self, g: IGain):
        super().__init__()
        self._g = g
        self._cache = {}

    def gain_ptr(self, geometry: Geometry) -> GainPtr:
        device_indices = [dev.idx for dev in geometry.devices()]

        if len(self._cache) != len(device_indices) or any([idx not in self._cache for idx in device_indices]):
            err = create_string_buffer(256)
            res = Base().gain_calc(self._g.gain_ptr(geometry), geometry.ptr(), err)
            if res._0 is None:
                raise AUTDError(err)

            for dev in geometry.devices():
                drives = np.zeros(dev.num_transducers, dtype=Drive)
                Base().gain_calc_get_result(res, drives.ctypes.data_as(POINTER(Drive)), dev.idx)  # type: ignore
                self._cache[dev.idx] = drives
            Base().gain_calc_free_result(res)

        return reduce(
            lambda acc, dev: Base().gain_custom_set(acc, dev.idx,
                                                    self._cache[dev.idx].ctypes.data_as(POINTER(Drive)),  # type: ignore
                                                    len(self._cache[dev.idx])),
            geometry.devices(),
            Base().gain_custom(),
        )

    def drives(self) -> Dict[int, np.ndarray]:
        """get cached drives

        """

        return self._cache


def __with_cache(self) -> Cache:
    """Cache the result of calculation

    """

    return Cache(self)


IGain.with_cache = __with_cache
