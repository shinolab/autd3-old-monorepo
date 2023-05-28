"""
File: twincat.py
Project: link
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""

import ctypes
from datetime import timedelta
from ctypes import c_void_p

from pyautd3.autd_error import AUTDError

from .link import Link

from pyautd3.native_methods.autd3capi_link_twincat import NativeMethods as LinkTwinCAT


class TwinCAT:
    _builder: c_void_p

    def __init__(self):
        self._builder = LinkTwinCAT().link_twin_cat()

    def timeout(self, timeout: timedelta):
        self._builder = LinkTwinCAT().link_twin_cat_timeout(
            self._builder, int(timeout.total_seconds() * 1000 * 1000 * 1000)
        )
        return self

    def build(self) -> Link:
        err = ctypes.create_string_buffer(256)
        link = LinkTwinCAT().link_twin_cat_build(self._builder, err)
        if not link:
            raise AUTDError(err)
        return Link(link)


class RemoteTwinCAT:
    _builder: c_void_p

    def __init__(self, server_ams_net_id: str):
        self._builder = LinkTwinCAT().link_remote_twin_cat(
            server_ams_net_id.encode("utf-8")
        )

    def server_ip(self, ip: str) -> "RemoteTwinCAT":
        self._builder = LinkTwinCAT().link_remote_twin_cat_server_ip(
            self._builder, ip.encode("utf-8")
        )
        return self

    def client_ams_net_id(self, id: str) -> "RemoteTwinCAT":
        self._builder = LinkTwinCAT().link_remote_twin_cat_client_ams_net_id(
            self._builder, id.encode("utf-8")
        )
        return self

    def timeout(self, timeout: timedelta) -> "RemoteTwinCAT":
        self._builder = LinkTwinCAT().link_remote_twin_cat_timeout(
            self._builder, int(timeout.total_seconds() * 1000 * 1000 * 1000)
        )
        return self

    def build(self) -> Link:
        err = ctypes.create_string_buffer(256)
        link = LinkTwinCAT().link_remote_twin_cat_build(self._builder, err)
        if not link:
            raise AUTDError(err)
        return Link(link)
