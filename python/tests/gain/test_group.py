'''
File: test_group.py
Project: gain
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 20/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from ..test_autd import create_controller

from pyautd3.gain import Null, Uniform, Group
from pyautd3.link.audit import Audit

import numpy as np


def test_group():
    autd = create_controller()

    cx = autd.geometry.center[0]

    assert autd.send(
        Group(
            lambda _,
            tr: "uniform" if tr.position[0] < cx else "null").set(
            "uniform",
            Uniform(0.5).with_phase(
                np.pi)).set(
                    "null",
            Null()))

    for dev in autd.geometry:
        duties, phases = Audit.duties_and_phases(autd._ptr, dev.idx, 0)
        for tr in dev:
            if tr.position[0] < cx:
                assert np.all(duties[tr.local_idx] == 680)
                assert np.all(phases[tr.local_idx] == 2048)
            else:
                assert np.all(duties[tr.local_idx] == 8)
                assert np.all(phases[tr.local_idx] == 0)
