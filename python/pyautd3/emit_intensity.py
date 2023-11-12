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


from .internal.utils import _validate_int
from .native_methods.autd3capi_def import DEFAULT_CORRECTED_ALPHA
from .native_methods.autd3capi_def import NativeMethods as Def


class EmitIntensity:
    """Ultrasound emission intensity."""

    _pulse_width: int

    def __new__(cls: type["EmitIntensity"]) -> "EmitIntensity":
        """DO NOT USE THIS CONSTRUCTOR."""
        raise NotImplementedError

    @classmethod
    def __private_new__(cls: type["EmitIntensity"], pulse_width: int) -> "EmitIntensity":
        ins = super().__new__(cls)
        ins._pulse_width = pulse_width
        return ins

    @staticmethod
    def new_normalized(value: float) -> "EmitIntensity":
        """Create by normalized intensity."""
        res = _validate_int(Def().emit_intensity_normalized_into(value))
        return EmitIntensity.__private_new__(res)

    @staticmethod
    def new_normalized_corrected_with_alpha(value: float, alpha: float) -> "EmitIntensity":
        """Create by normalized intensity with correction."""
        res = _validate_int(Def().emit_intensity_normalized_corrected_into(value, alpha))
        return EmitIntensity.__private_new__(res)

    @staticmethod
    def new_normalized_corrected(value: float) -> "EmitIntensity":
        """Create by normalized intensity with correction."""
        return EmitIntensity.new_normalized_corrected_with_alpha(value, DEFAULT_CORRECTED_ALPHA)

    @staticmethod
    def new_duty_ratio(value: float) -> "EmitIntensity":
        """Create by duty ratio."""
        res = _validate_int(Def().emit_intensity_duty_ratio_into(value))
        return EmitIntensity.__private_new__(res)

    @staticmethod
    def new_pulse_width(value: int) -> "EmitIntensity":
        """Create by pulse width."""
        res = _validate_int(Def().emit_intensity_pulse_width_into(value))
        return EmitIntensity.__private_new__(res)

    @property
    def pulse_width(self: "EmitIntensity") -> int:
        """Pulse width."""
        return self._pulse_width

    @property
    def normalized(self: "EmitIntensity") -> float:
        """Normalized intensity."""
        return float(Def().emit_intensity_normalized_from(self._pulse_width))

    @property
    def duty_ratio(self: "EmitIntensity") -> float:
        """Duty ratio."""
        return float(Def().emit_intensity_duty_ratio_from(self._pulse_width))

    @staticmethod
    def _cast(value: "EmitIntensity | int | float") -> "EmitIntensity":
        match value:
            case int():
                return EmitIntensity.new_pulse_width(value)
            case float():
                return EmitIntensity.new_normalized(value)
            case EmitIntensity():
                return value
            case _:
                err = f"Invalid type: {type(value)}"
                raise TypeError(err)
