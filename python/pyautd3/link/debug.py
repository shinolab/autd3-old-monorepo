'''
File: debug.py
Project: link
Created Date: 17/04/2023
Author: Shun Suzuki
-----
Last Modified: 29/04/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''

from datetime import timedelta

from ctypes import c_void_p, byref
from .link import Link

from pyautd3.native_methods.autd3capi import NativeMethods as LinkDebug
from pyautd3.log_level import LogLevel


class Debug:
    def __init__(self):
        self._builder = c_void_p()
        LinkDebug().dll.AUTDLinkDebug(byref(self._builder))

    def log_level(self, level: LogLevel):
        LinkDebug().dll.AUTDLinkDebugLogLevel(self._builder, int(level))
        return self

    def log_func(self, log_out, log_flush):
        LinkDebug().dll.AUTDLinkDebugLogFunc(self._builder, log_out, log_flush)
        return self

    def timeout(self, timeout: timedelta):
        LinkDebug().dll.AUTDLinkDebugTimeout(self._builder, int(timeout.total_seconds() * 1000 * 1000 * 1000))
        return self

    def build(self):
        link = c_void_p()
        LinkDebug().dll.AUTDLinkDebugBuild(byref(link), self._builder)
        return Link(link)
