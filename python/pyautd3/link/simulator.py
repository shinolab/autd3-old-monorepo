'''
File: simulator.py
Project: link
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 17/04/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''


from ctypes import c_void_p, byref
from datetime import timedelta

from .link import Link

from pyautd3.native_methods.autd3capi_link_simulator import NativeMethods as LinkSimulator


class Simulator:
    def __init__(self):
        self._timeout = 20 * 1000 * 1000
 
    def timeout(self, timeout: timedelta):
        self._timeout = int(timeout.total_seconds() * 1000 * 1000 * 1000)
        return self
 
    def build(self):
        link = c_void_p()
        LinkSimulator().dll.AUTDLinkSimulator(byref(link), self._timeout)
        return Link(link)
