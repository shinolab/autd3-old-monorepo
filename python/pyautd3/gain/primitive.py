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

import warnings

import functools
import numpy as np
from typing import Optional, List, Callable, Iterator
from abc import ABCMeta, abstractmethod
from ctypes import POINTER, create_string_buffer

from pyautd3.autd_error import AUTDError
from pyautd3.native_methods.autd3capi import Drive
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import GainPtr, AUTD3_ERR
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
    _indices: List[np.ndarray]

    def __init__(self):
        super().__init__()
        self._gains = []
        self._indices = []

    def add_gain(self, device_idx: int, gain: IGain) -> "Grouped":
        warnings.warn("`add_gain` is deprecated, use `add` instead", UserWarning)
        self._gains.append(gain)
        self._indices.append(np.array([device_idx], dtype=np.uint32))
        return self

    def add(self, device_idx: int, gain: IGain) -> "Grouped":
        self._gains.append(gain)
        self._indices.append(np.array([device_idx], dtype=np.uint32))
        return self

    def add_by_group(self, device_ids: List[int], gain: IGain) -> "Grouped":
        self._gains.append(gain)
        self._indices.append(np.array(device_ids, dtype=np.uint32))
        return self

    def gain_ptr(self, geometry: Geometry) -> GainPtr:
        return functools.reduce(
            lambda acc, v: Base().gain_grouped_add_by_group(
                acc,
                np.ctypeslib.as_ctypes(v[0]),
                len(v[0]),
                v[1].gain_ptr(geometry),
            ),
            zip(self._indices, self._gains),
            Base().gain_grouped(),
        )


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


class Cache(IGain):
    _drives: np.ndarray

    def __init__(self, g: IGain, geometry: Geometry):
        super().__init__()
        self._drives = np.zeros(geometry.num_transducers, dtype=Drive)
        dp = np.ctypeslib.as_ctypes(self._drives)
        err = create_string_buffer(256)
        if Base().gain_calc(g.gain_ptr(geometry), geometry.ptr(), dp, err) == AUTD3_ERR:
            raise AUTDError(err)

    def gain_ptr(self, _: Geometry) -> GainPtr:
        size = len(self._drives)
        dp = np.ctypeslib.as_ctypes(self._drives)
        return Base().gain_custom(dp, size)

    @property
    def drives(self) -> np.ndarray:
        return self._drives

    def __getitem__(self, key: int) -> Drive:
        return self._drives[key]

    def __iter__(self) -> Iterator[Drive]:
        return iter(self._drives)
