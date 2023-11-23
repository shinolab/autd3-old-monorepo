"""
File: test_fourier.py
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

from pyautd3.modulation import Fourier, Sine
from tests.test_autd import create_controller


@pytest.mark.asyncio()
async def test_fourier():
    autd = await create_controller()

    m = Fourier(Sine(50)).add_components_from_iter(Sine(x) for x in [100, 150]) + Sine(200)
    assert await autd.send_async(m)

    for dev in autd.geometry:
        mod = autd.link.modulation(dev.idx)
        mod_expect = [
            128,
            152,
            175,
            195,
            212,
            223,
            229,
            230,
            225,
            216,
            204,
            189,
            175,
            159,
            146,
            135,
            127,
            122,
            121,
            123,
            127,
            133,
            139,
            145,
            151,
            154,
            155,
            154,
            151,
            146,
            141,
            134,
            127,
            121,
            117,
            114,
            114,
            115,
            118,
            122,
            128,
            132,
            136,
            139,
            140,
            140,
            138,
            133,
            127,
            121,
            114,
            108,
            103,
            100,
            99,
            101,
            104,
            109,
            115,
            121,
            127,
            131,
            133,
            132,
            127,
            119,
            108,
            95,
            80,
            65,
            50,
            38,
            29,
            24,
            25,
            31,
            42,
            59,
            79,
            102,
        ]
        assert np.array_equal(mod, mod_expect)
        assert autd.link.modulation_frequency_division(dev.idx) == 5120
