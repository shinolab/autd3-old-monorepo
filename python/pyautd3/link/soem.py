"""
File: soem.py
Project: link
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""

from datetime import timedelta
import ctypes
from ctypes import c_void_p, byref
from typing import List
from .link import Link

from pyautd3.native_methods.autd3capi_link_soem import NativeMethods as LinkSOEM
from pyautd3.native_methods.autd3capi_def import Level, TimerStrategy
from pyautd3.native_methods.autd3capi_link_soem import SyncMode


OnLostFunc = ctypes.CFUNCTYPE(None, ctypes.c_char_p)


class EtherCATAdapter:
    desc: str
    name: str

    def __init__(self, name: str, desc: str):
        self.desc = desc
        self.name = name

    def __repr__(self) -> str:
        return f"{self.desc}, {self.name}"


class SOEM:
    _builder: c_void_p

    def __init__(self):
        self._builder = LinkSOEM().link_soem()

    def ifname(self, ifname: str) -> "SOEM":
        self._builder = LinkSOEM().link_soem_ifname(
            self._builder, ifname.encode("utf-8")
        )
        return self

    def buf_size(self, size: int) -> "SOEM":
        self._builder = LinkSOEM().link_soem_buf_size(self._builder, size)
        return self

    def send_cycle(self, cycle: int) -> "SOEM":
        self._builder = LinkSOEM().link_soem_send_cycle(self._builder, cycle)
        return self

    def sync0_cycle(self, cycle: int) -> "SOEM":
        self._builder = LinkSOEM().link_soem_sync_0_cycle(self._builder, cycle)
        return self

    def on_lost(self, handle) -> "SOEM":
        self._builder = LinkSOEM().link_soem_on_lost(self._builder, handle)
        return self

    def timer_strategy(self, strategy: TimerStrategy) -> "SOEM":
        self._builder = LinkSOEM().link_soem_timer_strategy(self._builder, strategy)
        return self

    def sync_mode(self, mode: SyncMode) -> "SOEM":
        self._builder = LinkSOEM().link_soem_sync_mode(self._builder, mode)
        return self

    def state_check_interval(self, interval: timedelta) -> "SOEM":
        self._builder = LinkSOEM().link_soem_state_check_interval(
            self._builder, int(interval.total_seconds() / 1000)
        )
        return self

    def log_level(self, level: Level) -> "SOEM":
        self._builder = LinkSOEM().link_soem_log_level(self._builder, level)
        return self

    def log_func(self, level: Level, log_out, log_flush) -> "SOEM":
        self._builder = LinkSOEM().link_soem_log_func(
            self._builder, level, log_out, log_flush
        )
        return self

    def timeout(self, timeout: timedelta) -> "SOEM":
        self._builder = LinkSOEM().link_soem_timeout(
            self._builder, int(timeout.total_seconds() * 1000 * 1000 * 1000)
        )
        return self

    def build(self) -> Link:
        link = LinkSOEM().link_soem_build(self._builder)
        return Link(link)

    @staticmethod
    def enumerate_adapters() -> List[EtherCATAdapter]:
        size = ctypes.c_uint32(0)
        handle = LinkSOEM().get_adapter_pointer(byref(size))
        res = []
        for i in range(int(size)):
            sb_desc = ctypes.create_string_buffer(128)
            sb_name = ctypes.create_string_buffer(128)
            LinkSOEM().get_adapter(handle, i, sb_desc, sb_name)
            res.append(
                EtherCATAdapter(
                    sb_name.value.decode("utf-8"), sb_desc.value.decode("utf-8")
                )
            )

        LinkSOEM().free_adapter_pointer(handle)

        return res


class RemoteSOEM:
    _builder = c_void_p()

    def __init__(self, ip: str, port: int):
        self._builder = LinkSOEM().link_remote_soem(ip.encode("utf-8"), port)

    def timeout(self, timeout: timedelta) -> "RemoteSOEM":
        self._builder = LinkSOEM().link_remote_soem_timeout(
            self._builder, int(timeout.total_seconds() * 1000 * 1000 * 1000)
        )
        return self

    def build(self) -> Link:
        link = LinkSOEM().link_remote_soem_build(self._builder)
        return Link(link)
