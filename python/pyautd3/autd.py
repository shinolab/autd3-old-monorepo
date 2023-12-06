"""
File: autd.py
Project: pyautd3
Created Date: 24/05/2021
Author: Shun Suzuki
-----
Last Modified: 25/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""


import asyncio
import ctypes
from collections.abc import Callable
from datetime import timedelta
from typing import Generic, TypeVar

import numpy as np

from .autd_error import InvalidDatagramTypeError, KeyAlreadyExistsError
from .geometry import AUTD3, Device, Geometry, Transducer
from .internal.datagram import Datagram, SpecialDatagram
from .internal.link import Link, LinkBuilder
from .internal.utils import _validate_int, _validate_ptr
from .native_methods.autd3capi import ControllerBuilderPtr
from .native_methods.autd3capi import NativeMethods as Base
from .native_methods.autd3capi_def import (
    AUTD3_TRUE,
    ControllerPtr,
    DatagramPtr,
    DatagramSpecialPtr,
    GeometryPtr,
    GroupKVMapPtr,
)

K = TypeVar("K")
L = TypeVar("L", bound=Link)


LogOutputFunc = ctypes.CFUNCTYPE(None, ctypes.c_char_p)
LogFlushFunc = ctypes.CFUNCTYPE(None)


class Silencer(Datagram):
    """Datagram for configure silencer."""

    _step_intensity: int
    _step_phase: int

    def __init__(self: "Silencer", step_intensity: int = 256, step_phase: int = 256) -> None:
        """Constructor.

        Arguments:
        ---------
            step_intensity: The intensity update step of silencer. The lower the value, the stronger the silencer effect.
            step_phase: The phase update step of silencer. The lower the value, the stronger the silencer effect.
        """
        super().__init__()
        self._step_intensity = step_intensity
        self._step_phase = step_phase

    @staticmethod
    def disable() -> "Silencer":
        """Disable silencer."""
        return Silencer(0xFFFF, 0xFFFF)

    def _datagram_ptr(self: "Silencer", _: Geometry) -> DatagramPtr:
        return _validate_ptr(Base().datagram_silencer(self._step_intensity, self._step_phase))


class FPGAInfo:
    """FPGA information."""

    info: ctypes.c_uint8

    def __init__(self: "FPGAInfo", info: ctypes.c_uint8) -> None:
        self.info = info

    def is_thermal_assert(self: "FPGAInfo") -> bool:
        """Check if thermal sensor is asserted."""
        return (int(self.info) & 0x01) != 0

    def __str__(self: "FPGAInfo") -> str:
        return f"Thermal assert = {self.is_thermal_assert()}"


class FirmwareInfo:
    """Firmware information."""

    _info: str

    def __init__(self: "FirmwareInfo", info: str) -> None:
        self._info = info

    @property
    def info(self: "FirmwareInfo") -> str:
        """Get firmware information."""
        return self._info

    @staticmethod
    def latest_version() -> str:
        """Get latest firmware version."""
        sb = ctypes.create_string_buffer(256)
        Base().firmware_latest(sb)
        return sb.value.decode("utf-8")

    def __str__(self: "FirmwareInfo") -> str:
        return self._info


class _Builder(Generic[L]):
    _ptr: ControllerBuilderPtr

    def __init__(self: "_Builder[L]") -> None:
        self._ptr = Base().controller_builder()

    def add_device(self: "_Builder[L]", device: AUTD3) -> "_Builder[L]":
        """Add device.

        Arguments:
        ---------
            device: Device to add
        """
        q = device._rot if device._rot is not None else np.array([1.0, 0.0, 0.0, 0.0])
        self._ptr = Base().controller_builder_add_device(
            self._ptr,
            device._pos[0],
            device._pos[1],
            device._pos[2],
            q[0],
            q[1],
            q[2],
            q[3],
        )
        return self

    async def open_with_async(self: "_Builder[L]", link: LinkBuilder[L]) -> "Controller[L]":
        """Open controller.

        Arguments:
        ---------
            link: LinkBuilder
        """
        return await Controller._open_impl_async(self._ptr, link)

    def open_with(self: "_Builder[L]", link: LinkBuilder[L]) -> "Controller[L]":
        """Open controller.

        Arguments:
        ---------
            link: LinkBuilder
        """
        return Controller._open_impl(self._ptr, link)


class Controller(Generic[L]):
    """Controller."""

    _geometry: Geometry
    _ptr: ControllerPtr
    link: L

    def __init__(self: "Controller", geometry: Geometry, ptr: ControllerPtr, link: L) -> None:
        self._geometry = geometry
        self._ptr = ptr
        self.link = link

    @staticmethod
    def builder() -> "_Builder[L]":
        """Create builder."""
        return _Builder()

    def __del__(self: "Controller") -> None:
        self._dispose()

    def _dispose(self: "Controller") -> None:
        if self._ptr._0 is not None:
            Base().controller_delete(self._ptr)
            self._ptr._0 = None

    def __enter__(self: "Controller[L]") -> "Controller[L]":
        return self

    def __exit__(self: "Controller[L]", *args: object) -> None:
        self._dispose()

    @property
    def geometry(self: "Controller") -> Geometry:
        """Get geometry."""
        return self._geometry

    @staticmethod
    async def _open_impl_async(builder: ControllerBuilderPtr, link_builder: LinkBuilder[L]) -> "Controller[L]":
        future: asyncio.Future = asyncio.Future()
        loop = asyncio.get_event_loop()
        loop.call_soon(
            lambda *_: future.set_result(
                Base().controller_open_with(
                    builder,
                    link_builder._link_builder_ptr(),
                ),
            ),
        )
        ptr = _validate_ptr(await future)
        geometry = Geometry(Base().geometry(ptr))
        link = link_builder._resolve_link(ptr)
        return Controller(geometry, ptr, link)

    @staticmethod
    def _open_impl(builder: ControllerBuilderPtr, link_builder: LinkBuilder[L]) -> "Controller[L]":
        ptr = _validate_ptr(
            Base().controller_open_with(
                builder,
                link_builder._link_builder_ptr(),
            ),
        )
        geometry = Geometry(Base().geometry(ptr))
        link = link_builder._resolve_link(ptr)
        return Controller(geometry, ptr, link)

    async def firmware_info_list_async(self: "Controller") -> list[FirmwareInfo]:
        """Get firmware information list."""
        future: asyncio.Future = asyncio.Future()
        loop = asyncio.get_event_loop()
        loop.call_soon(
            lambda *_: future.set_result(Base().controller_firmware_info_list_pointer(self._ptr)),
        )
        handle = _validate_ptr(await future)

        def get_firmware_info(i: int) -> FirmwareInfo:
            sb = ctypes.create_string_buffer(256)
            Base().controller_firmware_info_get(handle, i, sb)
            info = sb.value.decode("utf-8")
            return FirmwareInfo(info)

        res = list(map(get_firmware_info, range(self.geometry.num_devices)))
        Base().controller_firmware_info_list_pointer_delete(handle)
        return res

    def firmware_info_list(self: "Controller") -> list[FirmwareInfo]:
        """Get firmware information list."""
        handle = _validate_ptr(Base().controller_firmware_info_list_pointer(self._ptr))

        def get_firmware_info(i: int) -> FirmwareInfo:
            sb = ctypes.create_string_buffer(256)
            Base().controller_firmware_info_get(handle, i, sb)
            info = sb.value.decode("utf-8")
            return FirmwareInfo(info)

        res = list(map(get_firmware_info, range(self.geometry.num_devices)))
        Base().controller_firmware_info_list_pointer_delete(handle)
        return res

    async def close_async(self: "Controller") -> bool:
        """Close controller."""
        future: asyncio.Future = asyncio.Future()
        loop = asyncio.get_event_loop()
        loop.call_soon(
            lambda *_: future.set_result(
                Base().controller_close(
                    self._ptr,
                ),
            ),
        )
        return _validate_int(await future) == AUTD3_TRUE

    def close(self: "Controller") -> bool:
        """Close controller."""
        return _validate_int(Base().controller_close(self._ptr)) == AUTD3_TRUE

    async def fpga_info_async(self: "Controller") -> list[FPGAInfo]:
        """Get FPGA information list."""
        infos = np.zeros([self.geometry.num_devices]).astype(ctypes.c_uint8)
        pinfos = np.ctypeslib.as_ctypes(infos)
        future: asyncio.Future = asyncio.Future()
        loop = asyncio.get_event_loop()
        loop.call_soon(
            lambda *_: future.set_result(Base().controller_fpga_info(self._ptr, pinfos)),
        )
        _validate_int(await future)
        return [FPGAInfo(x) for x in infos]

    def fpga_info(self: "Controller") -> list[FPGAInfo]:
        """Get FPGA information list."""
        infos = np.zeros([self.geometry.num_devices]).astype(ctypes.c_uint8)
        pinfos = np.ctypeslib.as_ctypes(infos)
        _validate_int(Base().controller_fpga_info(self._ptr, pinfos))
        return [FPGAInfo(x) for x in infos]

    async def send_async(
        self: "Controller",
        d1: SpecialDatagram | Datagram | tuple[Datagram, Datagram],
        d2: Datagram | None = None,
        *,
        timeout: timedelta | None = None,
    ) -> bool:
        """Send data.

        Arguments:
        ---------
            d1: Data to send
            d2: Data to send
            timeout: Timeout

        Returns:
        -------
            bool: If true, it is confirmed that the data has been successfully transmitted.
                  If false, there are no errors, but it is unclear whether the data has been sent reliably or not.

        Raises:
        ------
            AUTDError: If an error occurs
        """
        timeout_ = -1 if timeout is None else int(timeout.total_seconds() * 1000 * 1000 * 1000)
        future: asyncio.Future = asyncio.Future()
        loop = asyncio.get_event_loop()
        match (d1, d2):
            case (SpecialDatagram(), None):
                ds_ptr = d1._special_datagram_ptr()  # type: ignore[union-attr]
                loop.call_soon(
                    lambda *_: future.set_result(
                        Base().controller_send_special(
                            self._ptr,
                            ds_ptr,
                            timeout_,
                        ),
                    ),
                )
            case (Datagram(), None):
                d_ptr: DatagramPtr = d1._datagram_ptr(self.geometry)  # type: ignore[union-attr]
                loop.call_soon(
                    lambda *_: future.set_result(
                        Base().controller_send(
                            self._ptr,
                            d_ptr,
                            DatagramPtr(None),
                            timeout_,
                        ),
                    ),
                )
            case ((Datagram(), Datagram()), None):
                (d11, d12) = d1  # type: ignore[misc]
                d11_ptr: DatagramPtr = d11._datagram_ptr(self.geometry)
                d22_ptr: DatagramPtr = d12._datagram_ptr(self.geometry)
                loop.call_soon(
                    lambda *_: future.set_result(
                        Base().controller_send(
                            self._ptr,
                            d11_ptr,
                            d22_ptr,
                            timeout_,
                        ),
                    ),
                )
            case (Datagram(), Datagram()):
                d1_ptr: DatagramPtr = d1._datagram_ptr(self.geometry)  # type: ignore[union-attr]
                d2_ptr: DatagramPtr = d2._datagram_ptr(self.geometry)  # type: ignore[union-attr]
                loop.call_soon(
                    lambda *_: future.set_result(
                        Base().controller_send(
                            self._ptr,
                            d1_ptr,
                            d2_ptr,
                            timeout_,
                        ),
                    ),
                )
            case _:
                raise InvalidDatagramTypeError
        res = _validate_int(await future)
        return res == AUTD3_TRUE

    def send(
        self: "Controller",
        d1: SpecialDatagram | Datagram | tuple[Datagram, Datagram],
        d2: Datagram | None = None,
        *,
        timeout: timedelta | None = None,
    ) -> bool:
        """Send data.

        Arguments:
        ---------
            d1: Data to send
            d2: Data to send
            timeout: Timeout

        Returns:
        -------
            bool: If true, it is confirmed that the data has been successfully transmitted.
                  If false, there are no errors, but it is unclear whether the data has been sent reliably or not.

        Raises:
        ------
            AUTDError: If an error occurs
        """
        timeout_ = -1 if timeout is None else int(timeout.total_seconds() * 1000 * 1000 * 1000)
        res: int
        match (d1, d2):
            case (SpecialDatagram(), None):
                ds_ptr = d1._special_datagram_ptr()  # type: ignore[union-attr]
                res = _validate_int(
                    Base().controller_send_special(
                        self._ptr,
                        ds_ptr,
                        timeout_,
                    ),
                )
            case (Datagram(), None):
                d_ptr: DatagramPtr = d1._datagram_ptr(self.geometry)  # type: ignore[union-attr]
                res = _validate_int(
                    Base().controller_send(
                        self._ptr,
                        d_ptr,
                        DatagramPtr(None),
                        timeout_,
                    ),
                )
            case ((Datagram(), Datagram()), None):
                (d11, d12) = d1  # type: ignore[misc]
                d11_ptr: DatagramPtr = d11._datagram_ptr(self.geometry)
                d22_ptr: DatagramPtr = d12._datagram_ptr(self.geometry)
                res = _validate_int(
                    Base().controller_send(
                        self._ptr,
                        d11_ptr,
                        d22_ptr,
                        timeout_,
                    ),
                )
            case (Datagram(), Datagram()):
                d1_ptr: DatagramPtr = d1._datagram_ptr(self.geometry)  # type: ignore[union-attr]
                d2_ptr: DatagramPtr = d2._datagram_ptr(self.geometry)  # type: ignore[union-attr]
                res = _validate_int(
                    Base().controller_send(
                        self._ptr,
                        d1_ptr,
                        d2_ptr,
                        timeout_,
                    ),
                )
            case _:
                raise InvalidDatagramTypeError
        return res == AUTD3_TRUE

    class _GroupGuard(Generic[K]):
        _controller: "Controller"
        _map: Callable[[Device], K | None]
        _kv_map: GroupKVMapPtr
        _keymap: dict[K, int]
        _k: int

        def __init__(self: "Controller._GroupGuard", group_map: Callable[[Device], K | None], controller: "Controller") -> None:
            self._map = group_map
            self._controller = controller
            self._kv_map = Base().controller_group_create_kv_map()
            self._keymap = {}
            self._k = 0

        def set_data(
            self: "Controller._GroupGuard",
            key: K,
            d1: SpecialDatagram | Datagram | tuple[Datagram, Datagram],
            d2: Datagram | None = None,
            *,
            timeout: timedelta | None = None,
        ) -> "Controller._GroupGuard":
            if key in self._keymap:
                raise KeyAlreadyExistsError
            self._keymap[key] = self._k

            timeout_ns = -1 if timeout is None else int(timeout.total_seconds() * 1000 * 1000 * 1000)

            match (d1, d2):
                case (SpecialDatagram(), None):
                    self._kv_map = _validate_ptr(
                        Base().controller_group_kv_map_set_special(
                            self._kv_map,
                            self._k,
                            d1._special_datagram_ptr(),  # type: ignore[union-attr]
                            timeout_ns,
                        ),
                    )
                case (Datagram(), None):
                    self._kv_map = _validate_ptr(
                        Base().controller_group_kv_map_set(
                            self._kv_map,
                            self._k,
                            d1._datagram_ptr(self._controller._geometry),  # type: ignore[union-attr]
                            DatagramPtr(None),
                            timeout_ns,
                        ),
                    )
                case ((Datagram(), Datagram()), None):
                    (d11, d12) = d1  # type: ignore[misc]
                    self._kv_map = _validate_ptr(
                        Base().controller_group_kv_map_set(
                            self._kv_map,
                            self._k,
                            d11._datagram_ptr(self._controller._geometry),  # type: ignore[union-attr]
                            d12._datagram_ptr(self._controller._geometry),  # type: ignore[union-attr]
                            timeout_ns,
                        ),
                    )
                case (Datagram(), Datagram()):
                    self._kv_map = _validate_ptr(
                        Base().controller_group_kv_map_set(
                            self._kv_map,
                            self._k,
                            d1._datagram_ptr(self._controller._geometry),  # type: ignore[union-attr]
                            d2._datagram_ptr(self._controller._geometry),  # type: ignore[union-attr]
                            timeout_ns,
                        ),
                    )
                case _:
                    raise InvalidDatagramTypeError

            self._k += 1

            return self

        async def send_async(self: "Controller._GroupGuard") -> bool:
            m = np.fromiter(
                (self._keymap[k] if k is not None else -1 for k in (self._map(dev) if dev.enable else None for dev in self._controller.geometry)),
                dtype=np.int32,
            )
            future: asyncio.Future = asyncio.Future()
            loop = asyncio.get_event_loop()
            loop.call_soon(
                lambda *_: future.set_result(
                    Base().controller_group(
                        self._controller._ptr,
                        np.ctypeslib.as_ctypes(m.astype(ctypes.c_int32)),
                        self._kv_map,
                    ),
                ),
            )
            return _validate_int(await future) == AUTD3_TRUE

        def send(self: "Controller._GroupGuard") -> bool:
            m = np.fromiter(
                (self._keymap[k] if k is not None else -1 for k in (self._map(dev) if dev.enable else None for dev in self._controller.geometry)),
                dtype=np.int32,
            )
            return (
                _validate_int(
                    Base().controller_group(
                        self._controller._ptr,
                        np.ctypeslib.as_ctypes(m.astype(ctypes.c_int32)),
                        self._kv_map,
                    ),
                )
                == AUTD3_TRUE
            )

    def group(self: "Controller", group_map: Callable[[Device], K | None]) -> "Controller._GroupGuard":
        """Grouping data."""
        return Controller._GroupGuard(group_map, self)


class Clear(Datagram):
    """Datagram for clear all data in devices."""

    def __init__(self: "Clear") -> None:
        super().__init__()

    def _datagram_ptr(self: "Clear", _: Geometry) -> DatagramPtr:
        return Base().datagram_clear()


class Stop(SpecialDatagram):
    """Datagram to stop output."""

    def __init__(self: "Stop") -> None:
        super().__init__()

    def _special_datagram_ptr(self: "Stop") -> DatagramSpecialPtr:
        return Base().datagram_stop()


class Synchronize(Datagram):
    """Datagram to synchronize devices."""

    def __init__(self: "Synchronize") -> None:
        super().__init__()

    def _datagram_ptr(self: "Synchronize", _: Geometry) -> DatagramPtr:
        return Base().datagram_synchronize()


class ConfigureModDelay(Datagram):
    """Datagram to configure modulation delay."""

    def __init__(self: "ConfigureModDelay", f: Callable[[Device, Transducer], int]) -> None:
        super().__init__()

        def f_native(_context: ctypes.c_void_p, geometry_ptr: GeometryPtr, dev_idx: int, tr_idx: int) -> int:
            dev = Device(dev_idx, Base().device(geometry_ptr, dev_idx))
            tr = Transducer(tr_idx, dev._ptr)
            return f(dev, tr)

        self._f_native = ctypes.CFUNCTYPE(ctypes.c_uint16, ctypes.c_void_p, GeometryPtr, ctypes.c_uint32, ctypes.c_uint8)(f_native)

    def _datagram_ptr(self: "ConfigureModDelay", geometry: Geometry) -> DatagramPtr:
        return Base().datagram_configure_mod_delay(self._f_native, None, geometry._ptr)  # type: ignore[arg-type]


class ConfigureDebugOutputIdx(Datagram):
    """Datagram to configure debug output index."""

    def __init__(self: "ConfigureDebugOutputIdx", f: Callable[[Device], Transducer | None]) -> None:
        super().__init__()

        def f_native(_context: ctypes.c_void_p, geometry_ptr: GeometryPtr, dev_idx: int) -> int:
            tr = f(Device(dev_idx, Base().device(geometry_ptr, dev_idx)))
            return tr.idx if tr is not None else 0xFF

        self._f_native = ctypes.CFUNCTYPE(ctypes.c_uint8, ctypes.c_void_p, GeometryPtr, ctypes.c_uint32)(f_native)

    def _datagram_ptr(self: "ConfigureDebugOutputIdx", geometry: Geometry) -> DatagramPtr:
        return Base().datagram_configure_debug_output_idx(self._f_native, None, geometry._ptr)  # type: ignore[arg-type]


class ConfigureForceFan(Datagram):
    """Datagram to configure force fan."""

    def __init__(self: "ConfigureForceFan", f: Callable[[Device], bool]) -> None:
        super().__init__()

        def f_native(_context: ctypes.c_void_p, geometry_ptr: GeometryPtr, dev_idx: int) -> bool:
            return f(Device(dev_idx, Base().device(geometry_ptr, dev_idx)))

        self._f_native = ctypes.CFUNCTYPE(ctypes.c_bool, ctypes.c_void_p, GeometryPtr, ctypes.c_uint32)(f_native)

    def _datagram_ptr(self: "ConfigureForceFan", geometry: Geometry) -> DatagramPtr:
        return Base().datagram_configure_force_fan(self._f_native, None, geometry._ptr)  # type: ignore[arg-type]


class ConfigureReadsFPGAInfo(Datagram):
    """Datagram to configure reads FPGA info."""

    def __init__(self: "ConfigureReadsFPGAInfo", f: Callable[[Device], bool]) -> None:
        super().__init__()

        def f_native(_context: ctypes.c_void_p, geometry_ptr: GeometryPtr, dev_idx: int) -> bool:
            return f(Device(dev_idx, Base().device(geometry_ptr, dev_idx)))

        self._f_native = ctypes.CFUNCTYPE(ctypes.c_bool, ctypes.c_void_p, GeometryPtr, ctypes.c_uint32)(f_native)

    def _datagram_ptr(self: "ConfigureReadsFPGAInfo", geometry: Geometry) -> DatagramPtr:
        return Base().datagram_configure_reads_fpga_info(self._f_native, None, geometry._ptr)  # type: ignore[arg-type]
