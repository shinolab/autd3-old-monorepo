"""
File: emit_intensity.py
Project: pyautd3
Created Date: 12/11/2023
Author: Shun Suzuki
-----
Last Modified: 12/11/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from ctypes import c_uint8

from .native_methods.autd3capi_def import DEFAULT_CORRECTED_ALPHA
from .native_methods.autd3capi_def import NativeMethods as Def


class EmitIntensity:
    """Ultrasound emission intensity."""

    _value: c_uint8

    def __init__(self: "EmitIntensity", intensity: int) -> None:
        self._value = c_uint8(intensity)

    @staticmethod
    def with_correction_alpha(value: int, alpha: float) -> "EmitIntensity":
        """Create by normalized intensity with correction."""
        return EmitIntensity(int(Def().emit_intensity_with_correction_alpha(value, alpha)))

    @staticmethod
    def with_correction(value: int) -> "EmitIntensity":
        """Create by normalized intensity with correction."""
        return EmitIntensity.with_correction_alpha(value, DEFAULT_CORRECTED_ALPHA)

    @property
    def value(self: "EmitIntensity") -> int:
        """Emission intensity."""
        return self._value.value

    @staticmethod
    def _cast(value: "EmitIntensity | int") -> "EmitIntensity":
        match value:
            case int():
                return EmitIntensity(value)
            case EmitIntensity():
                return value
            case _:
                err = f"Invalid type: {type(value)}"
                raise TypeError(err)

    @staticmethod
    def maximum() -> "EmitIntensity":
        """Maximum intensity."""
        return EmitIntensity(0xFF)

    @staticmethod
    def minimum() -> "EmitIntensity":
        """Minimum intensity."""
        return EmitIntensity(0x00)

    def __floordiv__(self: "EmitIntensity", other: int) -> "EmitIntensity":
        """Divide by integer."""
        return EmitIntensity(self.value // other)
