'''
File: geometry.py
Project: pyautd3
Created Date: 05/06/2023
Author: Shun Suzuki
-----
Last Modified: 01/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from ctypes import c_double, create_string_buffer
from functools import reduce
import numpy as np
from typing import Optional, List, Iterator

from .autd_error import AUTDError
from .native_methods.autd3capi import NativeMethods as Base
from .native_methods.autd3capi_def import GeometryPtr, TransMode, DevicePtr, TransducerPtr
from .native_methods.autd3capi_def import TRANS_SPACING_MM, DEVICE_WIDTH_MM, DEVICE_HEIGHT_MM
from .native_methods.autd3capi_def import NUM_TRANS_IN_X, NUM_TRANS_IN_UNIT, NUM_TRANS_IN_Y
from .native_methods.autd3capi_def import FPGA_CLK_FREQ, FPGA_SUB_CLK_FREQ


class AUTD3:
    """AUTD3 device

    """

    _pos: np.ndarray
    _rot: Optional[np.ndarray]
    _quat: Optional[np.ndarray]

    def __init__(
        self,
        pos: np.ndarray,
        rot: Optional[np.ndarray],
        quat: Optional[np.ndarray] = None,
    ):
        """Constructor

        Arguments:
        - `pos` - Position of the device
        - `rot` - Rotation of the device in Euler ZYZ
        """

        self._pos = pos
        self._rot = rot
        self._quat = quat

    @staticmethod
    def from_quaternion(pos: np.ndarray, quat: np.ndarray) -> "AUTD3":
        """Constructor

        Arguments:
        - `pos` - Position of the device
        - `quat` - Rotation of the device in quaternion
        """
        return AUTD3(pos, None, quat)

    @staticmethod
    def from_euler_zyz(pos: np.ndarray, rot: np.ndarray) -> "AUTD3":
        """Constructor

        Arguments:
        - `pos` - Position of the device
        - `rot` - Rotation of the device in Euler ZYZ
        """

        return AUTD3(pos, rot, None)

    @staticmethod
    def transducer_spacing() -> float:
        """Spacing between transducers

        """

        return TRANS_SPACING_MM

    @staticmethod
    def device_width() -> float:
        """Device width including substrate

        """

        return DEVICE_WIDTH_MM

    @staticmethod
    def device_height() -> float:
        """Device height including substrate

        """

        return DEVICE_HEIGHT_MM

    @staticmethod
    def num_transducer_in_x() -> int:
        """Number of transducer in x-axis of AUTD3 device

        """

        return NUM_TRANS_IN_X

    @staticmethod
    def num_transducer_in_y() -> int:
        """Number of transducer in y-axis of AUTD3 device

        """

        return NUM_TRANS_IN_Y

    @staticmethod
    def num_transducer_in_unit() -> int:
        """Number of transducer in an AUTD3 device

        """

        return NUM_TRANS_IN_UNIT

    @staticmethod
    def fpga_clk_freq() -> int:
        """FPGA main clock frequency

        """

        return FPGA_CLK_FREQ

    @staticmethod
    def fpga_sub_clk_freq() -> int:
        """FPGA sub clock frequency

        """

        return FPGA_SUB_CLK_FREQ


class Transducer:
    _local_idx: int
    _ptr: TransducerPtr

    def __init__(self, local_idx: int, ptr: DevicePtr):
        self._local_idx = local_idx
        self._ptr = Base().transducer(ptr, local_idx)

    @property
    def local_idx(self) -> int:
        """Get the local index of the transducer

        """

        return self._local_idx

    @property
    def position(self) -> np.ndarray:
        """Get the position of the transducer

        """

        v = np.zeros([3]).astype(c_double)
        vp = np.ctypeslib.as_ctypes(v)
        Base().transducer_position(self._ptr, vp)
        return v

    @property
    def rotation(self) -> np.ndarray:
        """Get the rotation of the transducer

        """

        v = np.zeros([4]).astype(c_double)
        vp = np.ctypeslib.as_ctypes(v)
        Base().transducer_rotation(self._ptr, vp)
        return v

    @property
    def x_direction(self) -> np.ndarray:
        """Get the x-direction of the transducer

        """

        v = np.zeros([3]).astype(c_double)
        vp = np.ctypeslib.as_ctypes(v)
        Base().transducer_direction_x(self._ptr, vp)
        return v

    @property
    def y_direction(self) -> np.ndarray:
        """Get the y-direction of the transducer

        """

        v = np.zeros([3]).astype(c_double)
        vp = np.ctypeslib.as_ctypes(v)
        Base().transducer_direction_y(self._ptr, vp)
        return v

    @property
    def z_direction(self) -> np.ndarray:
        """Get the z-direction of the transducer

        """

        v = np.zeros([3]).astype(c_double)
        vp = np.ctypeslib.as_ctypes(v)
        Base().transducer_direction_z(self._ptr, vp)
        return v

    @property
    def frequency(self) -> float:
        """Get the frequency of the transducer

        """

        return float(Base().transducer_frequency_get(self._ptr))

    @frequency.setter
    def frequency(self, freq: float):
        """Set ultrasound frequency

        Arguments:
        - `freq` - Ultrasound frequency [Hz]. The frequency closest to `freq` from the possible frequencies is set.
        """

        err = create_string_buffer(256)
        if not Base().transducer_frequency_set(self._ptr, freq, err):
            raise AUTDError(err)

    @property
    def cycle(self) -> int:
        """Get the cycle of the transducer

        """

        return int(Base().transducer_cycle_get(self._ptr))

    @cycle.setter
    def cycle(self, cycle: int):
        """Set ultrasound cycle

        Arguments:
        - `cycle` - Cycle of ultrasound
        """

        err = create_string_buffer(256)
        if not Base().transducer_cycle_set(self._ptr, cycle, err):
            raise AUTDError(err)

    @property
    def mod_delay(self) -> int:
        """Get the modulation delay of the transducer

        """

        return int(Base().transducer_mod_delay_get(self._ptr))

    @mod_delay.setter
    def mod_delay(self, delay: int):
        """Set the modulation delay of the transducer

        """

        return Base().transducer_mod_delay_set(self._ptr, delay)

    @property
    def amp_filter(self) -> float:
        """Get the amp filter

        """

        return float(Base().transducer_amp_filter_get(self._ptr))

    @amp_filter.setter
    def amp_filter(self, value: float):
        """Set the amp filter

        Arguments:
        - `value` - Amp filter value (from -1 to 1)
        """

        return Base().transducer_amp_filter_set(self._ptr, value)

    @property
    def phase_filter(self) -> float:
        """Get the phase filter

        """

        return float(Base().transducer_phase_filter_get(self._ptr))

    @phase_filter.setter
    def phase_filter(self, value: float):
        """Set the phase filter

        Arguments:
        - `value` - Phase filter value (from -2π to 2π)
        """

        return Base().transducer_phase_filter_set(self._ptr, value)

    def wavelength(self, sound_speed: float) -> float:
        """Get the wavelength of the transducer

        Arguments:
        - `sound_speed` - Sound speed [mm/s]
        """

        return float(Base().transducer_wavelength(self._ptr, sound_speed))

    def wavenumber(self, sound_speed: float) -> float:
        """Get the wavenumber of the transducer

        Arguments:
        - `sound_speed` - Sound speed [mm/s]
        """

        return 2.0 * np.pi / self.wavelength(sound_speed)


class Device:
    _idx: int
    _ptr: DevicePtr
    _transducers: List[Transducer]

    def __init__(self, i: int, ptr: DevicePtr):
        self._idx = i
        self._ptr = ptr
        self._transducers = list(
            map(lambda i: Transducer(i, self._ptr), range(int(Base().device_num_transducers(self._ptr))))
        )

    @property
    def idx(self) -> int:
        return self._idx

    @property
    def sound_speed(self) -> float:
        """Speed of sound [mm/s]

        """

        return float(Base().device_get_sound_speed(self._ptr))

    @sound_speed.setter
    def sound_speed(self, sound_speed: float):
        Base().device_set_sound_speed(self._ptr, sound_speed)

    def set_sound_speed_from_temp(
        self,
        temp: float,
        k: float = 1.4,
        r: float = 8.31446261815324,
        m: float = 28.9647e-3,
    ):
        """Set speed of sound from temperature

        Arguments:
        - `temp` - Temperature [K]
        - `k` - Ratio of specific heats
        - `r` - Specific gas constant
        - `m` - Molecular mass
        """
        Base().device_set_sound_speed_from_temp(self._ptr, temp, k, r, m)

    @property
    def attenuation(self) -> float:
        return float(Base().device_get_attenuation(self._ptr))

    @attenuation.setter
    def attenuation(self, attenuation: float):
        Base().device_set_attenuation(self._ptr, attenuation)

    @property
    def enable(self) -> bool:
        return bool(Base().device_enable_get(self._ptr))

    @enable.setter
    def enable(self, value: bool):
        Base().device_enable_set(self._ptr, value)

    @property
    def num_transducers(self) -> int:
        """Get the number of transducers

        """

        return len(self._transducers)

    @property
    def center(self) -> np.ndarray:
        """Get center position

        """

        v = np.zeros([3]).astype(c_double)
        vp = np.ctypeslib.as_ctypes(v)
        Base().device_center(self._ptr, vp)
        return v

    def _force_fan(self, value: bool):
        return Base().device_set_force_fan(self._ptr, value)

    force_fan = property(None, _force_fan)

    def _reads_fpga_info(self, value: bool):
        Base().device_set_reads_fpga_info(self._ptr, value)

    reads_fpga_info = property(None, _reads_fpga_info)

    def translate(self, t: np.ndarray):
        """Translate all transducers in the device

        Arguments:
        - `t` - Translation vector
        """
        Base().device_translate(self._ptr, t[0], t[1], t[2])

    def rotate(self, r: np.ndarray):
        """Rotate all transducers in the device

        Arguments:
        - `r` - Rotation quaternion
        """
        Base().device_rotate(self._ptr, r[0], r[1], r[2], r[3])

    def affine(self, t: np.ndarray, r: np.ndarray):
        """Affine transform

        Arguments:
        - `t` - Translation vector
        - `r` - Rotation quaternion
        """
        Base().device_affine(self._ptr, t[0], t[1], t[2], r[0], r[1], r[2], r[3])

    def __getitem__(self, key: int) -> Transducer:
        return self._transducers[key]

    def __iter__(self) -> Iterator[Transducer]:
        return iter(self._transducers)

    def ptr(self) -> DevicePtr:
        return self._ptr


class Geometry:
    _ptr: GeometryPtr
    _mode: TransMode
    _: List[Device]

    def __init__(self, ptr: GeometryPtr, mode: TransMode):
        self._ptr = ptr
        self._mode = mode
        self._devices = list(
            map(lambda i: Device(i, Base().device(self._ptr, i)), range(int(Base().geometry_num_devices(self._ptr))))
        )

    def mode(self) -> TransMode:
        return self._mode

    @property
    def center(self) -> np.ndarray:
        """Get center position of all devices

        """

        return reduce(
            lambda acc, x: acc + x.center,
            self._devices,
            np.zeros(3),
        ) / len(self._devices)

    @property
    def num_devices(self) -> int:
        """Get the number of devices

        """

        return len(self._devices)

    @property
    def num_transducers(self) -> int:
        """Get the number of total transducers

        """

        return reduce(
            lambda acc, x: acc + x.num_transducers,
            self._devices,
            0,
        )

    def __getitem__(self, key: int) -> Device:
        return self._devices[key]

    def __iter__(self) -> Iterator[Device]:
        return iter(self._devices)

    def devices(self) -> Iterator[Device]:
        """Get the iterator of enabled devices

        """

        return filter(lambda x: x.enable, self._devices)

    def set_sound_speed_from_temp(
        self,
        temp: float,
        k: float = 1.4,
        r: float = 8.31446261815324,
        m: float = 28.9647e-3,
    ):
        """Set speed of sound of enabled devices from temperature

        Arguments:
        - `temp` - Temperature [K]
        - `k` - Ratio of specific heats
        - `r` - Specific gas constant
        - `m` - Molecular mass
        """
        for d in self.devices():
            d.set_sound_speed_from_temp(temp, k, r, m)

    def set_sound_speed(
        self,
        c: float,
    ):
        """Set speed of sound of enabled devices

        Arguments:
        - `c` - Speed of sound [mm/s]
        """
        for d in self.devices():
            d.sound_speed = c

    def ptr(self) -> GeometryPtr:
        return self._ptr
