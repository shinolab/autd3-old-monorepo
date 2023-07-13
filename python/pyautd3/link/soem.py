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


class SOEM(Link):
    def __init__(self):
        super().__init__(LinkSOEM().link_soem())

    def with_ifname(self, ifname: str) -> "SOEM":
        self._ptr = LinkSOEM().link_soem_ifname(self._ptr, ifname.encode("utf-8"))
        return self

    def with_buf_size(self, size: int) -> "SOEM":
        self._ptr = LinkSOEM().link_soem_buf_size(self._ptr, size)
        return self

    def with_send_cycle(self, cycle: int) -> "SOEM":
        self._ptr = LinkSOEM().link_soem_send_cycle(self._ptr, cycle)
        return self

    def with_sync0_cycle(self, cycle: int) -> "SOEM":
        self._ptr = LinkSOEM().link_soem_sync_0_cycle(self._ptr, cycle)
        return self

    def with_on_lost(self, handle) -> "SOEM":
        self._ptr = LinkSOEM().link_soem_on_lost(self._ptr, handle)
        return self

    def with_timer_strategy(self, strategy: TimerStrategy) -> "SOEM":
        self._ptr = LinkSOEM().link_soem_timer_strategy(self._ptr, strategy)
        return self

    def with_sync_mode(self, mode: SyncMode) -> "SOEM":
        self._ptr = LinkSOEM().link_soem_sync_mode(self._ptr, mode)
        return self

    def with_state_check_interval(self, interval: timedelta) -> "SOEM":
        self._ptr = LinkSOEM().link_soem_state_check_interval(
            self._ptr, int(interval.total_seconds() / 1000)
        )
        return self

    def with_log_level(self, level: Level) -> "SOEM":
        self._ptr = LinkSOEM().link_soem_log_level(self._ptr, level)
        return self

    def with_log_func(self, log_out, log_flush) -> "SOEM":
        self._ptr = LinkSOEM().link_soem_log_func(self._ptr, log_out, log_flush)
        return self

    def with_timeout(self, timeout: timedelta) -> "SOEM":
        self._ptr = LinkSOEM().link_soem_timeout(
            self._ptr, int(timeout.total_seconds() * 1000 * 1000 * 1000)
        )
        return self

    @staticmethod
    def enumerate_adapters() -> List[EtherCATAdapter]:
        handle = LinkSOEM().get_adapter_pointer()
        size = LinkSOEM().get_adapter_size(handle)

        def get_adapter(i: int) -> EtherCATAdapter:
            sb_desc = ctypes.create_string_buffer(128)
            sb_name = ctypes.create_string_buffer(128)
            LinkSOEM().get_adapter(handle, i, sb_desc, sb_name)
            return EtherCATAdapter(
                sb_name.value.decode("utf-8"), sb_desc.value.decode("utf-8")
            )

        res = list(map(get_adapter, range(int(size.value))))

        LinkSOEM().free_adapter_pointer(handle)

        return res


class RemoteSOEM(Link):
    def __init__(self, addr: str):
        err = ctypes.create_string_buffer(256)
        super().__init__(LinkSOEM().link_remote_soem(addr.encode("utf-8"), err))
        if self._ptr._0 is None:
            raise RuntimeError(err.value.decode("utf-8"))

    def with_timeout(self, timeout: timedelta) -> "RemoteSOEM":
        self._ptr = LinkSOEM().link_remote_soem_timeout(
            self._ptr, int(timeout.total_seconds() * 1000 * 1000 * 1000)
        )
        return self
