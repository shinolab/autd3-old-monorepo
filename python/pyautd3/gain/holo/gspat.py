"""
File: gspat.py
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
from pyautd3.gain.gain import Gain
from .backend import Backend
from .constraint import AmplitudeConstraint, DontCare, Normalize, Uniform, Clamp

from pyautd3.native_methods.autd3capi_gain_holo import NativeMethods as GainHolo


class GSPAT(Gain):
    def __init__(self, backend: Backend):
        super().__init__()
        self.ptr = GainHolo().gain_holo_gspat(backend.ptr)

    def repeat(self, value: int):
        GainHolo().gain_holo_gspat_repeat(self.ptr, value)

    def __del__(self):
        super().__del__()

    def add(self, focus: np.ndarray, amp: float):
        GainHolo().gain_holo_gspat_add(self.ptr, focus[0], focus[1], focus[2], amp)

    def constraint(self, constraint: AmplitudeConstraint):
        if isinstance(constraint, DontCare):
            GainHolo().gain_holo_gspat_set_dot_care_constraint(self.ptr)
        elif isinstance(constraint, Normalize):
            GainHolo().gain_holo_gspat_set_normalize_constraint(self.ptr)
        elif isinstance(constraint, Uniform):
            GainHolo().gain_holo_gspat_set_uniform_constraint(self.ptr, constraint.value)
        elif isinstance(constraint, Clamp):
            GainHolo().gain_holo_gspat_set_clamp_constraint(
                self.ptr, constraint.min, constraint.max
            )
        else:
            raise ValueError("constraint must be DontCare, Normalize, Uniform or Clamp")
