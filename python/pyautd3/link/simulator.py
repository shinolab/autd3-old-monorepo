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


from datetime import timedelta
import ctypes

from .link import Link

from pyautd3.native_methods.autd3capi_link_simulator import (
    NativeMethods as LinkSimulator,
)
from pyautd3.autd_error import AUTDError


class Simulator(Link):
    def __init__(self, port: int):
        super().__init__(LinkSimulator().link_simulator(port))

    def with_server_ip(self, addr: str) -> "Simulator":
        err = ctypes.create_string_buffer(256)
        self._ptr = LinkSimulator().link_simulator_addr(
            self._ptr, addr.encode("utf-8"), err
        )
        if self._ptr._0 is None:
            raise AUTDError(err)
        return self

    def with_timeout(self, timeout: timedelta) -> "Simulator":
        self._ptr = LinkSimulator().link_simulator_timeout(
            self._ptr, int(timeout.total_seconds() * 1000 * 1000 * 1000)
        )
        return self
