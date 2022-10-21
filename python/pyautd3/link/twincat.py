'''
File: twincat.py
Project: link
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 21/10/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''

from ctypes import c_void_p, byref

from .link import Link

from pyautd3.native_methods.autd3capi_link_twincat import NativeMethods as LinkTwinCAT


class TwinCAT:
    def __init__(self):
        pass

    def build(self):
        link = c_void_p()
        LinkTwinCAT().dll.AUTDLinkTwinCAT(byref(link))
        return Link(link)
