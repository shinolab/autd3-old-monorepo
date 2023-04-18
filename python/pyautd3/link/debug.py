'''
File: debug.py
Project: link
Created Date: 17/04/2023
Author: Shun Suzuki
-----
Last Modified: 17/04/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from ctypes import c_void_p, byref
from .link import Link

from pyautd3.native_methods.autd3capi import NativeMethods as LinkDebug
from pyautd3.debug_level import DebugLevel


class Debug:
    def __init__(self):
        self._level = DebugLevel.Debug
        self._output = None
        self._flush = None

    def level(self, level: DebugLevel):
        self._level = level
        return self

    def log_func(self, log_out, log_flush):
        self._output = log_out
        self._flush = log_flush
        return self

    def build(self):
        link = c_void_p()
        LinkDebug().dll.AUTDLinkDebug(byref(link), int(self._level), self._output, self._flush)
        return Link(link)
