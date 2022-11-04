'''
File: remote_twincat.py
Project: link
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 04/11/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''


from ctypes import c_void_p, byref

from .link import Link

from pyautd3.native_methods.autd3capi_link_remote_twincat import NativeMethods as LinkRemoteTwinCAT


class RemoteTwinCAT:
    def __init__(self, remote_ip_addr, remote_ams_net_id):
        self._remote_ip_addr = remote_ip_addr
        self._remote_ams_net_id = remote_ams_net_id
        self._local_ams_net_id = ''

    def local_ams_net_id(self, local_ams_net_id):
        self._local_ams_net_id = local_ams_net_id
        return self

    def build(self):
        link = c_void_p()
        LinkRemoteTwinCAT().dll.AUTDLinkRemoteTwinCAT(byref(link), self._remote_ip_addr.encode('utf-8'),
                                                      self._remote_ams_net_id.encode('utf-8'),
                                                      self._local_ams_net_id.encode('utf-8'))
        return Link(link)
