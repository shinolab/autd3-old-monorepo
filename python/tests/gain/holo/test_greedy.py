'''
File: test_greedy.py
Project: holo
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 20/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from ...test_autd import create_controller

from pyautd3.gain.holo import Greedy, AmplitudeConstraint
from pyautd3.link.audit import Audit

import numpy as np


def test_greedy():
    autd = create_controller()

    g = Greedy()\
        .add_focus(autd.geometry.center + np.array([30, 0, 150]), 0.5)\
        .add_focus(autd.geometry.center + np.array([-30, 0, 150]), 0.5)\
        .add_foci_from_iter(map(lambda x: (autd.geometry.center + np.array([0, x, 150]), 0.5), [-30, 30]))\
        .with_phase_div(16)\
        .with_constraint(AmplitudeConstraint.uniform(0.5))
    assert autd.send(g)

    for dev in autd.geometry:
        duties, phases = Audit.duties_and_phases(autd._ptr, dev.idx, 0)
        assert np.all(duties == 680)
        assert not np.all(phases == 0)
