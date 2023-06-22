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

import numpy as np
from typing import Optional, List, Callable
from abc import ABCMeta, abstractmethod
from ctypes import byref, c_void_p, c_uint64, POINTER, memmove, sizeof

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


class Grouped(IGain):
    _gains: List[IGain]
    _indices: List[int]

    def __init__(self):
        super().__init__()
        self._gains = []
        self._indices = []

    def add_gain(self, device_idx: int, gain: IGain):
        self._gains.append(gain)
        self._indices.append(device_idx)

    def gain_ptr(self, geometry: Geometry) -> GainPtr:
        ptr = Base().gain_grouped()
        for i in range(len(self._gains)):
            ptr = Base().gain_grouped_add(
                ptr, self._indices[i], self._gains[i].gain_ptr(geometry)
            )
        return ptr


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

    def gain_ptr(self, geometry: Geometry) -> GainPtr:
        ptr = Base().gain_transducer_test()
        for i in range(len(self._indices)):
            ptr = Base().gain_transducer_test_set(
                ptr, self._indices[i], self._amps[i], self._phases[i]
            )
        return ptr


class Gain(IGain, metaclass=ABCMeta):
    @abstractmethod
    def calc(self, geometry: Geometry) -> np.ndarray:
        pass

    def gain_ptr(self, geometry: Geometry) -> GainPtr:
        drives = self.calc(geometry)
        size = len(drives)
        ptr = c_void_p()
        len_ = c_uint64()
        cap = c_uint64()
        Base().alloc_drive_buf(size, byref(ptr), byref(len_), byref(cap))
        memmove(
            ptr,
            drives.ctypes.data_as(POINTER(Drive)),
            int(len_.value) * sizeof(Drive),
        )
        return Base().gain_custom(ptr, int(len_.value), int(cap.value))

    @staticmethod
    def transform(geometry: Geometry, f: Callable[[Transducer], Drive]) -> np.ndarray:
        drives = np.zeros(geometry.num_transducers, dtype=Drive)
        for i, tr in enumerate(geometry):
            d = f(tr)
            drives[i]["phase"] = d.phase
            drives[i]["amp"] = d.amp
        return drives
