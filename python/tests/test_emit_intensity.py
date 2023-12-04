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
import pytest

from pyautd3 import EmitIntensity
from pyautd3.native_methods.autd3capi_def import DEFAULT_CORRECTED_ALPHA


def test_emit_intensity():
    for i in range(256):
        intensity = EmitIntensity(i)
        assert intensity.value == i


def test_emit_intensity_with_correction():
    for i in range(256):
        intensity = EmitIntensity.with_correction(i)
        assert intensity.value == int(np.round(np.arcsin(pow(i / 255, 1 / DEFAULT_CORRECTED_ALPHA)) / np.pi * 510))


def test_emit_intensity_min_max():
    assert EmitIntensity.minimum().value == 0x00
    assert EmitIntensity.maximum().value == 0xFF


def test_emit_intensity_cast():
    intensity = EmitIntensity._cast(0x80)
    assert intensity.value == 0x80

    intensity = EmitIntensity._cast(intensity)
    assert intensity.value == 0x80

    with pytest.raises(TypeError) as e:
        intensity = EmitIntensity._cast(0.0)
    assert str(e.value) == "Invalid type: <class 'float'>"
