"""
File: simulator.py
Project: link
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 25/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""


import asyncio
import ctypes
from datetime import timedelta

from pyautd3.autd_error import AUTDError
from pyautd3.geometry import Geometry
from pyautd3.internal.link import Link, LinkBuilder
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import AUTD3_ERR, ControllerPtr, LinkPtr, Resulti32Wrapper, RuntimePtr
from pyautd3.native_methods.autd3capi_def import NativeMethods as Def
from pyautd3.native_methods.autd3capi_link_simulator import (
    LinkBuilderPtr,
    LinkSimulatorBuilderPtr,
)
from pyautd3.native_methods.autd3capi_link_simulator import (
    NativeMethods as LinkSimulator,
)


class Simulator(Link):
    """Link for Simulator."""

    _ptr: LinkPtr
    _runtime: RuntimePtr

    class _Builder(LinkBuilder):
        _builder: LinkSimulatorBuilderPtr

        def __init__(self: "Simulator._Builder", port: int) -> None:
            self._builder = LinkSimulator().link_simulator(port)

        def with_server_ip(self: "Simulator._Builder", addr: str) -> "Simulator._Builder":
            """Set server IP address.

            Arguments:
            ---------
                addr: Server IP address
            """
            err = ctypes.create_string_buffer(256)
            self._builder = LinkSimulator().link_simulator_with_addr(self._builder, addr.encode("utf-8"), err)
            if self._builder._0 is None:
                raise AUTDError(err)
            return self

        def with_timeout(self: "Simulator._Builder", timeout: timedelta) -> "Simulator._Builder":
            """Set timeout.

            Arguments:
            ---------
                timeout: Timeout
            """
            self._builder = LinkSimulator().link_simulator_with_timeout(self._builder, int(timeout.total_seconds() * 1000 * 1000 * 1000))
            return self

        def _link_builder_ptr(self: "Simulator._Builder") -> LinkBuilderPtr:
            return LinkSimulator().link_simulator_into_builder(self._builder)

        def _resolve_link(self: "Simulator._Builder", ptr: ControllerPtr, runtime: RuntimePtr) -> "Simulator":
            return Simulator(Base().link_get(ptr), runtime)

    def __init__(self: "Simulator", ptr: LinkPtr, runtime: RuntimePtr) -> None:
        super().__init__(ptr)
        self._runtime = runtime

    @staticmethod
    def builder(port: int) -> _Builder:
        """Create Simulator link builder."""
        return Simulator._Builder(port)

    async def update_geometry(self: "Simulator", geometry: Geometry) -> None:
        """Update geometry."""
        future: asyncio.Future = asyncio.Future()
        ffi_future = LinkSimulator().link_simulator_update_geometry_async(self._ptr, geometry._geometry_ptr())
        loop = asyncio.get_event_loop()
        loop.call_soon(
            lambda *_: future.set_result(Def().await_resulti_32(self._runtime, ffi_future)),
        )
        res: Resulti32Wrapper = await future
        if res.result == AUTD3_ERR:
            err = ctypes.create_string_buffer(256)
            raise AUTDError(err)
