'''
File: soem.py
Project: link
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 17/04/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''

from datetime import timedelta
import ctypes
from ctypes import c_void_p, byref
from .link import Link

from pyautd3.native_methods.autd3capi_link_soem import NativeMethods as LinkSOEM
from pyautd3.debug_level import DebugLevel
from pyautd3.sync_mode import SyncMode
from pyautd3.timer_strategy import TimerStrategy


OnLostFunc = ctypes.CFUNCTYPE(None, ctypes.c_char_p)
LogOutputFunc = ctypes.CFUNCTYPE(None, ctypes.c_char_p)
LogFlushFunc = ctypes.CFUNCTYPE(None)


class SOEM:
    def __init__(self):
        LinkSOEM().init_dll()
        soem = c_void_p()
        LinkSOEM().dll.AUTDLinkSOEM(byref(soem))
        self._soem = soem

    def ifname(self, ifname: str):
        LinkSOEM().dll.AUTDLinkSOEMIfname(self._soem, ifname.encode('utf-8'))
        return self

    def buf_size(self, size: int):
        LinkSOEM().dll.AUTDLinkSOEMBufSize(self._soem, size)
        return self

    def send_cycle(self, cycle: int):
        LinkSOEM().dll.AUTDLinkSOEMSendCycle(self._soem, cycle)
        return self

    def sync0_cycle(self, cycle: int):
        LinkSOEM().dll.AUTDLinkSOEMSync0Cycle(self._soem, cycle)
        return self

    def on_lost(self, handle):
        LinkSOEM().dll.AUTDLinkSOEMOnLost(self._soem, handle)
        return self

    def timer_strategy(self, strategy: TimerStrategy):
        LinkSOEM().dll.AUTDLinkSOEMTimerStrategy(self._soem, int(strategy))
        return self

    def sync_mode(self, mode: SyncMode):
        LinkSOEM().dll.AUTDLinkSOEMFreerun(self._soem, mode == SyncMode.FreeRun)
        return self

    def state_check_interval(self, interval: timedelta):
        LinkSOEM().dll.AUTDLinkSOEMStateCheckInterval(self._soem, int(interval.total_seconds() / 1000))
        return self

    def debug_level(self, level: DebugLevel):
        LinkSOEM().dll.AUTDLinkSOEMLogLevel(self._soem, int(level))
        return self

    def debug_log_func(self, log_out, log_flush):
        LinkSOEM().dll.AUTDLinkSOEMLogFunc(self._soem, log_out, log_flush)
        return self

    def timeout(self, timeout: timedelta):
        LinkSOEM().dll.AUTDLinkSOEMTimeout(self._soem, int(timeout.total_seconds() * 1000 * 1000 * 1000))
        return self

    def build(self):
        link = c_void_p()
        LinkSOEM().dll.AUTDLinkSOEMBuild(byref(link), self._soem)
        LinkSOEM().dll.AUTDLinkSOEMDelete(self._soem)
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
