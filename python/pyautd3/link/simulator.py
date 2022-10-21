'''
File: simulator.py
Project: link
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 21/10/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''


from ctypes import c_void_p, byref

from .link import Link

from pyautd3.native_methods.autd3capi_link_simulator import NativeMethods as LinkSimulator


class Simulator:
    def __init__(self):
        self._port = 50632
        self._ip_addr = '127.0.0.1'

    def port(self, port: int):
        self._port = port
        return self

    def ip_addr(self, addr: str):
        self._ip_addr = addr
        return self

    def build(self):
        link = c_void_p()
        LinkSimulator().dll.AUTDLinkSimulator(byref(link), self._port, self._ip_addr.encode('utf-8'))
        return Link(link)
