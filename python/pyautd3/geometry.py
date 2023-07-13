"""
File: geometry.py
Project: pyautd3
Created Date: 05/06/2023
Author: Shun Suzuki
-----
Last Modified: 05/06/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from ctypes import c_double, create_string_buffer
import numpy as np
from typing import Optional, List, Iterator

from .autd_error import AUTDError
from .native_methods.autd3capi import NativeMethods as Base
from .native_methods.autd3capi_def import GeometryPtr, TransMode


class AUTD3:
    _pos: np.ndarray
    _rot: Optional[np.ndarray]
    _quat: Optional[np.ndarray]

    def __init__(
        self,
        pos: np.ndarray,
        rot: Optional[np.ndarray],
        quat: Optional[np.ndarray],
    ):
        self._pos = pos
        self._rot = rot
        self._quat = quat

    @staticmethod
    def from_quaternion(pos: np.ndarray, quat: np.ndarray) -> "AUTD3":
        return AUTD3(pos, None, quat)

    @staticmethod
    def from_euler_zyz(pos: np.ndarray, rot: np.ndarray) -> "AUTD3":
        return AUTD3(pos, rot, None)


class Transducer:
    _tr_id: int
    _ptr: GeometryPtr

    def __init__(self, tr_id: int, ptr: GeometryPtr):
        self._tr_id = tr_id
        self._ptr = ptr

    @property
    def idx(self) -> int:
        return self._tr_id

    @property
    def position(self) -> np.ndarray:
        v = np.zeros([3]).astype(c_double)
        vp = np.ctypeslib.as_ctypes(v)
        Base().trans_position(self._ptr, self._tr_id, vp)
        return v

    @property
    def rotation(self) -> np.ndarray:
        v = np.zeros([4]).astype(c_double)
        vp = np.ctypeslib.as_ctypes(v)
        Base().trans_rotation(self._ptr, self._tr_id, vp)
        return v

    @property
    def x_direction(self) -> np.ndarray:
        v = np.zeros([3]).astype(c_double)
        vp = np.ctypeslib.as_ctypes(v)
        Base().trans_x_direction(self._ptr, self._tr_id, vp)
        return v

    @property
    def y_direction(self) -> np.ndarray:
        v = np.zeros([3]).astype(c_double)
        vp = np.ctypeslib.as_ctypes(v)
        Base().trans_y_direction(self._ptr, self._tr_id, vp)
        return v

    @property
    def z_direction(self) -> np.ndarray:
        v = np.zeros([3]).astype(c_double)
        vp = np.ctypeslib.as_ctypes(v)
        Base().trans_z_direction(self._ptr, self._tr_id, vp)
        return v

    @property
    def frequency(self) -> float:
        return float(Base().get_trans_frequency(self._ptr, self._tr_id))

    @frequency.setter
    def frequency(self, freq: float):
        err = create_string_buffer(256)
        if not Base().set_trans_frequency(self._ptr, self._tr_id, freq, err):
            raise AUTDError(err)

    @property
    def cycle(self) -> int:
        return int(Base().get_trans_cycle(self._ptr, self._tr_id))

    @cycle.setter
    def cycle(self, cycle: int):
        err = create_string_buffer(256)
        if not Base().set_trans_cycle(self._ptr, self._tr_id, cycle, err):
            raise AUTDError(err)

    @property
    def mod_delay(self) -> int:
        return int(Base().get_trans_mod_delay(self._ptr, self._tr_id))

    @mod_delay.setter
    def mod_delay(self, delay: int):
        return Base().set_trans_mod_delay(self._ptr, self._tr_id, delay)

    def wavelength(self, sound_speed: float) -> float:
        return float(Base().get_wavelength(self._ptr, self._tr_id, sound_speed))

    def wavenumber(self, sound_speed: float) -> float:
        return 2.0 * np.pi / self.wavelength(sound_speed)


class Geometry:
    _ptr: GeometryPtr
    _mode: TransMode
    _transducers: List[Transducer]

    def __init__(self, ptr: GeometryPtr, mode: TransMode):
        self._ptr = ptr
        self._mode = mode

    def mode(self) -> TransMode:
        return self._mode

    @property
    def sound_speed(self) -> float:
        return float(Base().get_sound_speed(self._ptr))

    @sound_speed.setter
    def sound_speed(self, sound_speed: float):
        Base().set_sound_speed(self._ptr, sound_speed)

    def set_sound_speed_from_temp(
        self,
        temp: float,
        k: float = 1.4,
        r: float = 8.31446261815324,
        m: float = 28.9647e-3,
    ):
        Base().set_sound_speed_from_temp(self._ptr, temp, k, r, m)

    @property
    def attenuation(self) -> float:
        return float(Base().get_attenuation(self._ptr))

    @attenuation.setter
    def attenuation(self, attenuation: float):
        Base().set_attenuation(self._ptr, attenuation)

    @property
    def num_transducers(self) -> int:
        return int(Base().num_transducers(self._ptr))

    @property
    def num_devices(self) -> int:
        return int(Base().num_devices(self._ptr))

    @property
    def center(self) -> np.ndarray:
        v = np.zeros([3]).astype(c_double)
        vp = np.ctypeslib.as_ctypes(v)
        Base().geometry_center(self._ptr, vp)
        return v

    def center_of(self, dev_idx: int) -> np.ndarray:
        v = np.zeros([3]).astype(c_double)
        vp = np.ctypeslib.as_ctypes(v)
        Base().geometry_center_of(self._ptr, dev_idx, vp)
        return v

    def __getitem__(self, key: int) -> Transducer:
        return self._transducers[key]

    def __iter__(self) -> Iterator[Transducer]:
        return iter(self._transducers)

    def ptr(self) -> GeometryPtr:
        return self._ptr

    def _configure(self):
        self._transducers = list(
            map(lambda i: Transducer(i, self._ptr), range(self.num_transducers))
        )
