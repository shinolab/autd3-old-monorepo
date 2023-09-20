'''
File: gain.py
Project: internal
Created Date: 29/08/2023
Author: Shun Suzuki
-----
Last Modified: 20/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from abc import ABCMeta, abstractmethod
from functools import reduce
import numpy as np
from typing import Callable, Dict
from ctypes import create_string_buffer, POINTER

from pyautd3.autd import Datagram
from pyautd3.geometry import Geometry, Device, Transducer
from pyautd3.autd_error import AUTDError

from pyautd3.native_methods.autd3capi import Drive
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import DatagramPtr, GainPtr


class IGain(Datagram, metaclass=ABCMeta):
    def __init__(self):
        super().__init__()

    def ptr(self, geometry: Geometry) -> DatagramPtr:
        return Base().gain_into_datagram(self.gain_ptr(geometry))

    @abstractmethod
    def gain_ptr(self, geometry: Geometry) -> GainPtr:
        pass

    def with_cache(self):
        return Cache(self)

    def with_transform(self, f):
        return Transform(self, f)


class Cache(IGain):
    _g: IGain
    _cache: Dict[int, np.ndarray]

    def __init__(self, g: IGain):
        super().__init__()
        self._g = g
        self._cache = {}

    def gain_ptr(self, geometry: Geometry) -> GainPtr:
        device_indices = [dev.idx for dev in geometry]

        if len(self._cache) != len(device_indices) or any([idx not in self._cache for idx in device_indices]):
            err = create_string_buffer(256)
            res = Base().gain_calc(self._g.gain_ptr(geometry), geometry.ptr(), err)
            if res._0 is None:
                raise AUTDError(err)

            for dev in geometry:
                drives = np.zeros(dev.num_transducers, dtype=Drive)
                Base().gain_calc_get_result(res, drives.ctypes.data_as(POINTER(Drive)), dev.idx)  # type: ignore
                self._cache[dev.idx] = drives
            Base().gain_calc_free_result(res)

        return reduce(
            lambda acc, dev: Base().gain_custom_set(acc, dev.idx,
                                                    self._cache[dev.idx].ctypes.data_as(POINTER(Drive)),  # type: ignore
                                                    len(self._cache[dev.idx])),
            geometry,
            Base().gain_custom(),
        )


class Transform(IGain):
    _g: IGain
    _f: Callable[[Device, Transducer, Drive], Drive]

    def __init__(self, g: IGain, f: Callable[[Device, Transducer, Drive], Drive]):
        super().__init__()
        self._g = g
        self._f = f

    def gain_ptr(self, geometry: Geometry) -> GainPtr:
        err = create_string_buffer(256)
        res = Base().gain_calc(self._g.gain_ptr(geometry), geometry.ptr(), err)
        if res._0 is None:
            raise AUTDError(err)

        drives: Dict[int, np.ndarray] = {}
        for dev in geometry:
            d = np.zeros(dev.num_transducers, dtype=Drive)

            Base().gain_calc_get_result(res, d.ctypes.data_as(POINTER(Drive)), dev.idx)  # type: ignore
            for tr in dev:
                d[tr.local_idx] = np.void(self._f(dev, tr, Drive(d[tr.local_idx]["phase"], d[tr.local_idx]["amp"])))  # type: ignore
            drives[dev.idx] = d

        Base().gain_calc_free_result(res)

        return reduce(
            lambda acc, dev: Base().gain_custom_set(acc, dev.idx,
                                                    drives[dev.idx].ctypes.data_as(POINTER(Drive)),  # type: ignore
                                                    len(drives[dev.idx])),
            geometry,
            Base().gain_custom(),
        )
