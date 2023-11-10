"""
File: test_sine_legacy.py
Project: modulation
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 11/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from datetime import timedelta

import numpy as np
import pytest

from pyautd3.modulation import SineLegacy
from tests.test_autd import create_controller


@pytest.mark.asyncio()
async def test_sine_legacy():
    autd = await create_controller()

    assert await autd.send_async(SineLegacy(150.0).with_amp(0.5).with_offset(0.25))

    for dev in autd.geometry:
        mod = autd.link.modulation(dev.idx)
        mod_expext = [41, 50, 60, 68, 75, 81, 84, 84, 83, 78, 72, 64, 55, 45, 36, 26, 18, 11, 5, 1, 0, 0, 3, 8, 14, 22, 0]
        assert np.array_equal(mod, mod_expext)
        assert autd.link.modulation_frequency_division(dev.idx) == 5120

    assert await autd.send_async(SineLegacy(150).with_sampling_frequency_division(512))
    for dev in autd.geometry:
        assert autd.link.modulation_frequency_division(dev.idx) == 512

    assert await autd.send_async(SineLegacy(150).with_sampling_frequency(8e3))
    for dev in autd.geometry:
        assert autd.link.modulation_frequency_division(dev.idx) == 2560

    assert await autd.send_async(SineLegacy(150).with_sampling_period(timedelta(microseconds=100)))
    for dev in autd.geometry:
        assert autd.link.modulation_frequency_division(dev.idx) == 2048
