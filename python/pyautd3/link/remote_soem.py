'''
File: remote_twincat.py
Project: link
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 28/04/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''


from ctypes import c_void_p, byref
from datetime import timedelta

from .link import Link

from pyautd3.native_methods.autd3capi_link_remote_soem import NativeMethods as LinkRemoteSOEM
from pyautd3.debug_level import DebugLevel


class RemoteSOEM:
    def __init__(self, ip, port):
        self._builder = c_void_p()
        LinkRemoteSOEM().dll.AUTDLinkRemoteSOEM(byref(self._builder), ip.encode('utf-8'), port)

    def log_level(self, level: DebugLevel):
        LinkRemoteSOEM().dll.AUTDLinkRemoteSOEMLogLevel(self._builder, int(level))
        return self

    def log_func(self, log_out, log_flush):
        LinkRemoteSOEM().dll.AUTDLinkRemoteSOEMLogFunc(self._builder, log_out, log_flush)
        return self

    def timeout(self, timeout: timedelta):
        LinkRemoteSOEM().dll.AUTDLinkRemoteSOEMTimeout(self._builder, int(timeout.total_seconds() * 1000 * 1000 * 1000))
        return self

    def build(self):
        link = c_void_p()
        LinkRemoteSOEM().dll.AUTDLinkRemoteSOEMBuild(byref(link), self._builder)
        return Link(link)
