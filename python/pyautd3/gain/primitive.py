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
from typing import Optional, List, Callable, TypeVar, Generic, Dict
from abc import ABCMeta, abstractmethod
from ctypes import POINTER, c_int32

from pyautd3.native_methods.autd3capi import Drive
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import GainPtr
from pyautd3.geometry import Geometry, Transducer
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


class GroupByDevice(IGain, Generic[K]):
    _map: Dict[K, IGain]
    _f: Callable[[int], Optional[K]]

    def __init__(self, f: Callable[[int], Optional[K]]):
        super().__init__()
        self._map = {}
        self._f = f

    def set(self, key: K, gain: IGain) -> "GroupByDevice":
        self._map[key] = gain
        return self

    def gain_ptr(self, geometry: Geometry) -> GainPtr:
        keymap: Dict[K, int] = {}
        map: np.ndarray = np.ndarray(geometry.num_devices, dtype=np.int32)
        k: int = 0
        for dev in range(geometry.num_devices):
            key = self._f(dev)
            if key is not None:
                if key not in keymap:
                    keymap[key] = k
                    k += 1
                map[dev] = keymap[key]
            else:
                map[dev] = -1
        keys: np.ndarray = np.ndarray(len(self._map), dtype=np.int32)
        values: np.ndarray = np.ndarray(len(self._map), dtype=GainPtr)
        for i, (key, value) in enumerate(self._map.items()):
            keys[i] = keymap[key]
            values[i]["_0"] = value.gain_ptr(geometry)._0
        return Base().gain_group_by_device(
            np.ctypeslib.as_ctypes(map.astype(c_int32)),
            len(map),
            np.ctypeslib.as_ctypes(keys.astype(c_int32)),
            values.ctypes.data_as(POINTER(GainPtr)),  # type: ignore
            len(keys),
        )


class GroupByTransducer(IGain, Generic[K]):
    _map: Dict[K, IGain]
    _f: Callable[[Transducer], Optional[K]]

    def __init__(self, f: Callable[[Transducer], Optional[K]]):
        super().__init__()
        self._map = {}
        self._f = f

    def set(self, key: K, gain: IGain) -> "GroupByTransducer":
        self._map[key] = gain
        return self

    def gain_ptr(self, geometry: Geometry) -> GainPtr:
        keymap: Dict[K, int] = {}
        map: np.ndarray = np.ndarray(geometry.num_transducers, dtype=np.int32)
        k: int = 0
        for tr in geometry:
            key = self._f(tr)
            if key is not None:
                if key not in keymap:
                    keymap[key] = k
                    k += 1
                map[tr.idx] = keymap[key]
            else:
                map[tr.idx] = -1
        keys: np.ndarray = np.ndarray(len(self._map), dtype=np.int32)
        values: np.ndarray = np.ndarray(len(self._map), dtype=GainPtr)
        for i, (key, value) in enumerate(self._map.items()):
            keys[i] = keymap[key]
            values[i]["_0"] = value.gain_ptr(geometry)._0
        return Base().gain_group_by_transducer(
            np.ctypeslib.as_ctypes(map.astype(c_int32)),
            len(map),
            np.ctypeslib.as_ctypes(keys.astype(c_int32)),
            values.ctypes.data_as(POINTER(GainPtr)),  # type: ignore
            len(keys),
        )


class Group:
    @staticmethod
    def by_device(f: Callable[[int], Optional[K]]) -> GroupByDevice[K]:
        return GroupByDevice(f)

    @staticmethod
    def by_transducer(f: Callable[[Transducer], Optional[K]]) -> GroupByTransducer[K]:
        return GroupByTransducer(f)


class TransTest(IGain):
    _indices: List[int]
    _amps: List[float]
    _phases: List[float]

    def __init__(self):
        super().__init__()
        self._indices = []
        self._amps = []
        self._phases = []

    def set(self, tr_idx: int, amp: float, phase: float):
        self._indices.append(tr_idx)
        self._amps.append(amp)
        self._phases.append(phase)

    def gain_ptr(self, _: Geometry) -> GainPtr:
        return functools.reduce(
            lambda acc, v: Base().gain_transducer_test_set(acc, v[0], v[1], v[2]),
            zip(self._indices, self._phases, self._amps),
            Base().gain_transducer_test(),
        )


class Gain(IGain, metaclass=ABCMeta):
    @abstractmethod
    def calc(self, geometry: Geometry) -> np.ndarray:
        pass

    def gain_ptr(self, geometry: Geometry) -> GainPtr:
        drives = self.calc(geometry)
        size = len(drives)
        return Base().gain_custom(drives.ctypes.data_as(POINTER(Drive)), size)  # type: ignore

    @staticmethod
    def transform(geometry: Geometry, f: Callable[[Transducer], Drive]) -> np.ndarray:
        return np.fromiter(map(lambda tr: np.void(f(tr)), geometry), dtype=Drive)  # type: ignore
