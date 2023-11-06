"""
File: test_gain.py
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

from pyautd3 import Device, Drive, Geometry, Transducer
from pyautd3.gain import Gain, Uniform
from tests.test_autd import create_controller


def test_cache():
    autd = create_controller()

    assert autd.send(Uniform(0.5).with_phase(np.pi).with_cache())

    for dev in autd.geometry:
        duties, phases = autd.link.duties_and_phases(dev.idx, 0)
        assert np.all(duties == 85)
        assert np.all(phases == 256)


class CacheTest(Gain):
    calc_cnt: int

    def __init__(self: "CacheTest") -> None:
        self.calc_cnt = 0

    def calc(self: "CacheTest", geometry: Geometry) -> dict[int, np.ndarray]:
        self.calc_cnt += 1
        return Gain._transform(
            geometry,
            lambda _dev, _tr: Drive(
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

    duties, phases = autd.link.duties_and_phases(0, 0)
    assert np.all(duties == 0)
    assert np.all(phases == 0)

    duties, phases = autd.link.duties_and_phases(1, 0)
    assert np.all(duties == 85)
    assert np.all(phases == 256)


def test_transform():
    autd = create_controller()

    def transform(dev: Device, _tr: Transducer, d: Drive) -> Drive:
        if dev.idx == 0:
            return Drive(d.phase + np.pi / 4, d.amp)

        return Drive(d.phase - np.pi / 4, d.amp)

    assert autd.send(Uniform(0.5).with_phase(np.pi).with_transform(transform))

    duties, phases = autd.link.duties_and_phases(0, 0)
    assert np.all(duties == 85)
    assert np.all(phases == 256 + 64)

    duties, phases = autd.link.duties_and_phases(1, 0)
    assert np.all(duties == 85)
    assert np.all(phases == 256 - 64)


def test_transform_check_only_for_enabled():
    autd = create_controller()
    autd.geometry[0].enable = False

    check = np.zeros(2, dtype=bool)

    def transform(dev: Device, _tr: Transducer, d: Drive) -> Drive:
        check[dev.idx] = True
        return d

    assert autd.send(Uniform(0.5).with_phase(np.pi).with_transform(transform))

    assert not check[0]
    assert check[1]

    duties, phases = autd.link.duties_and_phases(0, 0)
    assert np.all(duties == 0)
    assert np.all(phases == 0)

    duties, phases = autd.link.duties_and_phases(1, 0)
    assert np.all(duties == 85)
    assert np.all(phases == 256)
