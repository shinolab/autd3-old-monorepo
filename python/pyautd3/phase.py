"""
File: phase.py
Project: pyautd3
Created Date: 02/12/2023
Author: Shun Suzuki
-----
Last Modified: 02/12/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from ctypes import c_uint8

from .native_methods.autd3capi_def import NativeMethods as Def


class Phase:
    """Ultrasound phase."""

    _value: c_uint8

    def __init__(self: "Phase", phase: int) -> None:
        self._value = c_uint8(phase)

    @staticmethod
    def from_rad(value: float) -> "Phase":
        """Create by radian."""
        return Phase(int(Def().phase_from_rad(value)))

    @property
    def value(self: "Phase") -> int:
        """Phase."""
        return self._value.value

    @property
    def radian(self: "Phase") -> float:
        """Radian."""
        return float(Def().phase_to_rad(self.value))

    class _UnitRad:
        def __new__(cls: type["Phase._UnitRad"]) -> "Phase._UnitRad":
            """DO NOT USE THIS CONSTRUCTOR."""
            raise NotImplementedError

        @classmethod
        def __private_new__(cls: type["Phase._UnitRad"]) -> "Phase._UnitRad":
            return super().__new__(cls)

        def __rmul__(self: "Phase._UnitRad", other: float) -> "Phase":
            return Phase.from_rad(other)


rad: Phase._UnitRad = Phase._UnitRad.__private_new__()
