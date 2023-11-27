"""
File: test_emit_intensity.py
Project: tests
Created Date: 25/11/2023
Author: Shun Suzuki
-----
Last Modified: 25/11/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


import numpy as np

from pyautd3 import EmitIntensity
from pyautd3.native_methods.autd3capi_def import DEFAULT_CORRECTED_ALPHA


def test_emit_intensity():
    for i in range(256):
        intensity = EmitIntensity(i)
        assert intensity.value == i


def test_emit_intensity_with_correction():
    for i in range(256):
        intensity = EmitIntensity.new_with_correction(i)
        assert intensity.value == int(np.round(np.arcsin(pow(i / 255, 1 / DEFAULT_CORRECTED_ALPHA)) / np.pi * 510))
