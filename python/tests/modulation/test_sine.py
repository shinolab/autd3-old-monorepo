"""
File: test_sine.py
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
from pyautd3.modulation import Sine
from tests.test_autd import create_controller


@pytest.mark.asyncio()
async def test_sine():
    autd = await create_controller()

    assert await autd.send_async(Sine(150).with_amp(0.5).with_offset(0.25).with_phase(np.pi / 2))

    for dev in autd.geometry:
        mod = autd.link.modulation(dev.idx)
        mod_expect = [
            128,
            126,
            121,
            112,
            101,
            88,
            74,
            59,
            44,
            30,
            19,
            9,
            3,
            0,
            1,
            5,
            12,
            22,
            35,
            49,
            64,
            79,
            93,
            105,
            115,
            123,
            127,
            127,
            124,
            118,
            109,
            97,
            83,
            69,
            54,
            39,
            26,
            15,
            7,
            2,
            0,
            2,
            7,
            15,
            26,
            39,
            54,
            69,
            83,
            97,
            109,
            118,
            124,
            127,
            127,
            123,
            115,
            105,
            93,
            79,
            64,
            49,
            35,
            22,
            12,
            5,
            1,
            0,
            3,
            9,
            19,
            30,
            44,
            59,
            74,
            88,
            101,
            112,
            121,
            126,
        ]
        assert np.array_equal(mod, mod_expect)
        assert autd.link.modulation_frequency_division(dev.idx) == 5120

    assert await autd.send_async(
        Sine(150).with_sampling_config(
            SamplingConfiguration.new_with_frequency_division(512),
        ),
    )
    for dev in autd.geometry:
        assert autd.link.modulation_frequency_division(dev.idx) == 512
