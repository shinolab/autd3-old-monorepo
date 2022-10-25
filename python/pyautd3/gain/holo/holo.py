'''
File: holo.py
Project: holo
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 25/10/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''

from pyautd3.native_methods.autd3capi_gain_holo import NativeMethods as GainHolo
from pyautd3.gain.gain import Gain
from .constraint import AmplitudeConstraint


class Holo(Gain):
    def __init__(self):
        super().__init__()

    def __del__(self):
        super().__del__()

    def add(self, focus, amp):
        GainHolo().dll.AUTDGainHoloAdd(self.ptr, focus[0], focus[1], focus[2], amp)

    def amplitude_constraint(self, constraint: AmplitudeConstraint):
        GainHolo().dll.AUTDSetConstraint(self.ptr, constraint.ptr)
