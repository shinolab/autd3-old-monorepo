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


import functools
import numpy as np
from typing import Optional, List, Callable, Tuple, TypeVar, Generic, Dict
from abc import ABCMeta, abstractmethod
from ctypes import POINTER, c_int32, c_uint32

from pyautd3.native_methods.autd3capi import Drive
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import GainPtr
from pyautd3.geometry import Transducer, Geometry, Device
from .gain import IGain


class Focus(IGain):
    _p: np.ndarray
    _amp: Optional[float]

    def __init__(self, pos: np.ndarray):
        super().__init__()
        self._p = pos
        self._amp = None

    def with_amp(self, amp: float) -> "Focus":
        self._amp = amp
        return self

    def gain_ptr(self, _: Geometry) -> GainPtr:
        ptr = Base().gain_focus(self._p[0], self._p[1], self._p[2])
        if self._amp is not None:
            ptr = Base().gain_focus_with_amp(ptr, self._amp)
        return ptr


class Bessel(IGain):
    _p: np.ndarray
    _d: np.ndarray
    _theta: float
    _amp: Optional[float]

    def __init__(self, pos: np.ndarray, dir: np.ndarray, theta_z: float):
        super().__init__()
        self._p = pos
        self._d = dir
        self._theta = theta_z
        self._amp = None

    def with_amp(self, amp: float) -> "Bessel":
        self._amp = amp
        return self

    def gain_ptr(self, _: Geometry) -> GainPtr:
        ptr = Base().gain_bessel(
            self._p[0],
            self._p[1],
            self._p[2],
            self._d[0],
            self._d[1],
            self._d[2],
            self._theta,
        )
        if self._amp is not None:
            ptr = Base().gain_bessel_with_amp(ptr, self._amp)
        return ptr


class Plane(IGain):
    _d: np.ndarray
    _amp: Optional[float]

    def __init__(self, dir: np.ndarray):
        super().__init__()
        self._d = dir
        self._amp = None

    def gain_ptr(self, _: Geometry) -> GainPtr:
        ptr = Base().gain_plane(self._d[0], self._d[1], self._d[2])
        if self._amp is not None:
            ptr = Base().gain_plane_with_amp(ptr, self._amp)
        return ptr


class Null(IGain):
    def __init__(self):
        super().__init__()

    def gain_ptr(self, _: Geometry) -> GainPtr:
        return Base().gain_null()


K = TypeVar("K")


class Group(IGain, Generic[K]):
    _map: Dict[K, IGain]
    _f: Callable[[Device, Transducer], Optional[K]]

    def __init__(self, f: Callable[[Device, Transducer], Optional[K]]):
        super().__init__()
        self._map = {}
        self._f = f

    def set(self, key: K, gain: IGain) -> "Group":
        self._map[key] = gain
        return self

    def gain_ptr(self, geometry: Geometry) -> GainPtr:

        keymap: Dict[K, int] = {}

        device_indices = np.array([dev.idx for dev in geometry])

        map = Base().gain_group_create_map(np.ctypeslib.as_ctypes(device_indices.astype(c_uint32)), len(device_indices))
        k: int = 0
        for dev in geometry:
            m = np.zeros(dev.num_transducers, dtype=np.int32)
            for tr in dev:
                key = self._f(dev, tr)
                if key is not None:
                    if key not in keymap:
                        keymap[key] = k
                        k += 1
                    m[tr.local_idx] = keymap[key]
                else:
                    m[tr.local_idx] = -1
            map = Base().gain_group_map_set(map, dev.idx, np.ctypeslib.as_ctypes(m.astype(c_int32)))

        keys: np.ndarray = np.ndarray(len(self._map), dtype=np.int32)
        values: np.ndarray = np.ndarray(len(self._map), dtype=GainPtr)
        for i, (key, value) in enumerate(self._map.items()):
            keys[i] = keymap[key]
            values[i]["_0"] = value.gain_ptr(geometry)._0
        return Base().gain_group(
            map,
            np.ctypeslib.as_ctypes(keys.astype(c_int32)),
            values.ctypes.data_as(POINTER(GainPtr)),  # type: ignore
            len(keys),
        )


class TransTest(IGain):
    _data: List[Tuple[int, int, float, float]]

    def __init__(self):
        super().__init__()
        self._data = []

    def set(self, dev_idx: int, tr_idx: int, amp: float, phase: float):
        self._data.append((dev_idx, tr_idx, phase, amp))

    def gain_ptr(self, _: Geometry) -> GainPtr:
        return functools.reduce(
            lambda acc, v: Base().gain_transducer_test_set(acc, v[0], v[1], v[2], v[3]),
            self._data,
            Base().gain_transducer_test(),
        )


class Gain(IGain, metaclass=ABCMeta):
    @abstractmethod
    def calc(self, geometry: Geometry) -> Dict[int, np.ndarray]:
        pass

    def gain_ptr(self, geometry: Geometry) -> GainPtr:
        drives = self.calc(geometry)
        return functools.reduce(
            lambda acc, dev: Base().gain_custom_set(acc, dev.idx,
                                                    drives[dev.idx].ctypes.data_as(POINTER(GainPtr)),  # type: ignore
                                                    len(drives[dev.idx])),
            geometry,
            Base().gain_custom(),
        )

    @staticmethod
    def transform(geometry: Geometry, f: Callable[[Device, Transducer], Drive]) -> Dict[int, np.ndarray]:
        return dict(
            map(
                lambda dev: (dev.idx, np.fromiter(map(lambda tr: np.void(f(dev, tr)), dev), dtype=Drive)),  # type: ignore
                geometry,
            )
        )
