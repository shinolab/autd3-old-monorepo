'''
File: test_transtest.py
Project: gain
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 25/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from ..test_autd import create_controller

from pyautd3.gain import TransducerTest
from pyautd3.link.audit import Audit

import numpy as np


def test_transtest():
    autd = create_controller()

    assert autd.send(TransducerTest().set(0, 0, np.pi, 0.5).set(1, 248, np.pi, 0.5))

    duties, phases = Audit.duties_and_phases(autd._ptr, 0, 0)
    assert duties[0] == 680
    assert phases[0] == 2048
    assert np.all(duties[1:-1] == 8)
    assert np.all(phases[1:-1] == 0)

    duties, phases = Audit.duties_and_phases(autd._ptr, 1, 0)
    assert duties[-1] == 680
    assert phases[-1] == 2048
    assert np.all(duties[:-1] == 8)
    assert np.all(phases[:-1] == 0)
