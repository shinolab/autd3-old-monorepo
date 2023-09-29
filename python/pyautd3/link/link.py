'''
File: link.py
Project: link
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 29/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

'''

import ctypes

from pyautd3.native_methods.autd3capi import NativeMethods as LinkLog
from pyautd3.native_methods.autd3capi_def import LinkPtr, Level

LogOutputFunc = ctypes.CFUNCTYPE(None, ctypes.c_char_p)
LogFlushFunc = ctypes.CFUNCTYPE(None)


class Link:
    _ptr: LinkPtr

    def __init__(self, ptr: LinkPtr):
        self._ptr = ptr

    def ptr(self) -> LinkPtr:
        return self._ptr

    def with_log(self):
        return Log(self)


class Log(Link):
    """Link to logging

    """

    def __init__(self, link: Link):
        super().__init__(LinkLog().link_log(link.ptr()))

    def with_log_level(self, level: Level) -> "Log":
        """Set log level

        Arguments:
        - `level` - Log level
        """

        self._ptr = LinkLog().link_log_with_log_level(self._ptr, level)
        return self

    def with_log_func(self, log_out, log_flush) -> "Log":
        """Set log function

        By default, this link will display log messages on the console.

        Arguments:
        - `log_out` - Log output function
        - `log_flush` - Log flush function
        """

        self._ptr = LinkLog().link_log_with_log_func(self._ptr, log_out, log_flush)
        return self
