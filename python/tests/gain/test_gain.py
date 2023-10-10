'''
File: test_gain.py
Project: gain
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
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
    check: np.ndarray

    def __init__(self, amp, phase, check):
        self._amp = amp
        self._phase = phase
        self.check = check

    def calc(self, geometry: Geometry) -> Dict[int, np.ndarray]:
        def f(dev, tr):
            self.check[dev.idx] = True
            return Drive(
                self._phase,
                self._amp,
            )
        return Gain.transform(geometry, f)


def test_gain():
    autd = create_controller()

    check = np.zeros(autd.geometry.num_devices, dtype=bool)
    assert autd.send(Uniform(0.5, np.pi, check))

    for dev in autd.geometry:
        duties, phases = autd.link().duties_and_phases(dev.idx, 0)
        assert np.all(duties == 680)
        assert np.all(phases == 2048)


def test_gain_check_only_for_enabled():
    autd = create_controller()
    autd.geometry[0].enable = False

    check = np.zeros(autd.geometry.num_devices, dtype=bool)
    g = Uniform(0.5, np.pi, check)
    assert autd.send(g)

    assert not g.check[0]
    assert g.check[1]

    duties, phases = autd.link().duties_and_phases(0, 0)
    assert np.all(duties == 0)
    assert np.all(phases == 0)

    duties, phases = autd.link().duties_and_phases(1, 0)
    assert np.all(duties == 680)
    assert np.all(phases == 2048)
