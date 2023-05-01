'''
File: simulator.py
Project: link
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 29/04/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''


from ctypes import c_void_p, byref
from datetime import timedelta

from .link import Link

from pyautd3.native_methods.autd3capi_link_simulator import NativeMethods as LinkSimulator
from pyautd3.log_level import LogLevel


class Simulator:
    def __init__(self):
        self._builder = c_void_p()
        LinkSimulator().dll.AUTDLinkSimulator(byref(self._builder))

    def log_level(self, level: LogLevel):
        LinkSimulator().dll.AUTDLinkSimulatorLogLevel(self._builder, int(level))
        return self

    def log_func(self, log_out, log_flush):
        LinkSimulator().dll.AUTDLinkSimulatorLogFunc(self._builder, log_out, log_flush)
        return self

    def timeout(self, timeout: timedelta):
        LinkSimulator().dll.AUTDLinkSimulatorTimeout(self._builder, int(timeout.total_seconds() * 1000 * 1000 * 1000))
        return self

    def build(self):
        link = c_void_p()
        LinkSimulator().dll.AUTDLinkSimulatorBuild(byref(link), self._builder)
        return Link(link)
