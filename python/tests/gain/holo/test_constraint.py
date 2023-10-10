'''
File: test_constraint.py
Project: holo
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from ...test_autd import create_controller

from pyautd3.gain.holo import NalgebraBackend, Naive, AmplitudeConstraint
from pyautd3.link.audit import Audit

import numpy as np


def test_constraint():
    autd = create_controller()

    backend = NalgebraBackend()
    g = Naive(backend)\
        .add_focus(autd.geometry.center + np.array([30, 0, 150]), 0.5)\
        .add_focus(autd.geometry.center + np.array([-30, 0, 150]), 0.5)\
        .with_constraint(AmplitudeConstraint.uniform(0.5))
    assert autd.send(g)
    for dev in autd.geometry:
        duties, phases = autd.link().duties_and_phases(dev.idx, 0)
        assert np.all(duties == 680)
        assert not np.all(phases == 0)

    g = Naive(backend)\
        .add_focus(autd.geometry.center + np.array([30, 0, 150]), 0.5)\
        .add_focus(autd.geometry.center + np.array([-30, 0, 150]), 0.5)\
        .with_constraint(AmplitudeConstraint.normalize())
    assert autd.send(g)
    for dev in autd.geometry:
        duties, phases = autd.link().duties_and_phases(dev.idx, 0)
        assert not np.all(duties == 0)
        assert not np.all(phases == 0)

    g = Naive(backend)\
        .add_focus(autd.geometry.center + np.array([30, 0, 150]), 0.5)\
        .add_focus(autd.geometry.center + np.array([-30, 0, 150]), 0.5)\
        .with_constraint(AmplitudeConstraint.clamp(0.4, 0.5))
    assert autd.send(g)
    for dev in autd.geometry:
        duties, phases = autd.link().duties_and_phases(dev.idx, 0)
        assert np.all(536 <= duties)
        assert np.all(duties <= 680)
        assert not np.all(phases == 0)

    g = Naive(backend)\
        .add_focus(autd.geometry.center + np.array([30, 0, 150]), 0.5)\
        .add_focus(autd.geometry.center + np.array([-30, 0, 150]), 0.5)\
        .with_constraint(AmplitudeConstraint.dont_care())
    assert autd.send(g)
    for dev in autd.geometry:
        duties, phases = autd.link().duties_and_phases(dev.idx, 0)
        assert not np.all(duties == 0)
        assert not np.all(phases == 0)
