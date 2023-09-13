'''
File: static.py
Project: modulation
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
from pyautd3.native_methods.autd3capi_def import ModulationPtr
from pyautd3.internal.modulation import IModulation


class Static(IModulation):
    _amp: Optional[float]

    def __init__(self):
        super().__init__()
        self._amp = None

    def with_amp(self, amp: float) -> "Static":
        self._amp = amp
        return self

    def modulation_ptr(self) -> ModulationPtr:
        ptr = Base().modulation_static()
        if self._amp is not None:
            ptr = Base().modulation_static_with_amp(ptr, self._amp)
        return ptr
