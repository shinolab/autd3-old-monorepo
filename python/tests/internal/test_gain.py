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

from pyautd3 import Drive, Geometry
from pyautd3.gain import Uniform, Gain
from pyautd3.link.audit import Audit

import numpy as np


def test_cache():
    autd = create_controller()

    assert autd.send(Uniform(0.5).with_phase(np.pi).with_cache())

    for dev in autd.geometry:
        duties, phases = autd.link().duties_and_phases(dev.idx, 0)
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
                np.pi,
                0.5,
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


def test_cache_check_only_for_enabled():
    autd = create_controller()
    autd.geometry[0].enable = False

    g = CacheTest()
    g_cached = g.with_cache()
    assert autd.send(g_cached)

    assert 0 not in g_cached.drives()
    assert 1 in g_cached.drives()

    duties, phases = autd.link().duties_and_phases(0, 0)
    assert np.all(duties == 0)
    assert np.all(phases == 0)

    duties, phases = autd.link().duties_and_phases(1, 0)
    assert np.all(duties == 680)
    assert np.all(phases == 2048)


def test_transform():
    autd = create_controller()

    def transform(dev, tr, d) -> Drive:
        if dev.idx == 0:
            return Drive(d.phase + np.pi / 4, d.amp)
        else:
            return Drive(d.phase - np.pi / 4, d.amp)

    assert autd.send(Uniform(0.5).with_phase(np.pi).with_transform(transform))

    duties, phases = autd.link().duties_and_phases(0, 0)
    assert np.all(duties == 680)
    assert np.all(phases == 2048 + 512)

    duties, phases = autd.link().duties_and_phases(1, 0)
    assert np.all(duties == 680)
    assert np.all(phases == 2048 - 512)


def test_transform_check_only_for_enabled():
    autd = create_controller()
    autd.geometry[0].enable = False

    check = np.zeros(2, dtype=bool)

    def transform(dev, tr, d) -> Drive:
        check[dev.idx] = True
        return d

    assert autd.send(Uniform(0.5).with_phase(np.pi).with_transform(transform))

    assert not check[0]
    assert check[1]

    duties, phases = autd.link().duties_and_phases(0, 0)
    assert np.all(duties == 0)
    assert np.all(phases == 0)

    duties, phases = autd.link().duties_and_phases(1, 0)
    assert np.all(duties == 680)
    assert np.all(phases == 2048)
