"""
File: test_modulation.py
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

from pyautd3 import EmitIntensity, SamplingConfiguration
from pyautd3.modulation import Modulation
from tests.test_autd import create_controller


class Burst(Modulation):
    def __init__(self: "Burst") -> None:
        super().__init__(SamplingConfiguration.from_frequency(4e3))

    def calc(self: "Burst"):
        buf = np.array([EmitIntensity(0)] * 10)
        buf[0] = EmitIntensity(0xFF)
        return buf


@pytest.mark.asyncio()
async def test_modulation():
    autd = await create_controller()

    m = Burst()

    assert m.sampling_config.frequency == 4e3
    assert len(m) == 10

    assert await autd.send_async(m)

    for dev in autd.geometry:
        mod = autd.link.modulation(dev.idx)
        assert len(mod) == 10
        assert mod[0] == 0xFF
        assert np.all(mod[1:] == 0)
        assert autd.link.modulation_frequency_division(dev.idx) == 5120
