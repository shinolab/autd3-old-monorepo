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
from datetime import timedelta

from pyautd3.geometry import Geometry
from pyautd3.internal.link import Link, LinkBuilder
from pyautd3.internal.utils import _validate_int, _validate_ptr
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import ControllerPtr, LinkPtr
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
            self._builder = _validate_ptr(LinkSimulator().link_simulator_with_addr(self._builder, addr.encode("utf-8")))
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
            return LinkSimulator().link_simulator_into_builder(self._builder)  # pragma: no cover

        def _resolve_link(self: "Simulator._Builder", ptr: ControllerPtr) -> "Simulator":
            return Simulator(Base().link_get(ptr))

    def __init__(self: "Simulator", ptr: LinkPtr) -> None:
        super().__init__(ptr)  # pragma: no cover

    @staticmethod
    def builder(port: int) -> _Builder:
        """Create Simulator link builder."""
        return Simulator._Builder(port)

    async def update_geometry_async(self: "Simulator", geometry: Geometry) -> None:
        """Update geometry."""
        future: asyncio.Future = asyncio.Future()
        loop = asyncio.get_event_loop()
        loop.call_soon(
            lambda *_: future.set_result(
                LinkSimulator().link_simulator_update_geometry(
                    self._ptr,
                    geometry._geometry_ptr(),
                ),
            ),
        )
        _validate_int(await future)

    def update_geometry(self: "Simulator", geometry: Geometry) -> None:
        """Update geometry."""
        _validate_int(
            LinkSimulator().link_simulator_update_geometry(
                self._ptr,
                geometry._geometry_ptr(),
            ),
        )
