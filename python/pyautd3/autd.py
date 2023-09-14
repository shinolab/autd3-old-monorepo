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
    DatagramPtr,
    DatagramSpecialPtr,
    ControllerPtr,
    LinkPtr,
)
from .link.link import Link
from .geometry import Geometry, AUTD3

LogOutputFunc = ctypes.CFUNCTYPE(None, ctypes.c_char_p)
LogFlushFunc = ctypes.CFUNCTYPE(None)


class SpecialDatagram(metaclass=ABCMeta):
    def __init__(self):
        pass

    @abstractmethod
    def ptr(self, _: Geometry) -> DatagramSpecialPtr:
        pass


class Datagram(metaclass=ABCMeta):
    def __init__(self):
        pass

    @abstractmethod
    def ptr(self, geometry: Geometry) -> DatagramPtr:
        pass


class Silencer(Datagram):
    _step: int

    def __init__(self, step: int = 10):
        super().__init__()
        self._step = step

    @staticmethod
    def disable() -> "Silencer":
        return Silencer(0xFFFF)

    def ptr(self, _: Geometry) -> DatagramPtr:
        return Base().create_silencer(self._step)


class FPGAInfo:
    info: ctypes.c_uint8

    def __init__(self, info: ctypes.c_uint8):
        self.info = info

    def is_thermal_assert(self) -> bool:
        return (int(self.info) & 0x01) != 0

    def __str__(self) -> str:
        return f"Thermal assert = {self.is_thermal_assert()}"


class FirmwareInfo:
    _info: str

    def __init__(self, info: str):
        self._info = info

    @property
    def info(self) -> str:
        return self._info

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

        def __init__(self, mode: TransMode):
            self._ptr = Base().create_controller_builder()
            self._mode = mode

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
        return Controller.Builder(TransMode.Legacy)

    @staticmethod
    def legacy_builder() -> "Controller.Builder":
        return Controller.Builder(TransMode.Legacy)

    @staticmethod
    def advanced_builder() -> "Controller.Builder":
        return Controller.Builder(TransMode.Advanced)

    @staticmethod
    def advanced_phase_builder() -> "Controller.Builder":
        return Controller.Builder(TransMode.AdvancedPhase)

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
        return Controller(geometry, ptr, mode)

    def firmware_info_list(self) -> List[FirmwareInfo]:
        err = ctypes.create_string_buffer(256)
        handle = Base().get_firmware_info_list_pointer(self._ptr, err)
        if not handle:
            raise AUTDError(err)

        def get_firmware_info(i: int) -> FirmwareInfo:
            sb = ctypes.create_string_buffer(256)
            Base().get_firmware_info(handle, i, sb)
            info = sb.value.decode("utf-8")
            return FirmwareInfo(info)

        res = list(map(get_firmware_info, range(self.geometry.num_devices)))

        Base().free_firmware_info_list_pointer(handle)

        return res

    def close(self):
        err = ctypes.create_string_buffer(256)
        if not Base().close(self._ptr, err):
            raise AUTDError(err)

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
        d: Union[DatagramSpecialPtr, Datagram, Tuple[Datagram, Datagram]],
        timeout: Optional[timedelta] = None,
    ) -> bool:
        timeout_ = (
            -1 if timeout is None else int(timeout.total_seconds() * 1000 * 1000 * 1000)
        )
        err = ctypes.create_string_buffer(256)
        res: ctypes.c_int32 = ctypes.c_int32(AUTD3_FALSE)
        if isinstance(d, DatagramSpecialPtr):
            res = Base().send_special(self._ptr, self._mode, d.ptr(self.geometry), timeout_, err)
        if isinstance(d, Datagram):
            res = Base().send(
                self._ptr, self._mode, d.ptr(self.geometry), DatagramPtr(None), timeout_, err
            )
        if isinstance(d, tuple) and len(d) == 2:
            (d1, d2) = d
            if isinstance(d1, Datagram) and isinstance(d2, Datagram):
                res = Base().send(
                    self._ptr,
                    self._mode,
                    d1.ptr(self.geometry),
                    d2.ptr(self.geometry),
                    timeout_,
                    err,
                )

        if res == AUTD3_ERR:
            raise AUTDError(err)

        return res == AUTD3_TRUE


class Amplitudes(Datagram):
    _amp: float

    def __init__(self, amp: float):
        super().__init__()
        self._amp = amp

    def ptr(self, _: Geometry) -> DatagramPtr:
        return Base().create_amplitudes(self._amp)


class Clear(Datagram):
    def __init__(self):
        super().__init__()

    def ptr(self, _: Geometry) -> DatagramPtr:
        return Base().clear()


class Stop(SpecialDatagram):
    def __init__(self):
        super().__init__()

    def ptr(self, _: Geometry) -> DatagramSpecialPtr:
        return Base().stop()


class UpdateFlags(Datagram):
    def __init__(self):
        super().__init__()

    def ptr(self, _: Geometry) -> DatagramPtr:
        return Base().update_flags()


class Synchronize(Datagram):
    def __init__(self):
        super().__init__()

    def ptr(self, _: Geometry) -> DatagramPtr:
        return Base().synchronize()


class ConfigureModDelay(Datagram):
    def __init__(self):
        super().__init__()

    def ptr(self, _: Geometry) -> DatagramPtr:
        return Base().configure_mod_delay()


class ConfigureAmpFilter(Datagram):
    def __init__(self):
        super().__init__()

    def ptr(self, _: Geometry) -> DatagramPtr:
        return Base().configure_amp_filter()


class ConfigurePhaseFilter(Datagram):
    def __init__(self):
        super().__init__()

    def ptr(self, _: Geometry) -> DatagramPtr:
        return Base().configure_phase_filter()
