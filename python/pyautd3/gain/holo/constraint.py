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


from abc import ABCMeta


class AmplitudeConstraint(metaclass=ABCMeta):
    pass


class DontCare(AmplitudeConstraint):
    def __init__(self):
        pass


class Normalize(AmplitudeConstraint):
    def __init__(self):
        pass


class Uniform(AmplitudeConstraint):
    value: float

    def __init__(self, value: float):
        self.value = value


class Clamp(AmplitudeConstraint):
    min: float
    max: float

    def __init__(self, min: float, max: float):
        self.min = min
        self.max = max
