'''
File: remote_twincat.py
Project: link
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 04/11/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''


from ctypes import c_void_p, byref

from .link import Link

from pyautd3.native_methods.autd3capi_link_remote_soem import NativeMethods as LinkRemoteSOEM


class RemoteSOEM:
    def __init__(self):
        self._ip = ""
        self._port = 50632

    def ip(self, ip):
        self._ip = ip
        return self

    def port(self, port):
        self._port = port
        return self

    def build(self):
        link = c_void_p()
        LinkRemoteSOEM().dll.AUTDLinkRemoteSOEM(byref(link), self._ip.encode('utf-8'), self._port)
        return Link(link)
