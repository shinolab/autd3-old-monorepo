"""
File: constraint.py
Project: holo
Created Date: 25/10/2022
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""


from abc import ABCMeta, abstractmethod
from pyautd3.gain.holo.holo import Holo

from pyautd3.native_methods.autd3capi_gain_holo import NativeMethods as GainHolo


class AmplitudeConstraint(metaclass=ABCMeta):
    @abstractmethod
    def set(self, holo: Holo):
        pass


class DontCare(AmplitudeConstraint):
    def __init__(self):
        pass

    def set(self, holo: Holo):
        GainHolo().gain_holo_set_dot_care_constraint(holo.ptr)


class Normalize(AmplitudeConstraint):
    def __init__(self):
        pass

    def set(self, holo: Holo):
        GainHolo().gain_holo_set_normalize_constraint(holo.ptr)


class Uniform(AmplitudeConstraint):
    value: float

    def __init__(self, value: float):
        self.value = value

    def set(self, holo: Holo):
        GainHolo().gain_holo_set_uniform_constraint(holo.ptr, self.value)


class Clamp(AmplitudeConstraint):
    min: float
    max: float

    def __init__(self, min: float, max: float):
        self.min = min
        self.max = max

    def set(self, holo: Holo):
        GainHolo().gain_holo_set_clamp_constraint(holo.ptr, self.min, self.max)
