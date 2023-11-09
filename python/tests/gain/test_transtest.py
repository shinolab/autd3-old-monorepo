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

    assert await autd.send(TransducerTest().set_drive(0, 0, np.pi, 0.5).set_drive(1, 248, np.pi, 0.5))

    duties, phases = autd.link.duties_and_phases(0, 0)
    assert duties[0] == 85
    assert phases[0] == 256
    assert np.all(duties[1:-1] == 0)
    assert np.all(phases[1:-1] == 0)

    duties, phases = autd.link.duties_and_phases(1, 0)
    assert duties[-1] == 85
    assert phases[-1] == 256
    assert np.all(duties[:-1] == 0)
    assert np.all(phases[:-1] == 0)
