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

from ctypes import c_void_p
from datetime import timedelta

from .link import Link

from pyautd3.native_methods.autd3capi import NativeMethods as LinkDebug
from pyautd3.native_methods.autd3capi import Level


class Debug:
    _builder: c_void_p

    def __init__(self):
        self._builder = LinkDebug().link_debug()

    def log_level(self, level: Level) -> "Debug":
        self._builder = LinkDebug().link_debug_log_level(self._builder, level)
        return self

    def log_func(self, level: Level, log_out, log_flush) -> "Debug":
        self._builder = LinkDebug().link_debug_log_func(
            self._builder, level, log_out, log_flush
        )
        return self

    def timeout(self, timeout: timedelta) -> "Debug":
        self._builder = LinkDebug().link_debug_timeout(
            self._builder, int(timeout.total_seconds() * 1000 * 1000 * 1000)
        )
        return self

    def build(self) -> Link:
        link = LinkDebug().link_debug_build(self._builder)
        return Link(link)
