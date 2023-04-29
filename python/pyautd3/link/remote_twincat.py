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

from datetime import timedelta
from ctypes import c_void_p, byref

from .link import Link

from pyautd3.native_methods.autd3capi_link_remote_twincat import NativeMethods as LinkRemoteTwinCAT
from pyautd3.debug_level import DebugLevel


class RemoteTwinCAT:
    def __init__(self, server_ams_net_id):
        self._builder = c_void_p()
        LinkRemoteTwinCAT().dll.AUTDLinkRemoteTwinCAT(byref(self._builder), server_ams_net_id.encode('utf-8'))

    def server_ip(self, ip):
        LinkRemoteTwinCAT().dll.AUTDLinkRemoteTwinCATServerIpAddr(self._builder, ip.encode('utf-8'))
        return self

    def client_ams_net_id(self, id):
        LinkRemoteTwinCAT().dll.AUTDLinkRemoteTwinCATClientAmsNetId(self._builder, id.encode('utf-8'))
        return self

    def log_level(self, level: DebugLevel):
        LinkRemoteTwinCAT().dll.AUTDLinkRemoteTwinCAT(self._builder, int(level))
        return self

    def log_func(self, log_out, log_flush):
        LinkRemoteTwinCAT().dll.AUTDLinkRemoteTwinCAT(self._builder, log_out, log_flush)
        return self

    def timeout(self, timeout: timedelta):
        LinkRemoteTwinCAT().dll.AUTDLinkRemoteTwinCAT(self._builder, int(timeout.total_seconds() * 1000 * 1000 * 1000))
        return self

    def build(self):
        link = c_void_p()
        LinkRemoteTwinCAT().dll.AUTDLinkRemoteTwinCAT(byref(link), self._builder)
        return Link(link)
