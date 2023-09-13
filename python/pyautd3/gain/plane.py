'''
File: plane.py
Project: gain
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 14/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


import numpy as np
from typing import Optional

from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import GainPtr
from pyautd3.geometry import Geometry
from ..internal.gain import IGain


class Plane(IGain):
    _d: np.ndarray
    _amp: Optional[float]

    def __init__(self, dir: np.ndarray):
        super().__init__()
        self._d = dir
        self._amp = None

    def gain_ptr(self, _: Geometry) -> GainPtr:
        ptr = Base().gain_plane(self._d[0], self._d[1], self._d[2])
        if self._amp is not None:
            ptr = Base().gain_plane_with_amp(ptr, self._amp)
        return ptr
