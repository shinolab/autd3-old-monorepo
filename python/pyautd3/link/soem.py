'''
File: soem.py
Project: link
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 29/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

'''

from datetime import timedelta
import ctypes
from typing import List
from .link import Link

from pyautd3.native_methods.autd3capi_link_soem import NativeMethods as LinkSOEM
from pyautd3.native_methods.autd3capi_def import Level, TimerStrategy
from pyautd3.native_methods.autd3capi_link_soem import SyncMode


OnLostFunc = ctypes.CFUNCTYPE(None, ctypes.c_char_p)


class EtherCATAdapter:
    """Ethernet adapter

    """

    desc: str
    """Description of the adapter"""

    name: str
    """Name of the adapter"""

    def __init__(self, name: str, desc: str):
        self.desc = desc
        self.name = name

    def __repr__(self) -> str:
        return f"{self.desc}, {self.name}"


class SOEM(Link):
    def __init__(self):
        super().__init__(LinkSOEM().link_soem())

    def with_ifname(self, ifname: str) -> "SOEM":
        """Set network interface name

        Arguments:
        - `ifname` - Network interface name (e.g. "eth0").
          If empty, this link will automatically find the network interface that is connected to AUTD3 devices.
        """

        self._ptr = LinkSOEM().link_soem_with_ifname(self._ptr, ifname.encode("utf-8"))
        return self

    def with_buf_size(self, size: int) -> "SOEM":
        """Set send buffer size

        Arguments:
        - `size` - send buffer size
        """

        self._ptr = LinkSOEM().link_soem_with_buf_size(self._ptr, size)
        return self

    def with_send_cycle(self, cycle: int) -> "SOEM":
        """Set send cycle

        Arguments:
        - `cycle` - send cycle  (the unit is 500us)
        """

        self._ptr = LinkSOEM().link_soem_with_send_cycle(self._ptr, cycle)
        return self

    def with_sync0_cycle(self, cycle: int) -> "SOEM":
        """Set sync0 cycle

        Arguments:
        - `cycle` - Sync0 cycle  (the unit is 500us)
        """

        self._ptr = LinkSOEM().link_soem_with_sync_0_cycle(self._ptr, cycle)
        return self

    def with_on_lost(self, handle) -> "SOEM":
        """Set callback function when the link is lost

        Arguments:
        - `handle` - Callback function
        """

        self._ptr = LinkSOEM().link_soem_with_on_lost(self._ptr, handle)
        return self

    def with_timer_strategy(self, strategy: TimerStrategy) -> "SOEM":
        """Set timer strategy

        Arguments:
        - `strategy` - Timer strategy
        """

        self._ptr = LinkSOEM().link_soem_with_timer_strategy(self._ptr, strategy)
        return self

    def with_sync_mode(self, mode: SyncMode) -> "SOEM":
        """Set sync mode

        See [Beckhoff's site](https://infosys.beckhoff.com/content/1033/ethercatsystem/2469122443.html) for more details.
        """

        self._ptr = LinkSOEM().link_soem_with_sync_mode(self._ptr, mode)
        return self

    def with_state_check_interval(self, interval: timedelta) -> "SOEM":
        """Set state check interval

        Arguments:
        - `interval` - State check interval
        """

        self._ptr = LinkSOEM().link_soem_with_state_check_interval(
            self._ptr, int(interval.total_seconds() / 1000)
        )
        return self

    def with_log_level(self, level: Level) -> "SOEM":
        """Set timeout

        Arguments:
        - `timeout` - Timeout
        """

        self._ptr = LinkSOEM().link_soem_with_log_level(self._ptr, level)
        return self

    def with_log_func(self, log_out, log_flush) -> "SOEM":
        """Set log function

        By default, this link will display log messages on the console.

        Arguments:
        - `log_out` - Log output function
        - `log_flush` - Log flush function
        """

        self._ptr = LinkSOEM().link_soem_with_log_func(self._ptr, log_out, log_flush)
        return self

    def with_timeout(self, timeout: timedelta) -> "SOEM":
        """Set timeout

        Arguments:
        - `timeout` - Timeout
        """

        self._ptr = LinkSOEM().link_soem_with_timeout(
            self._ptr, int(timeout.total_seconds() * 1000 * 1000 * 1000)
        )
        return self

    @staticmethod
    def enumerate_adapters() -> List[EtherCATAdapter]:
        """Enumerate ethernet adapters

        """

        handle = LinkSOEM().adapter_pointer()
        size = LinkSOEM().adapter_get_size(handle)

        def get_adapter(i: int) -> EtherCATAdapter:
            sb_desc = ctypes.create_string_buffer(128)
            sb_name = ctypes.create_string_buffer(128)
            LinkSOEM().adapter_get_adapter(handle, i, sb_desc, sb_name)
            return EtherCATAdapter(
                sb_name.value.decode("utf-8"), sb_desc.value.decode("utf-8")
            )

        res = list(map(get_adapter, range(int(size))))

        LinkSOEM().adapter_pointer_delete(handle)

        return res


class RemoteSOEM(Link):
    """Link to connect to remote SOEMServer

    """

    def __init__(self, addr: str):
        """Constructor

        Arguments:
        - `addr` - IP address and port of SOEMServer (e.g. "127.0.0.1:8080")
        """

        err = ctypes.create_string_buffer(256)
        super().__init__(LinkSOEM().link_remote_soem(addr.encode("utf-8"), err))
        if self._ptr._0 is None:
            raise RuntimeError(err.value.decode("utf-8"))

    def with_timeout(self, timeout: timedelta) -> "RemoteSOEM":
        """Set timeout

        Arguments:
        - `timeout` - Timeout
        """

        self._ptr = LinkSOEM().link_remote_soem_with_timeout(
            self._ptr, int(timeout.total_seconds() * 1000 * 1000 * 1000)
        )
        return self
