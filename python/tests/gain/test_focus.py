"""
File: test_focus.py
Project: gain
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

from pyautd3.gain import Focus
from tests.test_autd import create_controller


@pytest.mark.asyncio()
async def test_focus():
    autd = await create_controller()

    assert await autd.send_async(Focus(autd.geometry.center))
    for dev in autd.geometry:
        intensities, phases = autd.link.intensities_and_phases(dev.idx, 0)
        assert np.all(intensities == 0xFF)
        assert not np.all(phases == 0)

    assert await autd.send_async(Focus(autd.geometry.center).with_intensity(0x80))
    for dev in autd.geometry:
        intensities, phases = autd.link.intensities_and_phases(dev.idx, 0)
        assert np.all(intensities == 0x80)
        assert not np.all(phases == 0)
