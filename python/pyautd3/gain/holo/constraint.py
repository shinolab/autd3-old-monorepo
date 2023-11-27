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


from pyautd3.emit_intensity import EmitIntensity
from pyautd3.native_methods.autd3capi_gain_holo import EmissionConstraintPtr
from pyautd3.native_methods.autd3capi_gain_holo import NativeMethods as GainHolo


class EmissionConstraint:
    """Amplitude constraint."""

    _ptr: EmissionConstraintPtr

    def __init__(self: "EmissionConstraint", ptr: EmissionConstraintPtr) -> None:
        self._ptr = ptr

    @staticmethod
    def dont_care() -> "EmissionConstraint":
        """Do nothing (this is equivalent to `clamp(0, 1)`)."""
        return EmissionConstraint(GainHolo().gain_holo_constraint_dot_care())

    @staticmethod
    def normalize() -> "EmissionConstraint":
        """Normalize the value by dividing the maximum value."""
        return EmissionConstraint(GainHolo().gain_holo_constraint_normalize())

    @staticmethod
    def uniform(value: int | EmitIntensity) -> "EmissionConstraint":
        """Set all amplitudes to the specified value.

        Arguments:
        ---------
            value: emission intensity
        """
        return EmissionConstraint(GainHolo().gain_holo_constraint_uniform(EmitIntensity._cast(value).value))

    @staticmethod
    def clamp(min_v: int | EmitIntensity, max_v: int | EmitIntensity) -> "EmissionConstraint":
        """Clamp all amplitudes to the specified range.

        Arguments:
        ---------
            min_v: minimum emission intensity
            max_v: maximum emission intensity
        """
        return EmissionConstraint(
            GainHolo().gain_holo_constraint_clamp(
                EmitIntensity._cast(min_v).value,
                EmitIntensity._cast(max_v).value,
            ),
        )

    def _constraint_ptr(self: "EmissionConstraint") -> EmissionConstraintPtr:
        return self._ptr
