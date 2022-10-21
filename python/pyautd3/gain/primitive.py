'''
File: primitive.py
Project: gain
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 21/10/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''

from ctypes import byref
import numpy as np

from pyautd3.autd import Controller
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from .gain import Gain


class Focus(Gain):
    def __init__(self, pos, amp: float = 1.0):
        super().__init__()
        Base().dll.AUTDGainFocus(byref(self.ptr), pos[0], pos[1], pos[2], amp)

    def __del__(self):
        super().__del__()


class BesselBeam(Gain):
    def __init__(self, pos, dir, theta_z, amp: float = 1.0):
        super().__init__()
        Base().dll.AUTDGainBesselBeam(byref(self.ptr), pos[0], pos[1], pos[2], dir[0], dir[1], dir[2], theta_z, amp)

    def __del__(self):
        super().__del__()


class PlaneWave(Gain):
    def __init__(self, pos, dir, amp: float = 1.0):
        super().__init__()
        Base().dll.AUTDGainPlaneWave(byref(self.ptr), pos[0], pos[1], pos[2], dir[0], dir[1], dir[2], amp)

    def __del__(self):
        super().__del__()


class Custom(Gain):
    def __init__(self, amp, phase):
        super().__init__()
        p_size = len(phase)
        phase = np.ctypeslib.as_ctypes(np.array(phase).astype(np.double))
        amp = np.ctypeslib.as_ctypes(np.array(amp).astype(np.double))
        Base().dll.AUTDGainCustom(byref(self.ptr), amp, phase, p_size)

    def __del__(self):
        super().__del__()


class Null(Gain):
    def __init__(self):
        super().__init__()
        Base().dll.AUTDGainNull(byref(self.ptr))

    def __del__(self):
        super().__del__()


class Grouped(Gain):
    def __init__(self, autd: Controller):
        super().__init__()
        Base().dll.AUTDGainGrouped(byref(self.ptr), autd.p_cnt)

    def __del__(self):
        super().__del__()

    def add(self, dev_idx: int, gain: Gain):
        Base().dll.AUTDGainGroupedAdd(self.ptr, dev_idx, gain.ptr)
