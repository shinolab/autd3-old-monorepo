'''
File: soem.py
Project: link
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 20/03/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''

import warnings
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
        self._ifname = None
        self._buf_size = 0
        self._send_cycle = 2
        self._sync0_cycle = 2
        self._on_lost = None
        self._timer_strategy = TimerStrategy.Sleep
        self._sync_mode = SyncMode.FreeRun
        self._check_interval = 500
        self._debug_level = DebugLevel.Info
        self._debug_log_out = None
        self._debug_log_flush = None

    def ifname(self, ifname: str):
        self._ifname = ifname
        return self

    def buf_size(self, size: int):
        self._buf_size = size
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
        warnings.warn('This methods is deprecated. Use timer_strategy(TimerStrategy) instead.', UserWarning)
        self._timer_strategy = TimerStrategy.BusyWait if flag else TimerStrategy.Sleep
        return self

    def timer_strategy(self, strategy: TimerStrategy):
        self._timer_strategy = strategy
        return self

    def freerun(self, flag: bool):
        warnings.warn('This methods is deprecated. Use sync_mode(SyncMode) instead.', UserWarning)
        self._sync_mode = SyncMode.FreeRun if flag else SyncMode.DC
        return self

    def sync_mode(self, mode: SyncMode):
        self._sync_mode = mode
        return self

    def check_interval(self, interval: int):
        self._check_interval = interval
        return self

    def debug_level(self, level: DebugLevel):
        self._debug_level = level
        return self

    def debug_log(self, log_out, log_flush):
        self._debug_log_out = log_out
        self._debug_log_flush = log_flush
        return self

    def build(self):
        LinkSOEM().init_dll()
        link = c_void_p()
        LinkSOEM().dll.AUTDLinkSOEM(byref(link), self._ifname.encode('utf-8') if self._ifname is not None else None,
                                    self._buf_size, self._sync0_cycle, self._send_cycle, self._sync_mode == SyncMode.FreeRun, self._on_lost, int(
                                        self._timer_strategy), self._check_interval,
                                    int(self._debug_level), self._debug_log_out, self._debug_log_flush)
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
