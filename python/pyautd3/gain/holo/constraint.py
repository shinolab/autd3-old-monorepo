"""
File: constraint.py
Project: holo
Created Date: 25/10/2022
Author: Shun Suzuki
-----
Last Modified: 29/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""


from pyautd3.native_methods.autd3capi_gain_holo import ConstraintPtr
from pyautd3.native_methods.autd3capi_gain_holo import NativeMethods as GainHolo


class AmplitudeConstraint:
    """Amplitude constraint."""

    _ptr: ConstraintPtr

    def __init__(self: "AmplitudeConstraint", ptr: ConstraintPtr) -> None:
        self._ptr = ptr

    @staticmethod
    def dont_care() -> "AmplitudeConstraint":
        """Do nothing (this is equivalent to `clamp(0, 1)`)."""
        return AmplitudeConstraint(GainHolo().gain_holo_constraint_dot_care())

    @staticmethod
    def normalize() -> "AmplitudeConstraint":
        """Normalize the value by dividing the maximum value."""
        return AmplitudeConstraint(GainHolo().gain_holo_constraint_normalize())

    @staticmethod
    def uniform(value: float) -> "AmplitudeConstraint":
        """Set all amplitudes to the specified value.

        Arguments:
        ---------
            value: amplitude
        """
        return AmplitudeConstraint(GainHolo().gain_holo_constraint_uniform(value))

    @staticmethod
    def clamp(min_v: float, max_v: float) -> "AmplitudeConstraint":
        """Clamp all amplitudes to the specified range.

        Arguments:
        ---------
            min_v: minimum amplitude
            max_v: maximum amplitude
        """
        return AmplitudeConstraint(GainHolo().gain_holo_constraint_clamp(min_v, max_v))

    def _constraint_ptr(self: "AmplitudeConstraint") -> ConstraintPtr:
        return self._ptr
