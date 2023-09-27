'''
File: autd.py
Project: pyautd3
Created Date: 24/05/2021
Author: Shun Suzuki
-----
Last Modified: 27/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

'''


from abc import ABCMeta, abstractmethod
from datetime import timedelta
import ctypes
import numpy as np
from typing import Callable, Dict, Generic, List, Optional, Tuple, TypeVar, Union

from .autd_error import AUTDError
from .native_methods.autd3capi import NativeMethods as Base
from .native_methods.autd3capi import ControllerBuilderPtr
from .native_methods.autd3capi_def import (
    GroupKVMapPtr,
    TimerStrategy,
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
from .geometry import Device, Geometry, AUTD3

K = TypeVar("K")

LogOutputFunc = ctypes.CFUNCTYPE(None, ctypes.c_char_p)
LogFlushFunc = ctypes.CFUNCTYPE(None)


class SpecialDatagram(metaclass=ABCMeta):
    def __init__(self):
        pass

    @abstractmethod
    def ptr(self) -> DatagramSpecialPtr:
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
        return Base().datagram_silencer(self._step)


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
        Base().firmware_latest(sb)
        return sb.value.decode("utf-8")

    def __repr__(self) -> str:
        return self._info


class Controller:
    class Builder:
        _ptr: ControllerBuilderPtr
        _mode: TransMode

        def __init__(self):
            self._ptr = Base().controller_builder()
            self._mode = TransMode.Legacy

        def legacy(self) -> "Controller.Builder":
            self._mode = TransMode.Legacy
            return self

        def advanced(self) -> "Controller.Builder":
            self._mode = TransMode.Advanced
            return self

        def advanced_phase(self) -> "Controller.Builder":
            self._mode = TransMode.AdvancedPhase
            return self

        def add_device(self, device: AUTD3) -> "Controller.Builder":
            if device._rot is not None:
                self._ptr = Base().controller_builder_add_device(
                    self._ptr,
                    device._pos[0],
                    device._pos[1],
                    device._pos[2],
                    device._rot[0],
                    device._rot[1],
                    device._rot[2],
                )
            elif device._quat is not None:
                self._ptr = Base().controller_builder_add_device_quaternion(
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
        return Controller.Builder()

    def __del__(self):
        self.dispose()

    def dispose(self):
        if self._ptr._0 is not None:
            Base().controller_delete(self._ptr)
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
        geometry = Geometry(Base().geometry(ptr), mode)
        return Controller(geometry, ptr, mode)

    def firmware_info_list(self) -> List[FirmwareInfo]:
        err = ctypes.create_string_buffer(256)
        handle = Base().controller_firmware_info_list_pointer(self._ptr, err)
        if handle._0 is None:
            raise AUTDError(err)

        def get_firmware_info(i: int) -> FirmwareInfo:
            sb = ctypes.create_string_buffer(256)
            Base().controller_firmware_info_get(handle, i, sb)
            info = sb.value.decode("utf-8")
            return FirmwareInfo(info)

        res = list(map(get_firmware_info, range(self.geometry.num_devices)))

        Base().controller_firmware_info_list_pointer_delete(handle)

        return res

    def close(self):
        err = ctypes.create_string_buffer(256)
        if not Base().controller_close(self._ptr, err):
            raise AUTDError(err)

    @property
    def fpga_info(self) -> List[FPGAInfo]:
        infos = np.zeros([self.geometry.num_devices]).astype(ctypes.c_uint8)
        pinfos = np.ctypeslib.as_ctypes(infos)
        err = ctypes.create_string_buffer(256)
        if not Base().controller_fpga_info(self._ptr, pinfos, err):
            raise AUTDError(err)
        return list(map(lambda x: FPGAInfo(x), infos))

    def send(
        self,
        d: Union[SpecialDatagram, Datagram, Tuple[Datagram, Datagram]],
        timeout: Optional[timedelta] = None,
    ) -> bool:
        timeout_ = (
            -1 if timeout is None else int(timeout.total_seconds() * 1000 * 1000 * 1000)
        )
        err = ctypes.create_string_buffer(256)
        res: ctypes.c_int32 = ctypes.c_int32(AUTD3_FALSE)
        if isinstance(d, SpecialDatagram):
            res = Base().controller_send_special(self._ptr, self._mode, d.ptr(), timeout_, err)
        if isinstance(d, Datagram):
            res = Base().controller_send(
                self._ptr, self._mode, d.ptr(self.geometry), DatagramPtr(None), timeout_, err
            )
        if isinstance(d, tuple) and len(d) == 2:
            (d1, d2) = d
            if isinstance(d1, Datagram) and isinstance(d2, Datagram):
                res = Base().controller_send(
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

    class SoftwareSTM():
        class Context():
            controller: "Controller"
            callback: Callable[["Controller", int, timedelta], bool]

            def __init__(self, controller: "Controller", callback: Callable[["Controller", int, timedelta], bool]):
                self.controller = controller
                self.callback = callback

        _ptr: ControllerPtr
        _context: Context
        _strategy: TimerStrategy

        def __init__(self, ptr: ControllerPtr, context: Context):
            self._ptr = ptr
            self._context = context
            self._strategy = TimerStrategy.Sleep

        def start(self, interval: timedelta):
            interval_ns = int(interval.total_seconds() * 1000 * 1000 * 1000)

            def callback_native(ptr: ctypes.c_void_p, i: ctypes.c_uint64, elapsed: ctypes.c_uint64):
                context_ptr = ctypes.cast(ptr, ctypes.POINTER(ctypes.py_object)).contents
                context: Controller.SoftwareSTM.Context = context_ptr.value
                return context.callback(context.controller, int(i), timedelta(seconds=int(elapsed) / 1000 / 1000 / 1000))

            callback_f = ctypes.CFUNCTYPE(ctypes.c_bool, ctypes.c_void_p, ctypes.c_uint64, ctypes.c_uint64)(callback_native)

            context = ctypes.py_object(self._context)
            ptr = ctypes.cast(ctypes.pointer(context), ctypes.c_void_p)

            err = ctypes.create_string_buffer(256)

            if Base().controller_software_stm(
                self._ptr,
                callback_f,  # type: ignore
                ptr,
                self._strategy,
                interval_ns,
                err
            ) == AUTD3_ERR:
                raise AUTDError(err)

        def with_timer_strategy(self, strategy: TimerStrategy) -> "Controller.SoftwareSTM":
            self._strategy = strategy
            return self

    def software_stm(self, callback: Callable[["Controller", int, timedelta], bool]) -> "Controller.SoftwareSTM":
        return Controller.SoftwareSTM(self._ptr, Controller.SoftwareSTM.Context(self, callback))

    class GroupGuard(Generic[K]):
        _controller: "Controller"
        _map: Callable[[Device], Optional[K]]
        _kv_map: GroupKVMapPtr
        _keymap: Dict[K, int]
        _k: int

        def __init__(self, map: Callable[[Device], Optional[K]], controller: "Controller"):
            self._map = map
            self._controller = controller
            self._kv_map = Base().controller_group_create_kv_map()
            self._keymap = {}
            self._k = 0

        def set(self,
                key: K,
                d: Union[SpecialDatagram, Datagram, Tuple[Datagram, Datagram]],
                timeout: Optional[timedelta] = None,
                ) -> "Controller.GroupGuard":
            if key in self._keymap:
                e = AUTDError()
                e.msg = "Key is already exists"
                raise e

            timeout_ns = (
                -1 if timeout is None else int(timeout.total_seconds() * 1000 * 1000 * 1000)
            )

            err = ctypes.create_string_buffer(256)
            if isinstance(d, SpecialDatagram):
                self._keymap[key] = self._k
                self._kv_map = Base().controller_group_kv_map_set_special(self._kv_map, self._k, d.ptr(), self._controller._mode, timeout_ns, err)
                self._k += 1
                if self._kv_map._0 is None:
                    raise AUTDError(err)
            if isinstance(d, Datagram):
                self._keymap[key] = self._k
                self._kv_map = Base().controller_group_kv_map_set(self._kv_map, self._k, d.ptr(
                    self._controller._geometry), DatagramPtr(None), self._controller._mode, timeout_ns, err)
                self._k += 1
                if self._kv_map._0 is None:
                    raise AUTDError(err)

            if isinstance(d, tuple) and len(d) == 2:
                (d1, d2) = d
                if isinstance(d1, Datagram) and isinstance(d2, Datagram):
                    self._keymap[key] = self._k
                    self._kv_map = Base().controller_group_kv_map_set(
                        self._kv_map, self._k, d1.ptr(
                            self._controller._geometry), d2.ptr(
                            self._controller._geometry), self._controller._mode, timeout_ns, err)
                    self._k += 1
                    if self._kv_map._0 is None:
                        raise AUTDError(err)

            return self

        def send(self):
            m = np.fromiter(map(lambda k: self._keymap[k] if k is not None else -
                                1, map(lambda dev: self._map(dev) if dev.enable else None, self._controller.geometry)), dtype=np.int32)
            err = ctypes.create_string_buffer(256)
            if Base().controller_group(self._controller._ptr, np.ctypeslib.as_ctypes(m.astype(ctypes.c_int32)), self._kv_map, err) == AUTD3_ERR:
                raise AUTDError(err)

    def group(self, map: Callable[[Device], Optional[K]]) -> "Controller.GroupGuard":
        return Controller.GroupGuard(map, self)


class Amplitudes(Datagram):
    _amp: float

    def __init__(self, amp: float):
        super().__init__()
        self._amp = amp

    def ptr(self, _: Geometry) -> DatagramPtr:
        return Base().datagram_amplitudes(self._amp)


class Clear(Datagram):
    def __init__(self):
        super().__init__()

    def ptr(self, _: Geometry) -> DatagramPtr:
        return Base().datagram_clear()


class Stop(SpecialDatagram):
    def __init__(self):
        super().__init__()

    def ptr(self) -> DatagramSpecialPtr:
        return Base().datagram_stop()


class UpdateFlags(Datagram):
    def __init__(self):
        super().__init__()

    def ptr(self, _: Geometry) -> DatagramPtr:
        return Base().datagram_update_flags()


class Synchronize(Datagram):
    def __init__(self):
        super().__init__()

    def ptr(self, _: Geometry) -> DatagramPtr:
        return Base().datagram_synchronize()


class ConfigureModDelay(Datagram):
    def __init__(self):
        super().__init__()

    def ptr(self, _: Geometry) -> DatagramPtr:
        return Base().datagram_configure_mod_delay()


class ConfigureAmpFilter(Datagram):
    def __init__(self):
        super().__init__()

    def ptr(self, _: Geometry) -> DatagramPtr:
        return Base().datagram_configure_amp_filter()


class ConfigurePhaseFilter(Datagram):
    def __init__(self):
        super().__init__()

    def ptr(self, _: Geometry) -> DatagramPtr:
        return Base().datagram_configure_phase_filter()
