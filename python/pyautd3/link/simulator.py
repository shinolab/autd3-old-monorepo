"""
File: simulator.py
Project: link
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""


from ctypes import c_void_p
from datetime import timedelta

from .link import Link

from pyautd3.native_methods.autd3capi_link_simulator import (
    NativeMethods as LinkSimulator,
)


class Simulator:
    _builder: c_void_p

    def __init__(self, port: int):
        self._builder = LinkSimulator().link_simulator(port)

    def addr(self, addr: str) -> "Simulator":
        self._builder = LinkSimulator().link_simulator_addr(
            self._builder, addr.encode("utf-8")
        )
        return self

    def timeout(self, timeout: timedelta) -> "Simulator":
        self._builder = LinkSimulator().link_simulator_timeout(
            self._builder, int(timeout.total_seconds() * 1000 * 1000 * 1000)
        )
        return self

    def build(self) -> Link:
        link = LinkSimulator().link_simulator_build(self._builder)
        return Link(link)
