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


from pyautd3.native_methods.autd3capi_gain_holo import NativeMethods as GainHolo
from pyautd3.native_methods.autd3capi_gain_holo import ConstraintPtr


class AmplitudeConstraint:
    _ptr: ConstraintPtr

    def __init__(self, ptr: ConstraintPtr):
        self._ptr = ptr

    @staticmethod
    def dont_care() -> "AmplitudeConstraint":
        return AmplitudeConstraint(GainHolo().gain_holo_dot_care_constraint())

    @staticmethod
    def normalize() -> "AmplitudeConstraint":
        return AmplitudeConstraint(GainHolo().gain_holo_normalize_constraint())

    @staticmethod
    def uniform(value: float) -> "AmplitudeConstraint":
        return AmplitudeConstraint(GainHolo().gain_holo_uniform_constraint(value))

    @staticmethod
    def clamp(min: float, max: float) -> "AmplitudeConstraint":
        return AmplitudeConstraint(GainHolo().gain_holo_clamp_constraint(min, max))

    def ptr(self) -> ConstraintPtr:
        return self._ptr
