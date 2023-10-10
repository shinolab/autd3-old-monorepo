'''
File: simulator.py
Project: link
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

'''


from datetime import timedelta
import ctypes

from pyautd3.native_methods.autd3capi_link_simulator import (
    LinkSimulatorBuilderPtr,
    NativeMethods as LinkSimulator,
)
from pyautd3.native_methods.autd3capi_link_simulator import LinkBuilderPtr
from pyautd3.autd_error import AUTDError
from pyautd3.internal.link import LinkBuilder


class Simulator:
    """Link for Simulator

    """

    class _Builder(LinkBuilder):
        _builder: LinkSimulatorBuilderPtr

        def __init__(self, port: int):
            self._builder = LinkSimulator().link_simulator(port)

        def with_server_ip(self, addr: str) -> "Simulator._Builder":
            """Set server IP address

            Arguments:
            - `addr` - Server IP address
            """

            err = ctypes.create_string_buffer(256)
            self._builder = LinkSimulator().link_simulator_with_addr(
                self._builder, addr.encode("utf-8"), err
            )
            if self._builder._0 is None:
                raise AUTDError(err)
            return self

        def with_timeout(self, timeout: timedelta) -> "Simulator._Builder":
            """Set timeout

            Arguments:
            - `timeout` - Timeout
            """

            self._builder = LinkSimulator().link_simulator_with_timeout(
                self._builder, int(timeout.total_seconds() * 1000 * 1000 * 1000)
            )
            return self

        def _ptr(self) -> LinkBuilderPtr:
            return LinkSimulator().link_simulator_into_builder(self._builder)

    @staticmethod
    def builder(port: int) -> _Builder:
        return Simulator._Builder(port)
