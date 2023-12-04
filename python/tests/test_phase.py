"""
File: test_phase.py
Project: tests
Created Date: 04/12/2023
Author: Shun Suzuki
-----
Last Modified: 04/12/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


import numpy as np
import pytest

from pyautd3.phase import Phase, rad


def test_phase():
    for i in range(256):
        phase = Phase(i)
        assert phase.value == i

    phase = Phase.from_rad(0)
    assert phase.radian == 0
    phase = Phase.from_rad(np.pi)
    assert phase.radian == np.pi
    phase = Phase.from_rad(2 * np.pi)
    assert phase.radian == 0

    phase = 0 * rad
    assert phase.radian == 0
    phase = np.pi * rad
    assert phase.radian == np.pi
    phase = 2 * np.pi * rad
    assert phase.radian == 0


def test_phase_ctr():
    with pytest.raises(NotImplementedError):
        _ = Phase._UnitRad()
