"""
File: twincat.py
Project: link
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 27/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""

from datetime import timedelta

from pyautd3.internal.link import Link, LinkBuilder
from pyautd3.internal.utils import _validate_ptr
from pyautd3.native_methods.autd3capi import (
    NativeMethods as Base,
)
from pyautd3.native_methods.autd3capi_def import ControllerPtr, LinkBuilderPtr, LinkPtr
from pyautd3.native_methods.autd3capi_link_twincat import LinkRemoteTwinCATBuilderPtr, LinkTwinCATBuilderPtr
from pyautd3.native_methods.autd3capi_link_twincat import NativeMethods as LinkTwinCAT


class TwinCAT(Link):
    """Link using TwinCAT3."""

    class _Builder(LinkBuilder):
        _builder: LinkTwinCATBuilderPtr

        def __init__(self: "TwinCAT._Builder") -> None:
            self._builder = LinkTwinCAT().link_twin_cat()

        def with_timeout(self: "TwinCAT._Builder", timeout: timedelta) -> "TwinCAT._Builder":
            """Set timeout.

            Arguments:
            ---------
                timeout: Timeout
            """
            self._builder = LinkTwinCAT().link_twin_cat_with_timeout(self._builder, int(timeout.total_seconds() * 1000 * 1000 * 1000))
            return self

        def _link_builder_ptr(self: "TwinCAT._Builder") -> LinkBuilderPtr:
            return LinkTwinCAT().link_twin_cat_into_builder(self._builder)  # pragma: no cover

        def _resolve_link(self: "TwinCAT._Builder", _ptr: ControllerPtr) -> "TwinCAT":
            return TwinCAT(Base().link_get(_ptr))

    def __init__(self: "TwinCAT", ptr: LinkPtr) -> None:
        super().__init__(ptr)  # pragma: no cover

    @staticmethod
    def builder() -> _Builder:
        """Create TwinCAT link builder."""
        return TwinCAT._Builder()


class RemoteTwinCAT(Link):
    """Link for remote TwinCAT3 server via [ADS](https://github.com/Beckhoff/ADS) library."""

    class _Builder(LinkBuilder):
        _builder: LinkRemoteTwinCATBuilderPtr

        def __init__(self: "RemoteTwinCAT._Builder", server_ams_net_id: str) -> None:
            self._builder = _validate_ptr(LinkTwinCAT().link_remote_twin_cat(server_ams_net_id.encode("utf-8")))

        def with_server_ip(self: "RemoteTwinCAT._Builder", ip: str) -> "RemoteTwinCAT._Builder":
            """Set server IP address.

            Arguments:
            ---------
                ip: Server IP address
            """
            self._builder = LinkTwinCAT().link_remote_twin_cat_with_server_ip(self._builder, ip.encode("utf-8"))
            return self

        def with_client_ams_net_id(self: "RemoteTwinCAT._Builder", ams_net_id: str) -> "RemoteTwinCAT._Builder":
            """Set client AMS Net ID.

            Arguments:
            ---------
                ams_net_id: Client AMS Net ID
            """
            self._builder = LinkTwinCAT().link_remote_twin_cat_with_client_ams_net_id(self._builder, ams_net_id.encode("utf-8"))
            return self

        def with_timeout(self: "RemoteTwinCAT._Builder", timeout: timedelta) -> "RemoteTwinCAT._Builder":
            """Set timeout.

            Arguments:
            ---------
                timeout: Timeout
            """
            self._builder = LinkTwinCAT().link_remote_twin_cat_with_timeout(self._builder, int(timeout.total_seconds() * 1000 * 1000 * 1000))
            return self

        def _link_builder_ptr(self: "RemoteTwinCAT._Builder") -> LinkBuilderPtr:
            return LinkTwinCAT().link_remote_twin_cat_into_builder(self._builder)  # pragma: no cover

        def _resolve_link(self: "RemoteTwinCAT._Builder", _ptr: ControllerPtr) -> "RemoteTwinCAT":
            return RemoteTwinCAT(Base().link_get(_ptr))

    def __init__(self: "RemoteTwinCAT", ptr: LinkPtr) -> None:
        super().__init__(ptr)  # pragma: no cover

    @staticmethod
    def builder(server_ams_net_id: str) -> _Builder:
        """Create TwinCAT link builder.

        Arguments:
        ---------
            server_ams_net_id: Server AMS Net ID
        """
        return RemoteTwinCAT._Builder(server_ams_net_id)
