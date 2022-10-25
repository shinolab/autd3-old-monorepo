'''
File: constraint.py
Project: holo
Created Date: 25/10/2022
Author: Shun Suzuki
-----
Last Modified: 25/10/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''


from ctypes import c_void_p, byref

from pyautd3.native_methods.autd3capi_gain_holo import NativeMethods as GainHolo


class AmplitudeConstraint():
    def __init__(self):
        self.ptr = c_void_p()


class DontCare(AmplitudeConstraint):
    def __init__(self):
        super().__init__()
        GainHolo().dll.AUTDConstraintDontCare(byref(self.ptr))


class Normalize(AmplitudeConstraint):
    def __init__(self):
        super().__init__()
        GainHolo().dll.AUTDConstraintNormalize(byref(self.ptr))


class Uniform(AmplitudeConstraint):
    def __init__(self, value: float):
        super().__init__()
        GainHolo().dll.AUTDConstraintUniform(byref(self.ptr), value)


class Clamp(AmplitudeConstraint):
    def __init__(self):
        super().__init__()
        GainHolo().dll.AUTDConstraintClamp(byref(self.ptr))
