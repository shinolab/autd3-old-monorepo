'''
File: test_gain.py
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

from pyautd3 import Drive
from pyautd3.gain import Uniform
from pyautd3.link.audit import Audit

import numpy as np


def test_cache():
    autd = create_controller()

    assert autd.send(Uniform(0.5).with_phase(np.pi).with_cache())

    for dev in autd.geometry:
        duties, phases = Audit.duties_and_phases(autd._ptr, dev.idx, 0)
        assert np.all(duties == 680)
        assert np.all(phases == 2048)


def test_transform():
    autd = create_controller()

    def transform(dev, tr, d) -> Drive:
        if dev.idx == 0:
            return Drive(d.phase + np.pi / 4, d.amp)
        else:
            return Drive(d.phase - np.pi / 4, d.amp)

    assert autd.send(Uniform(0.5).with_phase(np.pi).with_transform(transform))

    duties, phases = Audit.duties_and_phases(autd._ptr, 0, 0)
    assert np.all(duties == 680)
    assert np.all(phases == 2048 + 512)

    duties, phases = Audit.duties_and_phases(autd._ptr, 1, 0)
    assert np.all(duties == 680)
    assert np.all(phases == 2048 - 512)
