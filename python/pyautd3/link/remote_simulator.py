'''
File: remote_simulator.py
Project: link
Created Date: 01/05/2023
Author: Shun Suzuki
-----
Last Modified: 01/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from ctypes import c_void_p, byref
from datetime import timedelta

from .link import Link

from pyautd3.native_methods.autd3capi_link_remote_simulator import NativeMethods as LinkRemoteSimulator
from pyautd3.log_level import LogLevel


class RemoteSimulator:
    def __init__(self, ip, port):
        self._builder = c_void_p()
        LinkRemoteSimulator().dll.AUTDLinkRemoteSimulator(byref(self._builder), ip.encode('utf-8'), port)

    def log_level(self, level: LogLevel):
        LinkRemoteSimulator().dll.AUTDLinkRemoteSimulatorLogLevel(self._builder, int(level))
        return self

    def log_func(self, log_out, log_flush):
        LinkRemoteSimulator().dll.AUTDLinkRemoteSimulatorLogFunc(self._builder, log_out, log_flush)
        return self

    def timeout(self, timeout: timedelta):
        LinkRemoteSimulator().dll.AUTDLinkRemoteSimulatorTimeout(self._builder, int(timeout.total_seconds() * 1000 * 1000 * 1000))
        return self

    def build(self):
        link = c_void_p()
        LinkRemoteSimulator().dll.AUTDLinkRemoteSimulatorBuild(byref(link), self._builder)
        return Link(link)
