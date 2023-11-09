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
from pyautd3.gain import Gain
from tests.test_autd import create_controller


class Uniform(Gain):
    _amp: float
    _phase: float
    check: np.ndarray

    def __init__(self: "Uniform", amp: float, phase: float, check: np.ndarray) -> None:
        self._amp = amp
        self._phase = phase
        self.check = check

    def calc(self: "Uniform", geometry: Geometry) -> dict[int, np.ndarray]:
        def f(dev: Device, _tr: Transducer) -> Drive:
            self.check[dev.idx] = True
            return Drive(
                self._phase,
                self._amp,
            )

        return Gain._transform(geometry, f)


@pytest.mark.asyncio()
async def test_gain():
    autd = await create_controller()

    check = np.zeros(autd.geometry.num_devices, dtype=bool)
    assert await autd.send(Uniform(0.5, np.pi, check))

    for dev in autd.geometry:
        duties, phases = autd.link.duties_and_phases(dev.idx, 0)
        assert np.all(duties == 85)
        assert np.all(phases == 256)


@pytest.mark.asyncio()
async def test_gain_check_only_for_enabled():
    autd = await create_controller()
    autd.geometry[0].enable = False

    check = np.zeros(autd.geometry.num_devices, dtype=bool)
    g = Uniform(0.5, np.pi, check)
    assert await autd.send(g)

    assert not g.check[0]
    assert g.check[1]

    duties, phases = autd.link.duties_and_phases(0, 0)
    assert np.all(duties == 0)
    assert np.all(phases == 0)

    duties, phases = autd.link.duties_and_phases(1, 0)
    assert np.all(duties == 85)
    assert np.all(phases == 256)
