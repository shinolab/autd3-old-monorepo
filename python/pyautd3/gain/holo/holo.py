'''
File: holo.py
Project: holo
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 21/10/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''

from ctypes import byref, c_double

from pyautd3.native_methods.autd3capi_gain_holo import NativeMethods as GainHolo
from pyautd3.gain.gain import Gain


class AmplitudeConstraint():
    def __init__(self, id, value):
        self._id = id
        self.value = None if value is None else c_double(value)

    def id(self):
        return self._id

    def ptr(self):
        return None if self.value is None else byref(self.value)


class DontCare(AmplitudeConstraint):
    def __init__(self):
        super().__init__(0, None)


class Normalize(AmplitudeConstraint):
    def __init__(self):
        super().__init__(1, None)


class Uniform(AmplitudeConstraint):
    def __init__(self, value: float):
        super().__init__(2, value)


class Clamp(AmplitudeConstraint):
    def __init__(self):
        super().__init__(3, None)


class Holo(Gain):
    def __init__(self):
        super().__init__()
        self._constraint = Normalize()

    def __del__(self):
        super().__del__()

    def add(self, focus, amp):
        GainHolo().dll.AUTDGainHoloAdd(self.ptr, focus[0], focus[1], focus[2], amp)

    def amplitude_constraint(self, constraint: AmplitudeConstraint):
        GainHolo().dll.AUTDSetConstraint(self.ptr, constraint.id(), constraint.ptr())
