'''
File: soem.py
Project: link
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

'''

from datetime import timedelta
import ctypes
from typing import List

from pyautd3.internal.link import LinkBuilder
from pyautd3.native_methods.autd3capi_link_soem import NativeMethods as LinkSOEM
from pyautd3.native_methods.autd3capi_link_soem import LinkRemoteSOEMBuilderPtr, LinkSOEMBuilderPtr
from pyautd3.native_methods.autd3capi_def import LinkBuilderPtr, TimerStrategy
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


class SOEM:
    class _Builder(LinkBuilder):
        _builder: LinkSOEMBuilderPtr

        def __init__(self):
            self._builder = LinkSOEM().link_soem()

        def with_ifname(self, ifname: str) -> "SOEM._Builder":
            """Set network interface name

            Arguments:
            - `ifname` - Network interface name (e.g. "eth0").
            If empty, this link will automatically find the network interface that is connected to AUTD3 devices.
            """

            self._builder = LinkSOEM().link_soem_with_ifname(self._builder, ifname.encode("utf-8"))
            return self

        def with_buf_size(self, size: int) -> "SOEM._Builder":
            """Set send buffer size

            Arguments:
            - `size` - send buffer size
            """

            self._builder = LinkSOEM().link_soem_with_buf_size(self._builder, size)
            return self

        def with_send_cycle(self, cycle: int) -> "SOEM._Builder":
            """Set send cycle

            Arguments:
            - `cycle` - send cycle  (the unit is 500us)
            """

            self._builder = LinkSOEM().link_soem_with_send_cycle(self._builder, cycle)
            return self

        def with_sync0_cycle(self, cycle: int) -> "SOEM._Builder":
            """Set sync0 cycle

            Arguments:
            - `cycle` - Sync0 cycle  (the unit is 500us)
            """

            self._builder = LinkSOEM().link_soem_with_sync_0_cycle(self._builder, cycle)
            return self

        def with_on_lost(self, handle) -> "SOEM._Builder":
            """Set callback function when the link is lost

            Arguments:
            - `handle` - Callback function
            """

            self._builder = LinkSOEM().link_soem_with_on_lost(self._builder, handle)
            return self

        def with_timer_strategy(self, strategy: TimerStrategy) -> "SOEM._Builder":
            """Set timer strategy

            Arguments:
            - `strategy` - Timer strategy
            """

            self._builder = LinkSOEM().link_soem_with_timer_strategy(self._builder, strategy)
            return self

        def with_sync_mode(self, mode: SyncMode) -> "SOEM._Builder":
            """Set sync mode

            See [Beckhoff's site](https://infosys.beckhoff.com/content/1033/ethercatsystem/2469122443.html) for more details.
            """

            self._builder = LinkSOEM().link_soem_with_sync_mode(self._builder, mode)
            return self

        def with_state_check_interval(self, interval: timedelta) -> "SOEM._Builder":
            """Set state check interval

            Arguments:
            - `interval` - State check interval
            """

            self._builder = LinkSOEM().link_soem_with_state_check_interval(
                self._builder, int(interval.total_seconds() / 1000)
            )
            return self

        def with_timeout(self, timeout: timedelta) -> "SOEM._Builder":
            """Set timeout

            Arguments:
            - `timeout` - Timeout
            """

            self._builder = LinkSOEM().link_soem_with_timeout(
                self._builder, int(timeout.total_seconds() * 1000 * 1000 * 1000)
            )
            return self

        def _ptr(self) -> LinkBuilderPtr:
            return LinkSOEM().link_soem_into_builder(self._builder)

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

    @staticmethod
    def builder() -> _Builder:
        return SOEM._Builder()


class RemoteSOEM:
    """Link to connect to remote SOEMServer

    """

    class _Builder(LinkBuilder):
        _builder: LinkRemoteSOEMBuilderPtr

        def __init__(self, addr: str):
            err = ctypes.create_string_buffer(256)
            self._builder = LinkSOEM().link_remote_soem(addr.encode("utf-8"), err)
            if self._builder._0 is None:
                raise RuntimeError(err.value.decode("utf-8"))

        def with_timeout(self, timeout: timedelta) -> "RemoteSOEM._Builder":
            """Set timeout

            Arguments:
            - `timeout` - Timeout
            """

            self._builder = LinkSOEM().link_remote_soem_with_timeout(
                self._builder, int(timeout.total_seconds() * 1000 * 1000 * 1000)
            )
            return self

        def _ptr(self) -> LinkBuilderPtr:
            return LinkSOEM().link_remote_soem_into_builder(self._builder)

    @staticmethod
    def builder(addr: str) -> _Builder:
        """Constructor

        Arguments:
        - `addr` - IP address and port of SOEMServer (e.g. "127.0.0.1:8080")
        """

        return RemoteSOEM._Builder(addr)
