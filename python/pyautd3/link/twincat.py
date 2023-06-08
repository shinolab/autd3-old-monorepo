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

from pyautd3.autd_error import AUTDError

from .link import Link

from pyautd3.native_methods.autd3capi_link_twincat import NativeMethods as LinkTwinCAT


class TwinCAT(Link):
    def __init__(self):
        err = ctypes.create_string_buffer(256)
        self._ptr = LinkTwinCAT().link_twin_cat(err)
        if self._ptr._0 is None:
            raise AUTDError(err)

    def with_timeout(self, timeout: timedelta):
        self._ptr = LinkTwinCAT().link_twin_cat_timeout(
            self._ptr, int(timeout.total_seconds() * 1000 * 1000 * 1000)
        )
        return self


class RemoteTwinCAT(Link):
    def __init__(self, server_ams_net_id: str):
        err = ctypes.create_string_buffer(256)
        self._ptr = LinkTwinCAT().link_remote_twin_cat(
            server_ams_net_id.encode("utf-8"), err
        )
        if self._ptr._0 is None:
            raise AUTDError(err)

    def with_server_ip(self, ip: str) -> "RemoteTwinCAT":
        self._ptr = LinkTwinCAT().link_remote_twin_cat_server_ip(
            self._ptr, ip.encode("utf-8")
        )
        return self

    def with_client_ams_net_id(self, id: str) -> "RemoteTwinCAT":
        self._ptr = LinkTwinCAT().link_remote_twin_cat_client_ams_net_id(
            self._ptr, id.encode("utf-8")
        )
        return self

    def with_timeout(self, timeout: timedelta) -> "RemoteTwinCAT":
        self._ptr = LinkTwinCAT().link_remote_twin_cat_timeout(
            self._ptr, int(timeout.total_seconds() * 1000 * 1000 * 1000)
        )
        return self
