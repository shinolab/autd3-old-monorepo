"""
File: test_transtest.py
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

from pyautd3.gain import TransducerTest
from tests.test_autd import create_controller


@pytest.mark.asyncio()
async def test_transtest():
    autd = await create_controller()

    assert await autd.send_async(TransducerTest().set_drive(autd.geometry[0][0], np.pi, 0x80).set_drive(autd.geometry[1][248], np.pi, 0x81))

    intensities, phases = autd.link.intensities_and_phases(0, 0)
    assert intensities[0] == 0x80
    assert phases[0] == 128
    assert np.all(intensities[1:-1] == 0)
    assert np.all(phases[1:-1] == 0)

    intensities, phases = autd.link.intensities_and_phases(1, 0)
    assert intensities[-1] == 0x81
    assert phases[-1] == 128
    assert np.all(intensities[:-1] == 0)
    assert np.all(phases[:-1] == 0)
