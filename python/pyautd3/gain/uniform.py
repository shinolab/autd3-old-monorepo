'''
File: uniform.py
Project: gain
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 14/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from typing import Optional

from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import GainPtr
from pyautd3.geometry import Geometry
from ..internal.gain import IGain


class Uniform(IGain):
    _amp: float
    _phase: Optional[float]

    def __init__(self, amp: float):
        super().__init__()
        self._amp = amp
        self._phase = None

    def with_phase(self, phase: float) -> "Uniform":
        self._phase = phase
        return self

    def gain_ptr(self, _: Geometry) -> GainPtr:
        ptr = Base().gain_uniform(self._amp)
        if self._phase is not None:
            ptr = Base().gain_uniform_with_phase(ptr, self._phase)
        return ptr
