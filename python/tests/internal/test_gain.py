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
import pytest

from pyautd3 import Device, Drive, Geometry, Transducer
from pyautd3.emit_intensity import EmitIntensity
from pyautd3.gain import Gain, Uniform
from pyautd3.phase import Phase
from tests.test_autd import create_controller


@pytest.mark.asyncio()
async def test_cache():
    autd = await create_controller()

    assert await autd.send_async(Uniform(0x80).with_phase(Phase(0x90)).with_cache())

    for dev in autd.geometry:
        intensities, phases = autd.link.intensities_and_phases(dev.idx, 0)
        assert np.all(intensities == 0x80)
        assert np.all(phases == 0x90)


class CacheTest(Gain):
    calc_cnt: int

    def __init__(self: "CacheTest") -> None:
        self.calc_cnt = 0

    def calc(self: "CacheTest", geometry: Geometry) -> dict[int, np.ndarray]:
        self.calc_cnt += 1
        return Gain._transform(
            geometry,
            lambda _dev, _tr: Drive(
                Phase(0x90),
                EmitIntensity(0x80),
            ),
        )


@pytest.mark.asyncio()
async def test_cache_check_once():
    autd = await create_controller()

    g = CacheTest()
    assert await autd.send_async(g)
    assert g.calc_cnt == 1
    assert await autd.send_async(g)
    assert g.calc_cnt == 2

    g = CacheTest()
    g_cached = g.with_cache()
    assert await autd.send_async(g_cached)
    assert g.calc_cnt == 1
    assert await autd.send_async(g_cached)
    assert g.calc_cnt == 1


@pytest.mark.asyncio()
async def test_cache_check_only_for_enabled():
    autd = await create_controller()
    autd.geometry[0].enable = False

    g = CacheTest()
    g_cached = g.with_cache()
    assert await autd.send_async(g_cached)

    assert 0 not in g_cached.drives()
    assert 1 in g_cached.drives()

    intensities, phases = autd.link.intensities_and_phases(0, 0)
    assert np.all(intensities == 0)
    assert np.all(phases == 0)

    intensities, phases = autd.link.intensities_and_phases(1, 0)
    assert np.all(intensities == 0x80)
    assert np.all(phases == 0x90)


@pytest.mark.asyncio()
async def test_transform():
    autd = await create_controller()

    def transform(dev: Device, _tr: Transducer, d: Drive) -> Drive:
        if dev.idx == 0:
            return Drive(Phase(d.phase.value + 32), d.intensity)

        return Drive(Phase(d.phase.value - 32), d.intensity)

    assert await autd.send_async(Uniform(0x80).with_phase(Phase(128)).with_transform(transform))

    intensities, phases = autd.link.intensities_and_phases(0, 0)
    assert np.all(intensities == 0x80)
    assert np.all(phases == 128 + 32)

    intensities, phases = autd.link.intensities_and_phases(1, 0)
    assert np.all(intensities == 0x80)
    assert np.all(phases == 128 - 32)


@pytest.mark.asyncio()
async def test_transform_check_only_for_enabled():
    autd = await create_controller()
    autd.geometry[0].enable = False

    check = np.zeros(2, dtype=bool)

    def transform(dev: Device, _tr: Transducer, d: Drive) -> Drive:
        check[dev.idx] = True
        return d

    assert await autd.send_async(Uniform(0x80).with_phase(Phase(0x90)).with_transform(transform))

    assert not check[0]
    assert check[1]

    intensities, phases = autd.link.intensities_and_phases(0, 0)
    assert np.all(intensities == 0)
    assert np.all(phases == 0)

    intensities, phases = autd.link.intensities_and_phases(1, 0)
    assert np.all(intensities == 0x80)
    assert np.all(phases == 0x90)
