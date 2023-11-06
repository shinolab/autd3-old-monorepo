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


import ctypes
from collections.abc import Callable
from datetime import timedelta
from typing import Generic, TypeVar

import numpy as np

from .autd_error import AUTDError, InvalidDatagramTypeError, KeyAlreadyExistsError
from .geometry import AUTD3, Device, Geometry
from .internal.datagram import Datagram, SpecialDatagram
from .internal.link import Link, LinkBuilder
from .native_methods.autd3capi import ControllerBuilderPtr
from .native_methods.autd3capi import NativeMethods as Base
from .native_methods.autd3capi_def import (
    AUTD3_ERR,
    AUTD3_FALSE,
    AUTD3_TRUE,
    ControllerPtr,
    DatagramPtr,
    DatagramSpecialPtr,
    GroupKVMapPtr,
)

K = TypeVar("K")
L = TypeVar("L", bound=Link)


LogOutputFunc = ctypes.CFUNCTYPE(None, ctypes.c_char_p)
LogFlushFunc = ctypes.CFUNCTYPE(None)


class Silencer(Datagram):
    """Datagram for configure silencer."""

    _step: int

    def __init__(self: "Silencer", step: int = 10) -> None:
        """Constructor.

        Arguments:
        ---------
            step: The update step of silencer. The lower the value, the stronger the silencer effect.
        """
        super().__init__()
        self._step = step

    @staticmethod
    def disable() -> "Silencer":
        """Disable silencer."""
        return Silencer(0xFFFF)

    def _datagram_ptr(self: "Silencer", _: Geometry) -> DatagramPtr:
        return Base().datagram_silencer(self._step)


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

    def __repr__(self: "FirmwareInfo") -> str:
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
    def _open_impl(builder: ControllerBuilderPtr, link_builder: LinkBuilder[L]) -> "Controller[L]":
        err = ctypes.create_string_buffer(256)
        ptr = Base().controller_open_with(builder, link_builder._link_builder_ptr(), err)
        if ptr._0 is None:
            raise AUTDError(err)
        geometry = Geometry(Base().geometry(ptr))
        link = link_builder._resolve_link(ptr)
        return Controller(geometry, ptr, link)

    def firmware_info_list(self: "Controller") -> list[FirmwareInfo]:
        """Get firmware information list."""
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

    def close(self: "Controller") -> None:
        """Close controller."""
        err = ctypes.create_string_buffer(256)
        if not Base().controller_close(self._ptr, err):
            raise AUTDError(err)

    @property
    def fpga_info(self: "Controller") -> list[FPGAInfo]:
        """Get FPGA information list."""
        infos = np.zeros([self.geometry.num_devices]).astype(ctypes.c_uint8)
        pinfos = np.ctypeslib.as_ctypes(infos)
        err = ctypes.create_string_buffer(256)
        if not Base().controller_fpga_info(self._ptr, pinfos, err):
            raise AUTDError(err)
        return [FPGAInfo(x) for x in infos]

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
        err = ctypes.create_string_buffer(256)
        res: ctypes.c_int32 = ctypes.c_int32(AUTD3_FALSE)
        match (d1, d2):
            case (SpecialDatagram(), None):
                res = Base().controller_send_special(self._ptr, d1._special_datagram_ptr(), timeout_, err)  # type: ignore[union-attr]
            case (Datagram(), None):
                res = Base().controller_send(
                    self._ptr,
                    d1._datagram_ptr(self.geometry),  # type: ignore[union-attr]
                    DatagramPtr(None),
                    timeout_,
                    err,
                )
            case ((Datagram(), Datagram()), None):
                (d11, d12) = d1  # type: ignore[misc]
                res = Base().controller_send(
                    self._ptr,
                    d11._datagram_ptr(self.geometry),  # type: ignore[union-attr]
                    d12._datagram_ptr(self.geometry),  # type: ignore[union-attr]
                    timeout_,
                    err,
                )
            case (Datagram(), Datagram()):
                res = Base().controller_send(
                    self._ptr,
                    d1._datagram_ptr(self.geometry),  # type: ignore[union-attr]
                    d2._datagram_ptr(self.geometry),  # type: ignore[union-attr]
                    timeout_,
                    err,
                )
            case _:
                raise InvalidDatagramTypeError

        if res == AUTD3_ERR:
            raise AUTDError(err)

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

            err = ctypes.create_string_buffer(256)
            match (d1, d2):
                case (SpecialDatagram(), None):
                    self._kv_map = Base().controller_group_kv_map_set_special(
                        self._kv_map,
                        self._k,
                        d1._special_datagram_ptr(),  # type: ignore[union-attr]
                        timeout_ns,
                        err,
                    )
                case (Datagram(), None):
                    self._kv_map = Base().controller_group_kv_map_set(
                        self._kv_map,
                        self._k,
                        d1._datagram_ptr(self._controller._geometry),  # type: ignore[union-attr]
                        DatagramPtr(None),
                        timeout_ns,
                        err,
                    )
                case ((Datagram(), Datagram()), None):
                    (d11, d12) = d1  # type: ignore[misc]
                    self._kv_map = Base().controller_group_kv_map_set(
                        self._kv_map,
                        self._k,
                        d11._datagram_ptr(self._controller._geometry),  # type: ignore[union-attr]
                        d12._datagram_ptr(self._controller._geometry),  # type: ignore[union-attr]
                        timeout_ns,
                        err,
                    )
                case (Datagram(), Datagram()):
                    self._kv_map = Base().controller_group_kv_map_set(
                        self._kv_map,
                        self._k,
                        d1._datagram_ptr(self._controller._geometry),  # type: ignore[union-attr]
                        d2._datagram_ptr(self._controller._geometry),  # type: ignore[union-attr]
                        timeout_ns,
                        err,
                    )
                case _:
                    raise InvalidDatagramTypeError

            if self._kv_map._0 is None:
                raise AUTDError(err)

            self._k += 1

            return self

        def send(self: "Controller._GroupGuard") -> bool:
            m = np.fromiter(
                (self._keymap[k] if k is not None else -1 for k in (self._map(dev) if dev.enable else None for dev in self._controller.geometry)),
                dtype=np.int32,
            )
            err = ctypes.create_string_buffer(256)
            res = Base().controller_group(self._controller._ptr, np.ctypeslib.as_ctypes(m.astype(ctypes.c_int32)), self._kv_map, err)
            if res == AUTD3_ERR:
                raise AUTDError(err)
            return res == AUTD3_TRUE

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


class UpdateFlags(Datagram):
    """Datagram to update flags (Force fan flag and reads FPGA info flag)."""

    def __init__(self: "UpdateFlags") -> None:
        super().__init__()

    def _datagram_ptr(self: "UpdateFlags", _: Geometry) -> DatagramPtr:
        return Base().datagram_update_flags()


class Synchronize(Datagram):
    """Datagram to synchronize devices."""

    def __init__(self: "Synchronize") -> None:
        super().__init__()

    def _datagram_ptr(self: "Synchronize", _: Geometry) -> DatagramPtr:
        return Base().datagram_synchronize()


class ConfigureModDelay(Datagram):
    """Datagram to configure modulation delay."""

    def __init__(self: "ConfigureModDelay") -> None:
        super().__init__()

    def _datagram_ptr(self: "ConfigureModDelay", _: Geometry) -> DatagramPtr:
        return Base().datagram_configure_mod_delay()


class ConfigureAmpFilter(Datagram):
    """Datagram to configure amplitude filter."""

    def __init__(self: "ConfigureAmpFilter") -> None:
        super().__init__()

    def _datagram_ptr(self: "ConfigureAmpFilter", _: Geometry) -> DatagramPtr:
        return Base().datagram_configure_amp_filter()


class ConfigurePhaseFilter(Datagram):
    """Datagram to configure phase filter."""

    def __init__(self: "ConfigurePhaseFilter") -> None:
        super().__init__()

    def _datagram_ptr(self: "ConfigurePhaseFilter", _: Geometry) -> DatagramPtr:
        return Base().datagram_configure_phase_filter()
