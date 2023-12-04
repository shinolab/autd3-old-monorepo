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
from numpy.typing import ArrayLike

from pyautd3 import Device, Drive, Geometry, Transducer
from pyautd3.emit_intensity import EmitIntensity
from pyautd3.gain import Gain
from pyautd3.phase import Phase
from tests.test_autd import create_controller


class Uniform(Gain):
    _intensity: EmitIntensity
    _phase: Phase
    check: np.ndarray

    def __init__(self: "Uniform", intensity: int, phase: Phase, check: ArrayLike) -> None:
        self._intensity = EmitIntensity(intensity)
        self._phase = phase
        self.check = np.array(check)

    def calc(self: "Uniform", geometry: Geometry) -> dict[int, np.ndarray]:
        def f(dev: Device, _tr: Transducer) -> Drive:
            self.check[dev.idx] = True
            return Drive(
                self._phase,
                self._intensity,
            )

        return Gain._transform(geometry, f)


@pytest.mark.asyncio()
async def test_gain():
    autd = await create_controller()

    check = np.zeros(autd.geometry.num_devices, dtype=bool)
    assert await autd.send_async(Uniform(0x80, Phase(0x90), check))

    for dev in autd.geometry:
        intensities, phases = autd.link.intensities_and_phases(dev.idx, 0)
        assert np.all(intensities == 0x80)
        assert np.all(phases == 0x90)


@pytest.mark.asyncio()
async def test_gain_check_only_for_enabled():
    autd = await create_controller()
    autd.geometry[0].enable = False

    check = np.zeros(autd.geometry.num_devices, dtype=bool)
    g = Uniform(0x80, Phase(0x90), check)
    assert await autd.send_async(g)

    assert not g.check[0]
    assert g.check[1]

    intensities, phases = autd.link.intensities_and_phases(0, 0)
    assert np.all(intensities == 0)
    assert np.all(phases == 0)

    intensities, phases = autd.link.intensities_and_phases(1, 0)
    assert np.all(intensities == 0x80)
    assert np.all(phases == 0x90)
