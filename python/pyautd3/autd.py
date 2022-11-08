'''
File: autd.py
Project: pyautd3
Created Date: 24/05/2021
Author: Shun Suzuki
-----
Last Modified: 08/11/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''


import ctypes
from ctypes import c_void_p, byref, c_double
import numpy as np

from .native_methods.autd3capi import NativeMethods as Base
from .link.link import Link


NUM_TRANS_IN_UNIT = 249
NUM_TRANS_X = 18
NUM_TRANS_Y = 14
TRANS_SPACING = 10.16
DEVICE_WIDTH = 192.0
DEVICE_HEIGHT = 151.4


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
    def __init__(self, dev_id: int, tr_id: int, cnt: c_void_p):
        self._dev_id = dev_id
        self._tr_id = tr_id
        self._cnt = cnt

    @ property
    def id(self) -> int:
        return NUM_TRANS_IN_UNIT * self._dev_id + self._tr_id

    @ property
    def position(self):
        x = c_double(0.0)
        y = c_double(0.0)
        z = c_double(0.0)
        Base().dll.AUTDTransPosition(self._cnt, self._dev_id, self._tr_id, byref(x), byref(y), byref(z))
        return np.array([x.value, y.value, z.value])

    @ property
    def frequency(self):
        return Base().dll.AUTDGetTransFrequency(self._cnt, self._dev_id, self._tr_id)

    @ frequency.setter
    def frequency(self, freq: float):
        return Base().dll.AUTDSetTransFrequency(self._cnt, self._dev_id, self._tr_id, freq)

    @ property
    def cycle(self):
        return Base().dll.AUTDGetTransCycle(self._cnt, self._dev_id, self._tr_id)

    @ cycle.setter
    def cycle(self, cycle: int):
        return Base().dll.AUTDSetTransCycle(self._cnt, self._dev_id, self._tr_id, cycle)

    @ property
    def mod_delay(self):
        return Base().dll.AUTDGetModDelay(self._cnt, self._dev_id, self._tr_id)

    @ mod_delay.setter
    def mod_delay(self, delay: int):
        return Base().dll.AUTDSetModDelay(self._cnt, self._dev_id, self._tr_id, delay)

    @ property
    def wavelength(self):
        return Base().dll.AUTDGetWavelength(self._cnt, self._dev_id, self._tr_id)

    @ property
    def x_direction(self):
        x = c_double(0.0)
        y = c_double(0.0)
        z = c_double(0.0)
        Base().dll.AUTDTransXDirection(self._cnt, self._dev_id, self._tr_id, byref(x), byref(y), byref(z))
        return np.array([x.value, y.value, z.value])

    @ property
    def y_direction(self):
        x = c_double(0.0)
        y = c_double(0.0)
        z = c_double(0.0)
        Base().dll.AUTDTransYDirection(self._cnt, self._dev_id, self._tr_id, byref(x), byref(y), byref(z))
        return np.array([x.value, y.value, z.value])

    @ property
    def z_direction(self):
        x = c_double(0.0)
        y = c_double(0.0)
        z = c_double(0.0)
        Base().dll.AUTDTransZDirection(self._cnt, self._dev_id, self._tr_id, byref(x), byref(y), byref(z))
        return np.array([x.value, y.value, z.value])


class Device:
    def __init__(self, id: int, cnt: c_void_p):
        self._id = id
        self._cnt = cnt

    @property
    def origin(self):
        return Transducer(self._id, 0, self._cnt).position

    @property
    def center(self):
        return sum(map(lambda x: x.position, self)) / NUM_TRANS_IN_UNIT

    def __getitem__(self, key):
        return Transducer(self._id, key, self._cnt)

    class TransducerIterator:
        def __init__(self, dev_id: int, cnt: c_void_p):
            self._dev_id = dev_id
            self._cnt = cnt
            self._i = 0

        def __next__(self):
            if self._i == NUM_TRANS_IN_UNIT:
                raise StopIteration()
            value = Transducer(self._dev_id, self._i, self._cnt)
            self._i += 1
            return value

    def __iter__(self):
        return Device.TransducerIterator(self._id, self._cnt)


class Geometry:
    def __init__(self, cnt: c_void_p):
        self._cnt = cnt

    def add_device(self, pos, rot):
        return Base().dll.AUTDAddDevice(self._cnt, pos[0], pos[1], pos[2], rot[0], rot[1], rot[2])

    def add_device_quaternion(self, pos, q):
        return Base().dll.AUTDAddDeviceQuaternion(self._cnt, pos[0], pos[1], pos[2], q[0], q[1], q[2], q[3])

    @ property
    def sound_speed(self):
        return Base().dll.AUTDGetSoundSpeed(self._cnt)

    @ sound_speed.setter
    def sound_speed(self, sound_speed: float):
        Base().dll.AUTDSetSoundSpeed(self._cnt, sound_speed)

    @ property
    def attenuation(self):
        return Base().dll.AUTDGetAttenuation(self._cnt)

    @ attenuation.setter
    def attenuation(self, attenuation: float):
        Base().dll.AUTDSetAttenuation(self._cnt, attenuation)

    @ property
    def num_devices(self):
        return Base().dll.AUTDNumDevices(self._cnt)

    @ property
    def num_transducers(self):
        return self.num_devices * NUM_TRANS_IN_UNIT

    @property
    def center(self):
        return sum(map(lambda x: x.center, self)) / self.num_devices

    def __getitem__(self, key):
        return Device(key, self._cnt)

    class DeviceIterator:
        def __init__(self, dev_len: int, cnt: c_void_p):
            self._dev_len = dev_len
            self._cnt = cnt
            self._i = 0

        def __next__(self):
            if self._i == self._dev_len:
                raise StopIteration()
            value = Device(self._i, self._cnt)
            self._i += 1
            return value

    def __iter__(self):
        return Geometry.DeviceIterator(self.num_devices, self._cnt)


class Controller:
    def __init__(self):
        self.p_cnt = c_void_p()
        Base().dll.AUTDCreateController(byref(self.p_cnt))
        self.__disposed = False

    def __del__(self):
        self.dispose()

    def last_error():
        size = Base().dll.AUTDGetLastError(None)
        err = ctypes.create_string_buffer(size)
        Base().dll.AUTDGetLastError(err)
        return err.value.decode('utf-8')

    def to_legacy(self):
        Base().dll.AUTDSetMode(self.p_cnt, 0)

    def to_normal(self):
        Base().dll.AUTDSetMode(self.p_cnt, 1)

    def to_normal_phaseself(self):
        Base().dll.AUTDSetMode(self.p_cnt, 2)

    @ property
    def geometry(self):
        return Geometry(self.p_cnt)

    def open(self, link: Link):
        return Base().dll.AUTDOpenController(self.p_cnt, link.link_ptr)

    def firmware_info_list(self):
        res = []
        handle = c_void_p()
        size = Base().dll.AUTDGetFirmwareInfoListPointer(self.p_cnt, byref(handle))

        for i in range(size):
            sb = ctypes.create_string_buffer(256)
            Base().dll.AUTDGetFirmwareInfo(handle, i, sb)
            res.append(sb.value.decode('utf-8'))

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
    def check_trials(self):
        return Base().dll.AUTDGetCheckTrials(self.p_cnt)

    @ check_trials.setter
    def check_trials(self, value: int):
        return Base().dll.AUTDSetCheckTrials(self.p_cnt, value)

    @ property
    def send_interval(self):
        return Base().dll.AUTDGetSendInterval(self.p_cnt)

    @ send_interval.setter
    def send_interval(self, value: int):
        return Base().dll.AUTDSetSendInterval(self.p_cnt, value)

    @ property
    def reads_fpga_info(self):
        Base().dll.AUTDGetReadsFPGAInfo(self.p_cnt)

    @ reads_fpga_info.setter
    def reads_fpga_info(self, value: bool):
        Base().dll.AUTDSetReadsFPGAInfo(self.p_cnt, value)

    @ property
    def fpga_info(self):
        infos = np.zeros([self.num_devices()]).astype(np.ubyte)
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

    def send_async(self, a, b=None):
        if b is None and isinstance(a, SpecialData):
            Base().dll.AUTDSendSpecialAsync(self.p_cnt, a.ptr)
            a.ptr = c_void_p()
            return
        if b is None and isinstance(a, Header):
            Base().dll.AUTDSendAsync(self.p_cnt, a.ptr, None)
            a.ptr = c_void_p()
            return
        if b is None and isinstance(a, Body):
            Base().dll.AUTDSendAsync(self.p_cnt, None, a.ptr)
            a.ptr = c_void_p()
            return
        if isinstance(a, Header) and isinstance(b, Body):
            Base().dll.AUTDSendAsync(self.p_cnt, a.ptr, b.ptr)
            a.ptr = c_void_p()
            b.ptr = c_void_p()
            return
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
