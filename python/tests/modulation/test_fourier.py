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
        mod_expext = [
            85,
            103,
            123,
            142,
            159,
            173,
            181,
            182,
            176,
            164,
            151,
            136,
            122,
            109,
            99,
            90,
            85,
            81,
            80,
            82,
            85,
            89,
            93,
            98,
            102,
            105,
            106,
            105,
            103,
            99,
            94,
            89,
            85,
            80,
            77,
            75,
            75,
            76,
            78,
            81,
            85,
            88,
            91,
            94,
            95,
            94,
            92,
            89,
            85,
            80,
            75,
            71,
            67,
            65,
            65,
            66,
            68,
            72,
            76,
            80,
            85,
            88,
            89,
            88,
            85,
            79,
            71,
            62,
            51,
            41,
            32,
            24,
            18,
            15,
            16,
            20,
            27,
            37,
            51,
            67,
        ]
        assert np.array_equal(mod, mod_expext)
        assert autd.link.modulation_frequency_division(dev.idx) == 5120
