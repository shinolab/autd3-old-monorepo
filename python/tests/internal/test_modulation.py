"""
File: test_modulation.py
Project: modulation
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

from pyautd3 import EmitIntensity, SamplingConfiguration
from pyautd3.modulation import Modulation, Sine
from tests.test_autd import create_controller


@pytest.mark.asyncio()
async def test_cache():
    autd1 = await create_controller()
    autd2 = await create_controller()

    m1 = Sine(150)
    m2 = Sine(150).with_cache()

    assert await autd1.send_async(m1)
    assert await autd2.send_async(m2)

    for dev in autd1.geometry:
        mod_expect = autd1.link.modulation(dev.idx)
        mod = autd2.link.modulation(dev.idx)
        assert np.array_equal(mod, mod_expect)
        assert autd2.link.modulation_frequency_division(dev.idx) == 5120

    mod_expect = autd1.link.modulation(0)
    buf = m2.buffer
    assert np.array_equal(buf, mod_expect)
    for i, m in enumerate(m2):
        assert m == mod_expect[i]
    for i in range(len(buf)):
        assert m2[i] == mod_expect[i]


class CacheTest(Modulation):
    calc_cnt: int

    def __init__(self: "CacheTest") -> None:
        super().__init__(SamplingConfiguration.from_frequency(4e3))
        self.calc_cnt = 0

    def calc(self: "CacheTest"):
        self.calc_cnt += 1
        return np.array([EmitIntensity(0xFF)] * 2)


@pytest.mark.asyncio()
async def test_cache_check_once():
    autd = await create_controller()

    m = CacheTest()
    assert await autd.send_async(m)
    assert m.calc_cnt == 1
    assert await autd.send_async(m)
    assert m.calc_cnt == 2

    m = CacheTest()
    m_cached = m.with_cache()

    assert await autd.send_async(m_cached)
    assert m.calc_cnt == 1
    assert await autd.send_async(m_cached)
    assert m.calc_cnt == 1


@pytest.mark.asyncio()
async def test_transform():
    autd1 = await create_controller()
    autd2 = await create_controller()

    m1 = Sine(150)
    m2 = Sine(150).with_transform(lambda _i, v: EmitIntensity(v.value // 2))

    assert await autd1.send_async(m1)
    assert await autd2.send_async(m2)

    for dev in autd1.geometry:
        mod_expect = autd1.link.modulation(dev.idx)
        mod = autd2.link.modulation(dev.idx)
        for i in range(len(mod_expect)):
            assert mod[i] == mod_expect[i] // 2
        assert autd2.link.modulation_frequency_division(dev.idx) == 5120


@pytest.mark.asyncio()
async def test_radiation_pressure():
    autd = await create_controller()

    m = Sine(150).with_radiation_pressure()

    assert await autd.send_async(m)

    for dev in autd.geometry:
        mod = autd.link.modulation(dev.idx)
        mod_expect = [
            180,
            200,
            217,
            231,
            242,
            249,
            253,
            254,
            251,
            245,
            235,
            222,
            206,
            187,
            165,
            141,
            115,
            87,
            60,
            32,
            0,
            32,
            60,
            87,
            115,
            141,
            165,
            187,
            206,
            222,
            235,
            245,
            251,
            254,
            253,
            249,
            242,
            231,
            217,
            200,
            180,
            157,
            133,
            107,
            78,
            50,
            23,
            0,
            39,
            70,
            97,
            125,
            150,
            173,
            194,
            212,
            227,
            239,
            247,
            252,
            254,
            252,
            247,
            239,
            227,
            212,
            194,
            173,
            150,
            125,
            97,
            70,
            39,
            0,
            23,
            50,
            78,
            107,
            133,
            157,
        ]
        assert np.array_equal(mod, mod_expect)
        assert autd.link.modulation_frequency_division(dev.idx) == 5120
