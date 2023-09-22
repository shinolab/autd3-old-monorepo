'''
File: test_gain.py
Project: gain
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 21/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from typing import Dict
from ..test_autd import create_controller

from pyautd3 import Drive, Geometry
from pyautd3.gain import Uniform, Gain
from pyautd3.link.audit import Audit

import numpy as np


def test_cache():
    autd = create_controller()

    assert autd.send(Uniform(0.5).with_phase(np.pi).with_cache())

    for dev in autd.geometry:
        duties, phases = Audit.duties_and_phases(autd._ptr, dev.idx, 0)
        assert np.all(duties == 680)
        assert np.all(phases == 2048)


class CacheTest(Gain):
    calc_cnt: int

    def __init__(self):
        self.calc_cnt = 0

    def calc(self, geometry: Geometry) -> Dict[int, np.ndarray]:
        self.calc_cnt += 1
        return Gain.transform(
            geometry,
            lambda dev, tr: Drive(
                0.0,
                1.0,
            ),
        )


def test_cache_check_once():
    autd = create_controller()

    g = CacheTest()
    assert autd.send(g)
    assert g.calc_cnt == 1
    assert autd.send(g)
    assert g.calc_cnt == 2

    g = CacheTest()
    g_cached = g.with_cache()
    assert autd.send(g_cached)
    assert g.calc_cnt == 1
    assert autd.send(g_cached)
    assert g.calc_cnt == 1


def test_transform():
    autd = create_controller()

    def transform(dev, tr, d) -> Drive:
        if dev.idx == 0:
            return Drive(d.phase + np.pi / 4, d.amp)
        else:
            return Drive(d.phase - np.pi / 4, d.amp)

    assert autd.send(Uniform(0.5).with_phase(np.pi).with_transform(transform))

    duties, phases = Audit.duties_and_phases(autd._ptr, 0, 0)
    assert np.all(duties == 680)
    assert np.all(phases == 2048 + 512)

    duties, phases = Audit.duties_and_phases(autd._ptr, 1, 0)
    assert np.all(duties == 680)
    assert np.all(phases == 2048 - 512)
