"""
File: test_square.py
Project: modulation
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 11/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


import numpy as np
import pytest

from pyautd3 import SamplingConfiguration
from pyautd3.autd_error import AUTDError
from pyautd3.modulation import SamplingMode, Square
from tests.test_autd import create_controller


@pytest.mark.asyncio()
async def test_square():
    autd = await create_controller()

    assert await autd.send_async(Square(200).with_low(32).with_high(85).with_duty(0.1))

    for dev in autd.geometry:
        mod = autd.link.modulation(dev.idx)
        mod_expect = [85, 85, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32]
        assert np.array_equal(mod, mod_expect)
        assert autd.link.modulation_frequency_division(dev.idx) == 5120

    assert await autd.send_async(
        Square(150).with_sampling_config(
            SamplingConfiguration.from_frequency_division(512),
        ),
    )
    for dev in autd.geometry:
        assert autd.link.modulation_frequency_division(dev.idx) == 512


@pytest.mark.asyncio()
async def test_square_mode():
    autd = await create_controller()

    assert await autd.send_async(Square(150).with_mode(SamplingMode.SizeOptimized))
    for dev in autd.geometry:
        mod = autd.link.modulation(dev.idx)
        mod_expect = [255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        assert np.array_equal(mod, mod_expect)

    with pytest.raises(AUTDError):
        await autd.send_async(Square(100.1).with_mode(SamplingMode.ExactFrequency))

    assert await autd.send_async(Square(100.1).with_mode(SamplingMode.SizeOptimized))
