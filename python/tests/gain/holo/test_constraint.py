"""
File: test_constraint.py
Project: holo
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

from pyautd3.gain.holo import AmplitudeConstraint, Naive, NalgebraBackend
from tests.test_autd import create_controller


@pytest.mark.asyncio()
async def test_constraint():
    autd = await create_controller()

    backend = NalgebraBackend()
    g = (
        Naive(backend)
        .add_focus(autd.geometry.center + np.array([30, 0, 150]), 0.5)
        .add_focus(autd.geometry.center + np.array([-30, 0, 150]), 0.5)
        .with_constraint(AmplitudeConstraint.uniform(0.5))
    )
    assert await autd.send_async(g)
    for dev in autd.geometry:
        duties, phases = autd.link.duties_and_phases(dev.idx, 0)
        assert np.all(duties == 85)
        assert not np.all(phases == 0)

    g = (
        Naive(backend)
        .add_focus(autd.geometry.center + np.array([30, 0, 150]), 0.5)
        .add_focus(autd.geometry.center + np.array([-30, 0, 150]), 0.5)
        .with_constraint(AmplitudeConstraint.normalize())
    )
    assert await autd.send_async(g)
    for dev in autd.geometry:
        duties, phases = autd.link.duties_and_phases(dev.idx, 0)
        assert not np.all(duties == 0)
        assert not np.all(phases == 0)

    g = (
        Naive(backend)
        .add_focus(autd.geometry.center + np.array([30, 0, 150]), 0.5)
        .add_focus(autd.geometry.center + np.array([-30, 0, 150]), 0.5)
        .with_constraint(AmplitudeConstraint.clamp(0.4, 0.5))
    )
    assert await autd.send_async(g)
    for dev in autd.geometry:
        duties, phases = autd.link.duties_and_phases(dev.idx, 0)
        assert np.all(duties >= 67)
        assert np.all(duties <= 85)
        assert not np.all(phases == 0)

    g = (
        Naive(backend)
        .add_focus(autd.geometry.center + np.array([30, 0, 150]), 5)
        .add_focus(autd.geometry.center + np.array([-30, 0, 150]), 5)
        .with_constraint(AmplitudeConstraint.dont_care())
    )
    assert await autd.send_async(g)
    for dev in autd.geometry:
        duties, phases = autd.link.duties_and_phases(dev.idx, 0)
        assert not np.all(duties == 0)
        assert not np.all(phases == 0)
