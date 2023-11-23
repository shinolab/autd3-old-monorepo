"""
File: test_static.py
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

from pyautd3.modulation import Static
from tests.test_autd import create_controller


@pytest.mark.asyncio()
async def test_static():
    autd = await create_controller()

    assert await autd.send_async(Static().with_intensity(0x80))

    for dev in autd.geometry:
        mod = autd.link.modulation(dev.idx)
        mod_expect = [0x80] * 2
        assert np.array_equal(mod, mod_expect)
        assert autd.link.modulation_frequency_division(dev.idx) == 5120
