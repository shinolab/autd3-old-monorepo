'''
File: test_plane.py
Project: gain
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from ..test_autd import create_controller

from pyautd3.gain import Plane
from pyautd3.link.audit import Audit

import numpy as np


def test_plane():
    autd = create_controller()

    assert autd.send(Plane([0, 0, 1]).with_amp(0.5))

    for dev in autd.geometry:
        duties, phases = autd.link().duties_and_phases(dev.idx, 0)
        assert np.all(duties == 680)
        assert np.all(phases == 0)
