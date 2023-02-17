'''
File: autd.py
Project: pyautd3
Created Date: 24/05/2021
Author: Shun Suzuki
-----
Last Modified: 18/02/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''


import ctypes
from ctypes import c_void_p, byref, c_double, c_bool
import numpy as np

from .native_methods.autd3capi import NativeMethods as Base
from .link.link import Link


NUM_TRANS_IN_UNIT = 249
NUM_TRANS_X = 18
NUM_TRANS_Y = 14
TRANS_SPACING = 10.16
DEVICE_WIDTH = 192.0
DEVICE_HEIGHT = 151.4

LogOutputFunc = ctypes.CFUNCTYPE(None, ctypes.c_char_p)
LogFlushFunc = ctypes.CFUNCTYPE(None)


def set_log_level(level: int):
    Base().dll.AUTDSetLogLevel(level)


def set_log_func(output, flush):
    Base().dll.AUTDSetDefaultLogger(output, flush)


class SpecialData:
    def __init__(self):
        self.ptr = c_void_p()


class Body:
    def __init__(self):
        self.ptr = c_void_p()


class Header:
    def __init__(self):
        self.ptr = c_void_p()


class SilencerConfig(Header):
    def __init__(self, step: int = 10, cycle: int = 4096):
        super().__init__()
        Base().dll.AUTDCreateSilencer(byref(self.ptr), step, cycle)

    def __del__(self):
        Base().dll.AUTDDeleteSilencer(self.ptr)

    @staticmethod
    def none():
        return SilencerConfig(0xFFFF, 4096)


class Transducer:
    def __init__(self, tr_id: int, cnt: c_void_p):
        self._tr_id = tr_id
        self._cnt = cnt

    @ property
    def id(self) -> int:
        return self._tr_id

    @ property
    def position(self):
        x = c_double(0.0)
        y = c_double(0.0)
        z = c_double(0.0)
        Base().dll.AUTDTransPosition(self._cnt, self._tr_id, byref(x), byref(y), byref(z))
        return np.array([x.value, y.value, z.value])

    @ property
    def frequency(self):
        return Base().dll.AUTDGetTransFrequency(self._cnt, self._tr_id)

    @ frequency.setter
    def frequency(self, freq: float):
        return Base().dll.AUTDSetTransFrequency(self._cnt, self._tr_id, freq)

    @ property
    def cycle(self):
        return Base().dll.AUTDGetTransCycle(self._cnt, self._tr_id)

    @ cycle.setter
    def cycle(self, cycle: int):
        return Base().dll.AUTDSetTransCycle(self._cnt, self._tr_id, cycle)

    @ property
    def mod_delay(self):
        return Base().dll.AUTDGetTransModDelay(self._cnt, self._tr_id)

    @ mod_delay.setter
    def mod_delay(self, delay: int):
        return Base().dll.AUTDSetTransModDelay(self._cnt, self._tr_id, delay)

    @ property
    def wavelength(self):
        return Base().dll.AUTDGetWavelength(self._cnt, self._tr_id)

    @ property
    def x_direction(self):
        x = c_double(0.0)
        y = c_double(0.0)
        z = c_double(0.0)
        Base().dll.AUTDTransXDirection(self._cnt, self._tr_id, byref(x), byref(y), byref(z))
        return np.array([x.value, y.value, z.value])

    @ property
    def y_direction(self):
        x = c_double(0.0)
        y = c_double(0.0)
        z = c_double(0.0)
        Base().dll.AUTDTransYDirection(self._cnt, self._tr_id, byref(x), byref(y), byref(z))
        return np.array([x.value, y.value, z.value])

    @ property
    def z_direction(self):
        x = c_double(0.0)
        y = c_double(0.0)
        z = c_double(0.0)
        Base().dll.AUTDTransZDirection(self._cnt, self._tr_id, byref(x), byref(y), byref(z))
        return np.array([x.value, y.value, z.value])


class Geometry:
    def __init__(self, ptr: c_void_p):
        self._ptr = ptr
        self.__disposed = False

    def __del__(self):
        self.dispose()

    @ property
    def sound_speed(self):
        return Base().dll.AUTDGetSoundSpeed(self._ptr)

    @ sound_speed.setter
    def sound_speed(self, sound_speed: float):
        Base().dll.AUTDSetSoundSpeed(self._ptr, sound_speed)

    @ property
    def attenuation(self):
        return Base().dll.AUTDGetAttenuation(self._ptr)

    @ attenuation.setter
    def attenuation(self, attenuation: float):
        Base().dll.AUTDSetAttenuation(self._ptr, attenuation)

    @ property
    def num_transducers(self) -> int:
        return Base().dll.AUTDNumTransducers(self._ptr)

    @ property
    def num_devices(self) -> int:
        return Base().dll.AUTDNumDevices(self._ptr)

    @property
    def center(self):
        x = c_double(0.0)
        y = c_double(0.0)
        z = c_double(0.0)
        Base().dll.AUTDGeometryCenter(self._ptr, byref(x), byref(y), byref(z))
        return np.array([x.value, y.value, z.value])

    def center_of(self, dev_idx: int):
        x = c_double(0.0)
        y = c_double(0.0)
        z = c_double(0.0)
        Base().dll.AUTDGeometryCenterOf(self._ptr, dev_idx, byref(x), byref(y), byref(z))
        return np.array([x.value, y.value, z.value])

    def __getitem__(self, key: int):
        return Transducer(key, self._ptr)

    class TransdducerIterator:
        def __init__(self, ptr: c_void_p):
            self._ptr = ptr
            self._i = 0
            self._num_trans = Base().dll.AUTDNumTransducers(ptr)

        def __next__(self):
            if self._i == self._num_trans:
                raise StopIteration()
            value = Transducer(self._i, self._ptr)
            self._i += 1
            return value

    def __iter__(self):
        return Geometry.TransdducerIterator(self._ptr)

    def dispose(self):
        if not self.__disposed:
            self._free()
            self.__disposed = True

    def _free(self):
        Base().dll.AUTDFreeGeometry(self._ptr)


class GeometryBuilder:
    def __init__(self):
        self._ptr = c_void_p()
        Base().dll.AUTDCreateGeometryBuilder(byref(self._ptr))

    def add_device(self, pos, rot):
        Base().dll.AUTDAddDevice(self._ptr, pos[0], pos[1], pos[2], rot[0], rot[1], rot[2])
        return self

    def add_device_quaternion(self, pos, q):
        Base().dll.AUTDAddDeviceQuaternion(self._ptr, pos[0], pos[1], pos[2], q[0], q[1], q[2], q[3])
        return self

    def build(self) -> Geometry:
        geometry_ptr = c_void_p()
        Base().dll.AUTDBuildGeometry(byref(geometry_ptr), self._ptr)
        return Geometry(geometry_ptr)


class FirmwareInfo:
    def __init__(self, info: str, matches_version: bool, is_latest: bool):
        self._info = info
        self._matches_version = matches_version
        self._is_latest = is_latest

    @ property
    def info(self):
        return self._info

    @ property
    def matches_version(self):
        return self._matches_version

    @ property
    def is_latest(self):
        return self._is_latest

    @staticmethod
    def latest_version():
        sb = ctypes.create_string_buffer(256)
        Base().dll.AUTDGetLatestFirmware(sb)
        return sb.value.decode('utf-8')

    def __repr__(self):
        return self._info


class Controller:
    def __init__(self, cnt: c_void_p, geometry: Geometry):
        self.p_cnt = cnt
        self._geometry = geometry
        self.__disposed = False

    def __del__(self):
        self.dispose()

    def to_legacy(self):
        Base().dll.AUTDSetMode(self.p_cnt, 0)

    def to_normal(self):
        Base().dll.AUTDSetMode(self.p_cnt, 1)

    def to_normal_phaseself(self):
        Base().dll.AUTDSetMode(self.p_cnt, 2)

    @ property
    def geometry(self):
        return self._geometry

    def set_sound_speed_from_temp(self, temp: float, k: float = 1.4, r: float = 8.31446261815324, m: float = 28.9647e-3):
        Base().dll.AUTDSetSoundSpeed(self.p_cnt, temp, k, r, m)

    @staticmethod
    def open(geometry: Geometry, link: Link):
        cnt = c_void_p()
        if not Base().dll.AUTDOpenController(cnt, geometry._ptr, link.link_ptr):
            raise Exception('Failed to open controller')
        return Controller(cnt, geometry)

    def firmware_info_list(self):
        res = []
        handle = c_void_p()
        size = Base().dll.AUTDGetFirmwareInfoListPointer(self.p_cnt, byref(handle))
        if size < 0:
            raise Exception('Failed to get firmware version.')

        for i in range(size):
            sb = ctypes.create_string_buffer(256)
            matches_version = c_bool(False)
            is_latest = c_bool(False)
            Base().dll.AUTDGetFirmwareInfo(handle, i, sb, byref(matches_version), byref(is_latest))
            info = sb.value.decode('utf-8')
            res.append(FirmwareInfo(info, matches_version, is_latest))

        Base().dll.AUTDFreeFirmwareInfoListPointer(handle)

        return res

    def dispose(self):
        if not self.__disposed:
            self.close()
            self._free()
            self.__disposed = True

    def close(self):
        return Base().dll.AUTDClose(self.p_cnt)

    def _free(self):
        Base().dll.AUTDFreeController(self.p_cnt)

    @ property
    def is_open(self):
        return Base().dll.AUTDIsOpen(self.p_cnt)

    @ property
    def force_fan(self):
        return Base().dll.AUTDGetForceFan(self.p_cnt)

    @ force_fan.setter
    def force_fan(self, value: bool):
        return Base().dll.AUTDSetForceFan(self.p_cnt, value)

    @ property
    def ack_check_timeout_ms(self):
        return Base().dll.AUTDGetAckCheckTimeout(self.p_cnt) / 1000 / 1000

    @ ack_check_timeout_ms.setter
    def ack_check_timeout_ms(self, value: int):
        return Base().dll.AUTDSetAckCheckTimeout(self.p_cnt, value * 1000 * 1000)

    @ property
    def ack_check_timeout_ns(self):
        return Base().dll.AUTDGetAckCheckTimeout(self.p_cnt)

    @ ack_check_timeout_ns.setter
    def ack_check_timeout_ns(self, value: int):
        return Base().dll.AUTDSetAckCheckTimeout(self.p_cnt, value)

    @ property
    def send_interval_ms(self):
        return Base().dll.AUTDGetSendInterval(self.p_cnt) / 1000 / 1000

    @ send_interval_ms.setter
    def send_interval_ms(self, value: int):
        return Base().dll.AUTDSetSendInterval(self.p_cnt, value * 1000 * 1000)

    @ property
    def send_interval_ns(self):
        return Base().dll.AUTDGetSendInterval(self.p_cnt)

    @ send_interval_ns.setter
    def send_interval_ns(self, value: int):
        return Base().dll.AUTDSetSendInterval(self.p_cnt, value)

    @ property
    def reads_fpga_info(self):
        Base().dll.AUTDGetReadsFPGAInfo(self.p_cnt)

    @ reads_fpga_info.setter
    def reads_fpga_info(self, value: bool):
        Base().dll.AUTDSetReadsFPGAInfo(self.p_cnt, value)

    @ property
    def fpga_info(self):
        infos = np.zeros([self.geometry.num_devices]).astype(np.ubyte)
        pinfos = np.ctypeslib.as_ctypes(infos)
        Base().dll.AUTDGetFPGAInfo(self.p_cnt, pinfos)
        return infos

    def send(self, a, b=None):
        if b is None and isinstance(a, SpecialData):
            return Base().dll.AUTDSendSpecial(self.p_cnt, a.ptr)
        if b is None and isinstance(a, Header):
            return Base().dll.AUTDSend(self.p_cnt, a.ptr, None)
        if b is None and isinstance(a, Body):
            return Base().dll.AUTDSend(self.p_cnt, None, a.ptr)
        if isinstance(a, Header) and isinstance(b, Body):
            return Base().dll.AUTDSend(self.p_cnt, a.ptr, b.ptr)
        raise NotImplementedError()


class Amplitudes(Body):
    def __init__(self, amp: float):
        super().__init__()
        Base().dll.AUTDCreateAmplitudes(byref(self.ptr), amp)

    def __del__(self):
        Base().dll.AUTDDeleteAmplitudes(self.ptr)


class Clear(SpecialData):
    def __init__(self):
        super().__init__()
        Base().dll.AUTDClear(byref(self.ptr))

    def __del__(self):
        Base().dll.AUTDDeleteSpecialData(self.ptr)


class Stop(SpecialData):
    def __init__(self):
        super().__init__()
        Base().dll.AUTDStop(byref(self.ptr))

    def __del__(self):
        Base().dll.AUTDDeleteSpecialData(self.ptr)


class UpdateFlag(SpecialData):
    def __init__(self):
        super().__init__()
        Base().dll.AUTDUpdateFlags(byref(self.ptr))

    def __del__(self):
        Base().dll.AUTDDeleteSpecialData(self.ptr)


class Synchronize(SpecialData):
    def __init__(self):
        super().__init__()
        Base().dll.AUTDSynchronize(byref(self.ptr))

    def __del__(self):
        Base().dll.AUTDDeleteSpecialData(self.ptr)


class ModDelayConfig(SpecialData):
    def __init__(self):
        super().__init__()
        Base().dll.AUTDModDelayConfig(byref(self.ptr))

    def __del__(self):
        Base().dll.AUTDDeleteSpecialData(self.ptr)
