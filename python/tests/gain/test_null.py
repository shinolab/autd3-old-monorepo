'''
File: test_null.py
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

from pyautd3.gain import Null
from pyautd3.link.audit import Audit

import numpy as np


def test_null():
    autd = create_controller()

    assert autd.send(Null())

    for dev in autd.geometry:
        duties, phases = Audit.duties_and_phases(autd._ptr, dev.idx, 0)
        assert np.all(duties == 8)
        assert np.all(phases == 0)
