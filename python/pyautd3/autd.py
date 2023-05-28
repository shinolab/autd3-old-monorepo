"""
File: autd.py
Project: pyautd3
Created Date: 24/05/2021
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""


from datetime import timedelta
import ctypes
from ctypes import c_void_p, byref, c_double, c_bool
import numpy as np
from typing import List, Optional, Tuple

from .autd_error import AUTDError
from .native_methods.autd3capi import NativeMethods as Base
from .native_methods.autd3capi import TransMode, ERR, TRUE, FALSE
from .link.link import Link

LogOutputFunc = ctypes.CFUNCTYPE(None, ctypes.c_char_p)
LogFlushFunc = ctypes.CFUNCTYPE(None)


class SpecialData:
    ptr: c_void_p

    def __init__(self):
        self.ptr = c_void_p()


class Body:
    ptr: c_void_p

    def __init__(self):
        self.ptr = c_void_p()


class Header:
    ptr: c_void_p

    def __init__(self):
        self.ptr = c_void_p()


class SilencerConfig(Header):
    def __init__(self, step: int = 10):
        super().__init__()
        self.ptr = Base().create_silencer(step)

    def __del__(self):
        Base().delete_silencer(self.ptr)

    @staticmethod
    def none() -> SilencerConfig:
        return SilencerConfig(0xFFFF)


class FPGAInfo:
    info: ctypes.c_uint8

    def __init__(self, info: ctypes.c_uint8):
        self.info = info

    def is_thernal_assert(self) -> bool:
        return (self.info.value & 0x01) != 0


class Transducer:
    _tr_id: int
    _cnt: c_void_p

    def __init__(self, tr_id: int, cnt: c_void_p):
        self._tr_id = tr_id
        self._cnt = cnt

    @property
    def id(self) -> int:
        return self._tr_id

    @property
    def position(self) -> np.ndarray:
        x = c_double(0.0)
        y = c_double(0.0)
        z = c_double(0.0)
        Base().trans_position(self._cnt, self._tr_id, byref(x), byref(y), byref(z))
        return np.array([x.value, y.value, z.value])

    @property
    def frequency(self) -> float:
        return Base().get_trans_frequency(self._cnt, self._tr_id).value

    @frequency.setter
    def frequency(self, freq: float):
        err = ctypes.create_string_buffer(256)
        if not Base().set_trans_frequency(self._cnt, self._tr_id, freq, err):
            raise AUTDError(err)

    @property
    def cycle(self) -> int:
        return Base().get_trans_cycle(self._cnt, self._tr_id).value

    @cycle.setter
    def cycle(self, cycle: int):
        err = ctypes.create_string_buffer(256)
        if not Base().set_trans_cycle(self._cnt, self._tr_id, cycle, err):
            raise AUTDError(err)

    @property
    def mod_delay(self) -> int:
        return Base().get_trans_mod_delay(self._cnt, self._tr_id).value

    @mod_delay.setter
    def mod_delay(self, delay: int):
        return Base().set_trans_mod_delay(self._cnt, self._tr_id, delay)

    @property
    def wavelength(self) -> float:
        return Base().get_wavelength(self._cnt, self._tr_id).value

    @property
    def x_direction(self) -> np.ndarray:
        x = c_double(0.0)
        y = c_double(0.0)
        z = c_double(0.0)
        Base().trans_x_direction(self._cnt, self._tr_id, byref(x), byref(y), byref(z))
        return np.array([x.value, y.value, z.value])

    @property
    def y_direction(self) -> np.ndarray:
        x = c_double(0.0)
        y = c_double(0.0)
        z = c_double(0.0)
        Base().trans_y_direction(self._cnt, self._tr_id, byref(x), byref(y), byref(z))
        return np.array([x.value, y.value, z.value])

    @property
    def z_direction(self) -> np.ndarray:
        x = c_double(0.0)
        y = c_double(0.0)
        z = c_double(0.0)
        Base().trans_z_direction(self._cnt, self._tr_id, byref(x), byref(y), byref(z))
        return np.array([x.value, y.value, z.value])


class Geometry:
    _ptr: c_void_p
    _mode: TransMode

    def __init__(self, ptr: c_void_p, mode: TransMode):
        self._ptr = ptr
        self._mode = mode

    def mode(self) -> TransMode:
        return self._mode

    @property
    def sound_speed(self) -> float:
        return Base().get_sound_speed(self._ptr).value

    @sound_speed.setter
    def sound_speed(self, sound_speed: float):
        Base().set_sound_speed(self._ptr, sound_speed)

    @property
    def attenuation(self) -> float:
        return Base().get_attenuation(self._ptr).value

    @attenuation.setter
    def attenuation(self, attenuation: float):
        Base().set_attenuation(self._ptr, attenuation)

    @property
    def num_transducers(self) -> int:
        return Base().num_transducers(self._ptr).value

    @property
    def num_devices(self) -> int:
        return Base().num_devices(self._ptr).value

    @property
    def center(self) -> np.ndarray:
        x = c_double(0.0)
        y = c_double(0.0)
        z = c_double(0.0)
        Base().geometry_center(self._ptr, byref(x), byref(y), byref(z))
        return np.array([x.value, y.value, z.value])

    def set_sound_speed_from_temp(
        self,
        temp: float,
        k: float = 1.4,
        r: float = 8.31446261815324,
        m: float = 28.9647e-3,
    ):
        Base().set_sound_speed_from_temp(self._ptr, temp, k, r, m)

    def center_of(self, dev_idx: int) -> np.ndarray:
        x = c_double(0.0)
        y = c_double(0.0)
        z = c_double(0.0)
        Base().geometry_center_of(self._ptr, dev_idx, byref(x), byref(y), byref(z))
        return np.array([x.value, y.value, z.value])

    def __getitem__(self, key: int) -> Transducer:
        return Transducer(key, self._ptr)

    class TransdducerIterator:
        _ptr: c_void_p
        _i: int
        _num_trans: int

        def __init__(self, ptr: c_void_p):
            self._ptr = ptr
            self._i = 0
            self._num_trans = Base().num_transducers(ptr).value

        def __next__(self) -> Transducer:
            if self._i == self._num_trans:
                raise StopIteration()
            value = Transducer(self._i, self._ptr)
            self._i += 1
            return value

    def __iter__(self) -> TransdducerIterator:
        return Geometry.TransdducerIterator(self._ptr)

    class Builder:
        _ptr: c_void_p
        _mode: TransMode

        def __init__(self):
            self._ptr = Base().create_geometry_builder()
            self._mode = TransMode.Legacy

        def add_device(self, pos, rot) -> Geometry.Builder:
            Base().add_device(self._ptr, pos[0], pos[1], pos[2], rot[0], rot[1], rot[2])
            return self

        def add_device_quaternion(self, pos, q) -> Geometry.Builder:
            Base().add_device_quaternion(
                self._ptr, pos[0], pos[1], pos[2], q[0], q[1], q[2], q[3]
            )
            return self

        def legacy_mode(self) -> Geometry.Builder:
            self._mode = TransMode.Legacy
            return self

        def advanced_mode(self) -> Geometry.Builder:
            self._mode = TransMode.Advanced
            return self

        def advanced_phase_mode(self) -> Geometry.Builder:
            self._mode = TransMode.AdvancedPhase
            return self

        def build(self) -> Geometry:
            err = ctypes.create_string_buffer(256)
            ptr = Base().build_geometry(self._ptr, err)
            if not ptr:
                raise AUTDError(err)
            return Geometry(ptr, self._mode)


class FirmwareInfo:
    _info: str
    _is_valid: bool
    _is_supported: bool

    def __init__(self, info: str, is_valid: bool, is_supported: bool):
        self._info = info
        self._is_valid = is_valid
        self._is_supported = is_supported

    @property
    def info(self) -> str:
        return self._info

    @property
    def is_valid(self) -> bool:
        return self._is_valid

    @property
    def is_supported(self) -> bool:
        return self._is_supported

    @staticmethod
    def latest_version() -> str:
        sb = ctypes.create_string_buffer(256)
        Base().get_latest_firmware(sb)
        return sb.value.decode("utf-8")

    def __repr__(self) -> str:
        return self._info


class Controller:
    p_cnt: c_void_p
    _geometry: Geometry
    __disposed: bool

    def __init__(self, cnt: c_void_p, mode: TransMode):
        self.p_cnt = cnt
        self._geometry = Geometry(cnt, mode)
        self.__disposed = False

    def __del__(self):
        self.dispose()

    @property
    def geometry(self) -> Geometry:
        return self._geometry

    @staticmethod
    def open(geometry: Geometry, link: Link) -> Controller:
        err = ctypes.create_string_buffer(256)
        cnt = Base().open_controller(geometry._ptr, link.link_ptr, err)
        if not cnt:
            raise AUTDError(err)
        return Controller(cnt, geometry.mode())

    def firmware_info_list(self) -> List[FirmwareInfo]:
        err = ctypes.create_string_buffer(256)
        handle = Base().get_firmware_info_list_pointer(self.p_cnt, err)
        if not handle:
            raise AUTDError(err)

        res = []
        for i in range(self.geometry.num_devices):
            sb = ctypes.create_string_buffer(256)
            is_valid = c_bool(False)
            is_supported = c_bool(False)
            Base().get_firmware_info(
                handle, i, sb, byref(is_valid), byref(is_supported)
            )
            info = sb.value.decode("utf-8")
            res.append(FirmwareInfo(info, is_valid.value, is_supported.value))

        Base().free_firmware_info_list_pointer(handle)

        return res

    def dispose(self):
        if not self.__disposed:
            self.close()
            self._free()
            self.__disposed = True

    def close(self):
        err = ctypes.create_string_buffer(256)
        if not Base().close(self.p_cnt, err):
            raise AUTDError(err)

    def _free(self):
        Base().free_controller(self.p_cnt)

    def force_fan(self, value: bool):
        return Base().set_force_fan(self.p_cnt, value)

    def reads_fpga_info(self, value: bool):
        Base().set_reads_fpga_info(self.p_cnt, value)

    @property
    def fpga_info(self) -> List[FPGAInfo]:
        infos = np.zeros([self.geometry.num_devices]).astype(np.ubyte)
        pinfos = np.ctypeslib.as_ctypes(infos)
        Base().dll.AUTDGetFPGAInfo(self.p_cnt, pinfos)
        return list(map(lambda x: FPGAInfo(x), infos))

    def send(
        self,
        d: SpecialData | Header | Body | Tuple[Header, Body],
        timeout: Optional[timedelta] = None,
    ) -> bool:
        timeout_ = (
            -1 if timeout is None else int(timeout.total_seconds() * 1000 * 1000 * 1000)
        )
        err = ctypes.create_string_buffer(256)
        res: ctypes.c_int32 = ctypes.c_int32(FALSE)
        if isinstance(d, SpecialData):
            res = Base().send_special(
                self.p_cnt, self.geometry.mode(), d.ptr, timeout_, err
            )
        if isinstance(d, Header):
            res = Base().send(
                self.p_cnt, self.geometry.mode(), d.ptr, c_void_p(None), timeout_, err
            )
        if isinstance(d, Body):
            res = Base().send(
                self.p_cnt, self.geometry.mode(), c_void_p(None), d.ptr, timeout_, err
            )
        if isinstance(d, tuple) and len(d) == 2:
            (h, b) = d
            if isinstance(h, Header) and isinstance(b, Body):
                res = Base().send(
                    self.p_cnt,
                    self.geometry.mode(),
                    h.ptr,
                    b.ptr,
                    timeout_,
                    err,
                )

        if res == ERR:
            raise AUTDError(err)

        return res == TRUE


class Amplitudes(Body):
    def __init__(self, amp: float):
        super().__init__()
        self.ptr = Base().create_amplitudes(amp)

    def __del__(self):
        Base().delete_amplitudes(self.ptr)


class Clear(SpecialData):
    def __init__(self):
        super().__init__()
        self.ptr = Base().clear()

    def __del__(self):
        Base().delete_special_data(self.ptr)


class Stop(SpecialData):
    def __init__(self):
        super().__init__()
        self.ptr = Base().stop()

    def __del__(self):
        Base().delete_special_data(self.ptr)


class UpdateFlag(SpecialData):
    def __init__(self):
        super().__init__()
        self.ptr = Base().update_flags()

    def __del__(self):
        Base().delete_special_data(self.ptr)


class Synchronize(SpecialData):
    def __init__(self):
        super().__init__()
        self.ptr = Base().dll.AUTDSynchronize()

    def __del__(self):
        Base().delete_special_data(self.ptr)


class ModDelayConfig(SpecialData):
    def __init__(self):
        super().__init__()
        self.ptr = Base().mod_delay_config()

    def __del__(self):
        Base().delete_special_data(self.ptr)
