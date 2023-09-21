'''
File: test_lm.py
Project: holo
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 20/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


import pytest

from pyautd3 import AUTD3, Controller
from pyautd3.gain.holo import NalgebraBackend, LM, AmplitudeConstraint
from pyautd3.gain.holo.backend_cuda import CUDABackend
from pyautd3.link.audit import Audit

import numpy as np


def test_lm():
    autd = Controller.builder()\
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))\
        .open_with(Audit())

    backend = NalgebraBackend()
    g = LM(backend)\
        .add_focus(autd.geometry.center + np.array([30, 0, 150]), 0.5)\
        .add_foci_from_iter(map(lambda x: (autd.geometry.center + np.array([0, x, 150]), 0.5), [-30]))\
        .with_eps1(1e-3)\
        .with_eps2(1e-3)\
        .with_tau(1e-3)\
        .with_kmax(5)\
        .with_initial(np.zeros(1))\
        .with_constraint(AmplitudeConstraint.uniform(0.5))
    assert autd.send(g)

    for dev in autd.geometry:
        duties, phases = Audit.duties_and_phases(autd._ptr, dev.idx, 0)
        assert np.all(duties == 680)
        assert not np.all(phases == 0)


@pytest.mark.cuda
def test_lm_cuda():
    autd = Controller.builder()\
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))\
        .open_with(Audit())

    backend = CUDABackend()
    g = LM(backend)\
        .add_focus(autd.geometry.center + np.array([30, 0, 150]), 0.5)\
        .add_foci_from_iter(map(lambda x: (autd.geometry.center + np.array([0, x, 150]), 0.5), [-30]))\
        .with_eps1(1e-3)\
        .with_eps2(1e-3)\
        .with_tau(1e-3)\
        .with_kmax(5)\
        .with_initial(np.zeros(1))\
        .with_constraint(AmplitudeConstraint.uniform(0.5))
    assert autd.send(g)

    for dev in autd.geometry:
        duties, phases = Audit.duties_and_phases(autd._ptr, dev.idx, 0)
        assert np.all(duties == 680)
        assert not np.all(phases == 0)
