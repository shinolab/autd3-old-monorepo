"""
File: radiation_pressure.py
Project: modulation
Created Date: 10/10/2023
Author: Shun Suzuki
-----
Last Modified: 27/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from typing import Generic, TypeVar

from pyautd3.internal.modulation import IModulation
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import ModulationPtr

M = TypeVar("M", bound=IModulation)


class RadiationPressure(IModulation, Generic[M]):
    """Modulation for modulating radiation pressure."""

    _m: M

    def __init__(self: "RadiationPressure", m: M) -> None:
        self._m = m

    def _modulation_ptr(self: "RadiationPressure") -> ModulationPtr:
        return Base().modulation_with_radiation_pressure(self._m._modulation_ptr())


def __with_radiation_pressure(self: M) -> RadiationPressure:
    """Apply modulation to radiation pressure instead of amplitude."""
    return RadiationPressure(self)


IModulation.with_radiation_pressure = __with_radiation_pressure  # type: ignore[method-assign]
