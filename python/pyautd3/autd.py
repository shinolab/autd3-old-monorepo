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


from abc import ABCMeta, abstractmethod
from datetime import timedelta
import ctypes
from ctypes import c_bool
import numpy as np
from typing import List, Optional, Tuple, Union

from .autd_error import AUTDError
from .native_methods.autd3capi import NativeMethods as Base
from .native_methods.autd3capi import ControllerBuilderPtr
from .native_methods.autd3capi_def import (
    TransMode,
    AUTD3_ERR,
    AUTD3_TRUE,
    AUTD3_FALSE,
    DatagramBodyPtr,
    DatagramHeaderPtr,
    DatagramSpecialPtr,
    ControllerPtr,
    LinkPtr,
)
from .link.link import Link
from .geometry import Geometry, AUTD3

LogOutputFunc = ctypes.CFUNCTYPE(None, ctypes.c_char_p)
LogFlushFunc = ctypes.CFUNCTYPE(None)


class SpecialData(metaclass=ABCMeta):
    def __init__(self):
        pass

    @abstractmethod
    def ptr(self) -> DatagramSpecialPtr:
        pass


class Body(metaclass=ABCMeta):
    def __init__(self):
        pass

    @abstractmethod
    def ptr(self, geometry: Geometry) -> DatagramBodyPtr:
        pass


class Header(metaclass=ABCMeta):
    def __init__(self):
        pass

    @abstractmethod
    def ptr(self) -> DatagramHeaderPtr:
        pass


class SilencerConfig(Header):
    _step: int

    def __init__(self, step: int = 10):
        super().__init__()
        self._step = step

    @staticmethod
    def none() -> "SilencerConfig":
        return SilencerConfig(0xFFFF)

    def ptr(self) -> DatagramHeaderPtr:
        return Base().create_silencer(self._step)


class FPGAInfo:
    info: ctypes.c_uint8

    def __init__(self, info: ctypes.c_uint8):
        self.info = info

    def is_thermal_assert(self) -> bool:
        return (int(self.info) & 0x01) != 0


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
    class Builder:
        _ptr: ControllerBuilderPtr
        _mode: TransMode

        def __init__(self):
            self._ptr = Base().create_controller_builder()
            self._mode = TransMode.Legacy

        def add_device(self, device: AUTD3) -> "Controller.Builder":
            if device._rot is not None:
                self._ptr = Base().add_device(
                    self._ptr,
                    device._pos[0],
                    device._pos[1],
                    device._pos[2],
                    device._rot[0],
                    device._rot[1],
                    device._rot[2],
                )
            elif device._quat is not None:
                self._ptr = Base().add_device_quaternion(
                    self._ptr,
                    device._pos[0],
                    device._pos[1],
                    device._pos[2],
                    device._quat[0],
                    device._quat[1],
                    device._quat[2],
                    device._quat[3],
                )
            return self

        def legacy_mode(self) -> "Controller.Builder":
            self._mode = TransMode.Legacy
            return self

        def advanced_mode(self) -> "Controller.Builder":
            self._mode = TransMode.Advanced
            return self

        def advanced_phase_mode(self) -> "Controller.Builder":
            self._mode = TransMode.AdvancedPhase
            return self

        def open_with(self, link: Link) -> "Controller":
            return Controller._open_impl(self._ptr, self._mode, link.ptr())

    _geometry: Geometry
    _ptr: ControllerPtr
    _mode: TransMode

    def __init__(self, geometry: Geometry, ptr: ControllerPtr, mode: TransMode):
        self._geometry = geometry
        self._ptr = ptr
        self._mode = mode

    @staticmethod
    def builder() -> "Controller.Builder":
        return Controller.Builder()

    def __del__(self):
        self.dispose()

    def dispose(self):
        if self._ptr._0 is not None:
            Base().free_controller(self._ptr)
            self._ptr._0 = None

    @property
    def geometry(self) -> Geometry:
        return self._geometry

    @staticmethod
    def _open_impl(
        builder: ControllerBuilderPtr, mode: TransMode, link: LinkPtr
    ) -> "Controller":
        err = ctypes.create_string_buffer(256)
        ptr = Base().controller_open_with(builder, link, err)
        if ptr._0 is None:
            raise AUTDError(err)
        geometry = Geometry(Base().get_geometry(ptr), mode)
        cnt = Controller(geometry, ptr, mode)
        cnt.geometry._configure()
        return cnt

    def firmware_info_list(self) -> List[FirmwareInfo]:
        err = ctypes.create_string_buffer(256)
        handle = Base().get_firmware_info_list_pointer(self._ptr, err)
        if not handle:
            raise AUTDError(err)

        def get_firmware_info(i: int) -> FirmwareInfo:
            sb = ctypes.create_string_buffer(256)
            props = np.zeros([2]).astype(c_bool)
            propsp = np.ctypeslib.as_ctypes(props)
            Base().get_firmware_info(handle, i, sb, propsp)
            info = sb.value.decode("utf-8")
            return FirmwareInfo(info, props[0], props[1])

        res = list(map(get_firmware_info, range(self.geometry.num_devices)))

        Base().free_firmware_info_list_pointer(handle)

        return res

    def close(self):
        err = ctypes.create_string_buffer(256)
        if not Base().close(self._ptr, err):
            raise AUTDError(err)

    def force_fan(self, value: bool):
        return Base().set_force_fan(self._ptr, value)

    def reads_fpga_info(self, value: bool):
        Base().set_reads_fpga_info(self._ptr, value)

    @property
    def fpga_info(self) -> List[FPGAInfo]:
        infos = np.zeros([self.geometry.num_devices]).astype(ctypes.c_uint8)
        pinfos = np.ctypeslib.as_ctypes(infos)
        err = ctypes.create_string_buffer(256)
        if not Base().get_fpga_info(self._ptr, pinfos, err):
            raise AUTDError(err)
        return list(map(lambda x: FPGAInfo(x), infos))

    def send(
        self,
        d: Union[SpecialData, Header, Body, Tuple[Header, Body]],
        timeout: Optional[timedelta] = None,
    ) -> bool:
        timeout_ = (
            -1 if timeout is None else int(timeout.total_seconds() * 1000 * 1000 * 1000)
        )
        err = ctypes.create_string_buffer(256)
        res: ctypes.c_int32 = ctypes.c_int32(AUTD3_FALSE)
        if isinstance(d, SpecialData):
            res = Base().send_special(self._ptr, self._mode, d.ptr(), timeout_, err)
        if isinstance(d, Header):
            res = Base().send(
                self._ptr, self._mode, d.ptr(), DatagramBodyPtr(None), timeout_, err
            )
        if isinstance(d, Body):
            res = Base().send(
                self._ptr,
                self._mode,
                DatagramHeaderPtr(None),
                d.ptr(self.geometry),
                timeout_,
                err,
            )
        if isinstance(d, tuple) and len(d) == 2:
            (h, b) = d
            if isinstance(h, Header) and isinstance(b, Body):
                res = Base().send(
                    self._ptr,
                    self._mode,
                    h.ptr(),
                    b.ptr(self.geometry),
                    timeout_,
                    err,
                )

        if res == AUTD3_ERR:
            raise AUTDError(err)

        return res == AUTD3_TRUE


class Amplitudes(Body):
    _amp: float

    def __init__(self, amp: float):
        super().__init__()
        self._amp = amp

    def ptr(self, geometry: Geometry) -> DatagramBodyPtr:
        return Base().create_amplitudes(self._amp)


class Clear(SpecialData):
    def __init__(self):
        super().__init__()

    def ptr(self) -> DatagramSpecialPtr:
        return Base().clear()


class Stop(SpecialData):
    def __init__(self):
        super().__init__()

    def ptr(self) -> DatagramSpecialPtr:
        return Base().stop()


class UpdateFlags(SpecialData):
    def __init__(self):
        super().__init__()

    def ptr(self) -> DatagramSpecialPtr:
        return Base().update_flags()


class Synchronize(SpecialData):
    def __init__(self):
        super().__init__()

    def ptr(self) -> DatagramSpecialPtr:
        return Base().synchronize()


class ModDelayConfig(SpecialData):
    def __init__(self):
        super().__init__()

    def ptr(self) -> DatagramSpecialPtr:
        return Base().mod_delay_config()
