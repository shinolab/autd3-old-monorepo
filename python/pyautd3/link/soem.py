'''
File: soem.py
Project: link
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 21/10/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''

import ctypes
from ctypes import c_void_p, byref
from .link import Link

from pyautd3.native_methods.autd3capi_link_soem import NativeMethods as LinkSOEM


ErrorHandlerFunc = ctypes.CFUNCTYPE(None, ctypes.c_char_p)


class SOEM:
    def __init__(self):
        self._ifname = None
        self._send_cycle = 1
        self._sync0_cycle = 1
        self._on_lost = None
        self._high_precision = False
        self._freerun = False

    def ifname(self, ifname: str):
        self._ifname = ifname
        return self

    def send_cycle(self, cycle: int):
        self._send_cycle = cycle
        return self

    def sync0_cycle(self, cycle: int):
        self._sync0_cycle = cycle
        return self

    def on_lost(self, handle):
        self._on_lost = handle
        return self

    def high_precision(self, flag: bool):
        self._high_precision = flag
        return self

    def freerun(self, flag: bool):
        self._freerun = flag
        return self

    def build(self):
        onlost = ErrorHandlerFunc(self._on_lost) if self._on_lost is not None else None
        LinkSOEM().init_dll()
        link = c_void_p()
        LinkSOEM().dll.AUTDLinkSOEM(byref(link), self._ifname.encode('utf-8') if self._ifname is not None else None,
                                    self._sync0_cycle, self._send_cycle, self._freerun, onlost, self._high_precision)
        return Link(link)

    @ staticmethod
    def enumerate_adapters():
        LinkSOEM().init_dll()
        res = []
        handle = c_void_p()
        size = LinkSOEM().dll.AUTDGetAdapterPointer(byref(handle))

        for i in range(size):
            sb_desc = ctypes.create_string_buffer(128)
            sb_name = ctypes.create_string_buffer(128)
            LinkSOEM().dll.AUTDGetAdapter(handle, i, sb_desc, sb_name)
            res.append([sb_name.value.decode('utf-8'), sb_desc.value.decode('utf-8')])

        LinkSOEM().dll.AUTDFreeAdapterPointer(handle)

        return res
