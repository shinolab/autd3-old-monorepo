"""
File: holo.py
Project: holo
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""

import numpy as np
from pyautd3.native_methods.autd3capi_gain_holo import NativeMethods as GainHolo
from pyautd3.gain.gain import Gain
from .constraint import AmplitudeConstraint


class Holo(Gain):
    def __init__(self):
        super().__init__()

    def __del__(self):
        super().__del__()

    def add(self, focus: np.ndarray, amp: float):
        GainHolo().gain_holo_add(self.ptr, focus[0], focus[1], focus[2], amp)

    def constraint(self, constraint: AmplitudeConstraint):
        constraint.set(self)
