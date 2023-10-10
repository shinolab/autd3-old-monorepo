'''
File: test_sine.py
Project: modulation
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from datetime import timedelta
from ..test_autd import create_controller

from pyautd3.modulation import Sine
from pyautd3.link.audit import Audit

import numpy as np


def test_sine():
    autd = create_controller()

    assert autd.send(Sine(150).with_amp(0.5).with_offset(0.25).with_phase(np.pi / 2))

    for dev in autd.geometry:
        mod = autd.link().modulation(dev.idx)
        mod_expext = [
            85,
            83,
            79,
            73,
            66,
            57,
            47,
            37,
            28,
            19,
            11,
            5,
            1,
            0,
            0,
            3,
            7,
            14,
            22,
            31,
            41,
            50,
            60,
            69,
            76,
            81,
            84,
            84,
            82,
            78,
            71,
            63,
            54,
            44,
            34,
            25,
            16,
            9,
            4,
            1,
            0,
            1,
            4,
            9,
            16,
            25,
            34,
            44,
            54,
            63,
            71,
            78,
            82,
            84,
            84,
            81,
            76,
            69,
            60,
            50,
            41,
            31,
            22,
            14,
            7,
            3,
            0,
            0,
            1,
            5,
            11,
            19,
            28,
            37,
            47,
            57,
            66,
            73,
            79,
            83]
        assert np.array_equal(mod, mod_expext)
        assert autd.link().modulation_frequency_division(dev.idx) == 40960

    assert autd.send(Sine(150).with_sampling_frequency_division(4096 // 8))
    for dev in autd.geometry:
        assert autd.link().modulation_frequency_division(dev.idx) == 4096

    assert autd.send(Sine(150).with_sampling_frequency(8e3))
    for dev in autd.geometry:
        assert autd.link().modulation_frequency_division(dev.idx) == 20480

    assert autd.send(Sine(150).with_sampling_period(timedelta(microseconds=100)))
    for dev in autd.geometry:
        assert autd.link().modulation_frequency_division(dev.idx) == 16384
