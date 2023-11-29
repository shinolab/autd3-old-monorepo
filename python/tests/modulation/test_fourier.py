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
            127,
            156,
            183,
            205,
            220,
            227,
            226,
            218,
            205,
            188,
            170,
            153,
            139,
            129,
            124,
            123,
            127,
            133,
            140,
            147,
            152,
            155,
            154,
            151,
            145,
            138,
            131,
            125,
            120,
            119,
            119,
            122,
            127,
            132,
            136,
            140,
            141,
            140,
            137,
            133,
            127,
            121,
            116,
            113,
            112,
            113,
            117,
            121,
            127,
            131,
            134,
            135,
            133,
            128,
            122,
            115,
            108,
            103,
            99,
            99,
            101,
            106,
            113,
            121,
            127,
            130,
            130,
            124,
            114,
            100,
            83,
            66,
            48,
            35,
            27,
            27,
            34,
            49,
            70,
            97,
        ]
        assert np.array_equal(mod, mod_expect)
        assert autd.link.modulation_frequency_division(dev.idx) == 5120
