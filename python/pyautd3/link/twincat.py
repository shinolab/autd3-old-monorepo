'''
File: twincat.py
Project: link
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 29/04/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''

from datetime import timedelta
from ctypes import c_void_p, byref

from .link import Link

from pyautd3.native_methods.autd3capi_link_twincat import NativeMethods as LinkTwinCAT
from pyautd3.log_level import LogLevel


class TwinCAT:
    def __init__(self):
        self._builder = c_void_p()
        LinkTwinCAT().dll.AUTDLinkTwinCAT(byref(self._builder))

    def log_level(self, level: LogLevel):
        LinkTwinCAT().dll.AUTDLinkTwinCATLogLevel(self._builder, int(level))
        return self

    def log_func(self, log_out, log_flush):
        LinkTwinCAT().dll.AUTDLinkTwinCATLogFunc(self._builder, log_out, log_flush)
        return self

    def timeout(self, timeout: timedelta):
        LinkTwinCAT().dll.AUTDLinkTwinCATTimeout(self._builder, int(timeout.total_seconds() * 1000 * 1000 * 1000))
        return self

    def build(self):
        link = c_void_p()
        LinkTwinCAT().dll.AUTDLinkTwinCATBuild(byref(link), self._builder)
        return Link(link)
