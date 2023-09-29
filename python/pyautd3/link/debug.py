'''
File: debug.py
Project: link
Created Date: 17/04/2023
Author: Shun Suzuki
-----
Last Modified: 29/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''

from datetime import timedelta

from .link import Link

from pyautd3.native_methods.autd3capi import NativeMethods as LinkDebug
from pyautd3.native_methods.autd3capi_def import Level


class Debug(Link):
    """Link to debug

    """

    def __init__(self):
        super().__init__(LinkDebug().link_debug())

    def with_log_level(self, level: Level) -> "Debug":
        """Set log level

        Arguments:
        - `level` - Log level
        """

        self._ptr = LinkDebug().link_debug_with_log_level(self._ptr, level)
        return self

    def with_log_func(self, log_out, log_flush) -> "Debug":
        """Set log function

        By default, this link will display log messages on the console.

        Arguments:
        - `log_out` - Log output function
        - `log_flush` - Log flush function
        """

        self._ptr = LinkDebug().link_debug_with_log_func(self._ptr, log_out, log_flush)
        return self

    def with_timeout(self, timeout: timedelta) -> "Debug":
        """Set timeout

        Arguments:
        - `timeout` - Timeout
        """

        self._ptr = LinkDebug().link_debug_with_timeout(
            self._ptr, int(timeout.total_seconds() * 1000 * 1000 * 1000)
        )
        return self
