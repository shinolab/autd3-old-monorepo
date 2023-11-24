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

    m = Fourier(Sine(50)).add_components_from_iter(Sine(x) for x in [100, 150]) + Sine(200) + Sine(250)
    assert await autd.send_async(m)

    for dev in autd.geometry:
        mod = autd.link.modulation(dev.idx)
        mod_expect = [
            128,
            157,
            183,
            205,
            220,
            227,
            227,
            219,
            206,
            189,
            171,
            153,
            140,
            129,
            124,
            124,
            127,
            133,
            141,
            147,
            153,
            155,
            155,
            151,
            146,
            139,
            131,
            125,
            121,
            119,
            120,
            123,
            127,
            132,
            137,
            140,
            142,
            141,
            138,
            133,
            128,
            121,
            116,
            113,
            112,
            114,
            117,
            122,
            127,
            132,
            135,
            135,
            133,
            129,
            123,
            116,
            108,
            103,
            100,
            99,
            102,
            107,
            114,
            121,
            127,
            130,
            130,
            125,
            115,
            101,
            84,
            66,
            49,
            35,
            27,
            27,
            34,
            49,
            71,
            98,
        ]
        assert np.array_equal(mod, mod_expect)
        assert autd.link.modulation_frequency_division(dev.idx) == 5120
