"""
File: geometry.py
Project: pyautd3
Created Date: 05/06/2023
Author: Shun Suzuki
-----
Last Modified: 04/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from collections.abc import Iterator
from ctypes import c_double
from functools import reduce

import numpy as np
from numpy.typing import ArrayLike

from .native_methods.autd3capi import NativeMethods as Base
from .native_methods.autd3capi_def import (
    DEVICE_HEIGHT_MM,
    DEVICE_WIDTH_MM,
    FPGA_CLK_FREQ,
    NUM_TRANS_IN_UNIT,
    NUM_TRANS_IN_X,
    NUM_TRANS_IN_Y,
    TRANS_SPACING_MM,
    DevicePtr,
    GeometryPtr,
    TransducerPtr,
)


class Angle:
    """Angle."""

    _value: float

    def __new__(cls: type["Angle"]) -> "Angle":
        """DO NOT USE THIS CONSTRUCTOR."""
        raise NotImplementedError

    @classmethod
    def __private_new__(cls: type["Angle"], value: float) -> "Angle":
        ins = super().__new__(cls)
        ins._value = value
        return ins

    @staticmethod
    def new_radian(value: float) -> "Angle":
        """Create by radian."""
        return Angle.__private_new__(value)

    @staticmethod
    def new_degree(value: float) -> "Angle":
        """Create by degree."""
        return Angle.__private_new__(np.deg2rad(value))

    @property
    def radian(self: "Angle") -> float:
        """Angle in radian."""
        return self._value

    class _UnitRad:
        def __new__(cls: type["Angle._UnitRad"]) -> "Angle._UnitRad":
            """DO NOT USE THIS CONSTRUCTOR."""
            raise NotImplementedError

        @classmethod
        def __private_new__(cls: type["Angle._UnitRad"]) -> "Angle._UnitRad":
            return super().__new__(cls)

        def __rmul__(self: "Angle._UnitRad", other: float) -> "Angle":
            return Angle.new_radian(other)

    class _UnitDegree:
        def __new__(cls: type["Angle._UnitDegree"]) -> "Angle._UnitDegree":
            """DO NOT USE THIS CONSTRUCTOR."""
            raise NotImplementedError

        @classmethod
        def __private_new__(cls: type["Angle._UnitDegree"]) -> "Angle._UnitDegree":
            return super().__new__(cls)

        def __rmul__(self: "Angle._UnitDegree", other: float) -> "Angle":
            return Angle.new_degree(other)


rad: Angle._UnitRad = Angle._UnitRad.__private_new__()
deg: Angle._UnitDegree = Angle._UnitDegree.__private_new__()


class EulerAngles:
    """Euler angles."""

    def __new__(cls: type["EulerAngles"]) -> "EulerAngles":
        """DO NOT USE THIS CONSTRUCTOR."""
        raise NotImplementedError

    @staticmethod
    def from_zyz(z1: Angle, y: Angle, z2: Angle) -> np.ndarray:
        """Create from Euler ZYZ.

        Arguments:
        ---------
            z1: First rotation angle
            y: Second rotation angle
            z2: Third rotation angle
        """
        v = np.zeros([4]).astype(c_double)
        vp = np.ctypeslib.as_ctypes(v)
        Base().rotation_from_euler_zyz(z1.radian, y.radian, z2.radian, vp)
        return v


class AUTD3:
    """AUTD3 device."""

    _pos: np.ndarray
    _rot: np.ndarray | None

    def __init__(self: "AUTD3", pos: ArrayLike) -> None:
        """Constructor.

        Arguments:
        ---------
            pos: Position of the device
        """
        self._pos = np.array(pos)
        self._rot = None

    def with_rotation(self: "AUTD3", rot: ArrayLike) -> "AUTD3":
        """Set device rotation.

        Arguments:
        ---------
            rot: Rotation of the device in quaternion
        """
        self._rot = np.array(rot)
        return self

    @staticmethod
    def transducer_spacing() -> float:
        """Spacing between transducers."""
        return TRANS_SPACING_MM

    @staticmethod
    def device_width() -> float:
        """Device width including substrate."""
        return DEVICE_WIDTH_MM

    @staticmethod
    def device_height() -> float:
        """Device height including substrate."""
        return DEVICE_HEIGHT_MM

    @staticmethod
    def num_transducer_in_x() -> int:
        """Number of transducer in x-axis of AUTD3 device."""
        return NUM_TRANS_IN_X

    @staticmethod
    def num_transducer_in_y() -> int:
        """Number of transducer in y-axis of AUTD3 device."""
        return NUM_TRANS_IN_Y

    @staticmethod
    def num_transducer_in_unit() -> int:
        """Number of transducer in an AUTD3 device."""
        return NUM_TRANS_IN_UNIT

    @staticmethod
    def fpga_clk_freq() -> int:
        """FPGA clock frequency."""
        return FPGA_CLK_FREQ


class Transducer:
    """Transducer."""

    _idx: int
    _ptr: TransducerPtr

    def __init__(self: "Transducer", idx: int, ptr: DevicePtr) -> None:
        self._idx = idx
        self._ptr = Base().transducer(ptr, idx)

    @property
    def idx(self: "Transducer") -> int:
        """Get the local index of the transducer."""
        return self._idx

    @property
    def position(self: "Transducer") -> np.ndarray:
        """Get the position of the transducer."""
        v = np.zeros([3]).astype(c_double)
        vp = np.ctypeslib.as_ctypes(v)
        Base().transducer_position(self._ptr, vp)
        return v

    @property
    def rotation(self: "Transducer") -> np.ndarray:
        """Get the rotation of the transducer."""
        v = np.zeros([4]).astype(c_double)
        vp = np.ctypeslib.as_ctypes(v)
        Base().transducer_rotation(self._ptr, vp)
        return v

    @property
    def x_direction(self: "Transducer") -> np.ndarray:
        """Get the x-direction of the transducer."""
        v = np.zeros([3]).astype(c_double)
        vp = np.ctypeslib.as_ctypes(v)
        Base().transducer_direction_x(self._ptr, vp)
        return v

    @property
    def y_direction(self: "Transducer") -> np.ndarray:
        """Get the y-direction of the transducer."""
        v = np.zeros([3]).astype(c_double)
        vp = np.ctypeslib.as_ctypes(v)
        Base().transducer_direction_y(self._ptr, vp)
        return v

    @property
    def z_direction(self: "Transducer") -> np.ndarray:
        """Get the z-direction of the transducer."""
        v = np.zeros([3]).astype(c_double)
        vp = np.ctypeslib.as_ctypes(v)
        Base().transducer_direction_z(self._ptr, vp)
        return v

    def wavelength(self: "Transducer", sound_speed: float) -> float:
        """Get the wavelength of the transducer.

        Arguments:
        ---------
            sound_speed: Sound speed [mm/s]
        """
        return float(Base().transducer_wavelength(self._ptr, sound_speed))

    def wavenumber(self: "Transducer", sound_speed: float) -> float:
        """Get the wavenumber of the transducer.

        Arguments:
        ---------
            sound_speed: Sound speed [mm/s]
        """
        return 2.0 * np.pi / self.wavelength(sound_speed)


class Device:
    """Device."""

    _idx: int
    _ptr: DevicePtr
    _transducers: list[Transducer]

    def __init__(self: "Device", idx: int, ptr: DevicePtr) -> None:
        self._idx = idx
        self._ptr = ptr
        self._transducers = [Transducer(i, self._ptr) for i in range(int(Base().device_num_transducers(self._ptr)))]

    @property
    def idx(self: "Device") -> int:
        """Get the index of the device."""
        return self._idx

    @property
    def sound_speed(self: "Device") -> float:
        """Speed of sound [mm/s]."""
        return float(Base().device_get_sound_speed(self._ptr))

    @sound_speed.setter
    def sound_speed(self: "Device", sound_speed: float) -> None:
        Base().device_set_sound_speed(self._ptr, sound_speed)

    def set_sound_speed_from_temp(
        self: "Device",
        temp: float,
        k: float = 1.4,
        r: float = 8.31446261815324,
        m: float = 28.9647e-3,
    ) -> None:
        """Set speed of sound from temperature.

        Arguments:
        ---------
            temp: Temperature [K]
            k: Ratio of specific heats
            r: Specific gas constant
            m: Molecular mass
        """
        Base().device_set_sound_speed_from_temp(self._ptr, temp, k, r, m)

    @property
    def attenuation(self: "Device") -> float:
        """Get the attenuation of the device."""
        return float(Base().device_get_attenuation(self._ptr))

    @attenuation.setter
    def attenuation(self: "Device", attenuation: float) -> None:
        Base().device_set_attenuation(self._ptr, attenuation)

    @property
    def enable(self: "Device") -> bool:
        """Get the enable state of the device."""
        return bool(Base().device_enable_get(self._ptr))

    @enable.setter
    def enable(self: "Device", value: bool) -> None:
        Base().device_enable_set(self._ptr, value)

    @property
    def num_transducers(self: "Device") -> int:
        """Get the number of transducers."""
        return len(self._transducers)

    @property
    def center(self: "Device") -> np.ndarray:
        """Get center position."""
        v = np.zeros([3]).astype(c_double)
        vp = np.ctypeslib.as_ctypes(v)
        Base().device_center(self._ptr, vp)
        return v

    def translate(self: "Device", t: ArrayLike) -> None:
        """Translate all transducers in the device.

        Arguments:
        ---------
            t: Translation vector
        """
        t = np.array(t)
        Base().device_translate(self._ptr, t[0], t[1], t[2])

    def rotate(self: "Device", r: ArrayLike) -> None:
        """Rotate all transducers in the device.

        Arguments:
        ---------
            r: Rotation quaternion
        """
        r = np.array(r)
        Base().device_rotate(self._ptr, r[0], r[1], r[2], r[3])

    def affine(self: "Device", t: ArrayLike, r: ArrayLike) -> None:
        """Affine transform.

        Arguments:
        ---------
            t: Translation vector
            r: Rotation quaternion
        """
        t = np.array(t)
        r = np.array(r)
        Base().device_affine(self._ptr, t[0], t[1], t[2], r[0], r[1], r[2], r[3])

    def __getitem__(self: "Device", key: int) -> Transducer:
        return self._transducers[key]

    def __iter__(self: "Device") -> Iterator[Transducer]:
        return iter(self._transducers)


class Geometry:
    """Geometry."""

    _ptr: GeometryPtr
    _devices: list[Device]

    def __init__(self: "Geometry", ptr: GeometryPtr) -> None:
        self._ptr = ptr
        self._devices = [Device(i, Base().device(self._ptr, i)) for i in range(int(Base().geometry_num_devices(self._ptr)))]

    @property
    def center(self: "Geometry") -> np.ndarray:
        """Get center position of all devices."""
        return reduce(
            lambda acc, x: acc + x.center,
            self._devices,
            np.zeros(3),
        ) / len(self._devices)

    @property
    def num_devices(self: "Geometry") -> int:
        """Get the number of devices."""
        return len(self._devices)

    @property
    def num_transducers(self: "Geometry") -> int:
        """Get the number of total transducers."""
        return reduce(
            lambda acc, x: acc + x.num_transducers,
            self._devices,
            0,
        )

    def __getitem__(self: "Geometry", key: int) -> Device:
        return self._devices[key]

    def __iter__(self: "Geometry") -> Iterator[Device]:
        return iter(self._devices)

    def devices(self: "Geometry") -> Iterator[Device]:
        """Get the iterator of enabled devices."""
        return filter(lambda x: x.enable, self._devices)

    def set_sound_speed_from_temp(
        self: "Geometry",
        temp: float,
        k: float = 1.4,
        r: float = 8.31446261815324,
        m: float = 28.9647e-3,
    ) -> None:
        """Set speed of sound of enabled devices from temperature.

        Arguments:
        ---------
            temp: Temperature [K]
            k: Ratio of specific heats
            r: Specific gas constant
            m: Molecular mass
        """
        for d in self.devices():
            d.set_sound_speed_from_temp(temp, k, r, m)

    def set_sound_speed(
        self: "Geometry",
        c: float,
    ) -> None:
        """Set speed of sound of enabled devices.

        Arguments:
        ---------
            c: Speed of sound [mm/s]
        """
        for d in self.devices():
            d.sound_speed = c

    def _geometry_ptr(self: "Geometry") -> GeometryPtr:
        return self._ptr
