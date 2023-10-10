'''
File: radiation_pressure.py
Project: modulation
Created Date: 10/10/2023
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import ModulationPtr

from pyautd3.internal.modulation import IModulation


class RadiationPressure(IModulation):
    """Modulation for modulating radiation pressure

    """

    _m: IModulation

    def __init__(self, m: IModulation):
        self._m = m

    def modulation_ptr(self) -> ModulationPtr:
        return Base().modulation_with_radiation_pressure(self._m.modulation_ptr())


def __with_radiation_pressure(self):
    """Apply modulation to radiation pressure instead of amplitude

    """

    return RadiationPressure(self)


IModulation.with_radiation_pressure = __with_radiation_pressure  # type: ignore
