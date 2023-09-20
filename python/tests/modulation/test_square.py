'''
File: test_square.py
Project: modulation
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 20/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from datetime import timedelta
from ..test_autd import create_controller

from pyautd3.modulation import Square
from pyautd3.link.audit import Audit

import numpy as np


def test_square():
    autd = create_controller()

    assert autd.send(Square(200).with_low(0.2).with_high(0.5).with_duty(0.1))

    for dev in autd.geometry:
        mod = Audit.modulation(autd._ptr, dev.idx)
        mod_expext = [
            85, 85, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32]
        assert np.array_equal(mod, mod_expext)
        assert Audit.modulation_frequency_division(autd._ptr, dev.idx) == 40960

    assert autd.send(Square(150).with_sampling_frequency_division(4096 // 8))
    for dev in autd.geometry:
        assert Audit.modulation_frequency_division(autd._ptr, dev.idx) == 4096

    assert autd.send(Square(150).with_sampling_frequency(8e3))
    for dev in autd.geometry:
        assert Audit.modulation_frequency_division(autd._ptr, dev.idx) == 20480

    assert autd.send(Square(150).with_sampling_period(timedelta(microseconds=100)))
    for dev in autd.geometry:
        assert Audit.modulation_frequency_division(autd._ptr, dev.idx) == 16384
