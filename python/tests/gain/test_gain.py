'''
File: test_gain.py
Project: gain
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 20/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from typing import Dict

from ..test_autd import create_controller

from pyautd3 import Geometry, Drive
from pyautd3.gain import Gain
from pyautd3.link.audit import Audit

import numpy as np


class Uniform(Gain):
    _amp: float
    _phase: float

    def __init__(self, amp, phase):
        self._amp = amp
        self._phase = phase

    def calc(self, geometry: Geometry) -> Dict[int, np.ndarray]:
        return Gain.transform(
            geometry,
            lambda dev, tr: Drive(
                self._phase, self._amp
            ),
        )


def test_gain():
    autd = create_controller()

    assert autd.send(Uniform(0.5, np.pi))

    for dev in autd.geometry:
        duties, phases = Audit.duties_and_phases(autd._ptr, dev.idx, 0)
        assert np.all(duties == 680)
        assert np.all(phases == 2048)
