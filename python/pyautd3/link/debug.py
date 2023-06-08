"""
File: debug.py
Project: link
Created Date: 17/04/2023
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""

from datetime import timedelta

from .link import Link

from pyautd3.native_methods.autd3capi import NativeMethods as LinkDebug
from pyautd3.native_methods.autd3capi_def import Level


class Debug(Link):
    def __init__(self):
        super().__init__(LinkDebug().link_debug())

    def with_log_level(self, level: Level) -> "Debug":
        self._ptr = LinkDebug().link_debug_with_log_level(self._ptr, level)
        return self

    def with_log_func(self, log_out, log_flush) -> "Debug":
        self._ptr = LinkDebug().link_debug_with_log_func(self._ptr, log_out, log_flush)
        return self

    def with_timeout(self, timeout: timedelta) -> "Debug":
        self._ptr = LinkDebug().link_debug_with_timeout(
            self._ptr, int(timeout.total_seconds() * 1000 * 1000 * 1000)
        )
        return self
