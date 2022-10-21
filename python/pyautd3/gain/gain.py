'''
File: gain.py
Project: gain
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 21/10/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''

from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.autd import Body


class Gain(Body):
    def __init__(self):
        super().__init__()

    def __del__(self):
        Base().dll.AUTDDeleteGain(self.ptr)
