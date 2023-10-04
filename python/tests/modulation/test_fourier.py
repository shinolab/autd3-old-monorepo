'''
File: test_fourier.py
Project: modulation
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 04/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from ..test_autd import create_controller

from pyautd3.modulation import Sine, Fourier
from pyautd3.link.audit import Audit

import numpy as np


def test_fourier():
    autd = create_controller()

    m = Fourier(Sine(50)).add_components_from_iter(map(lambda x: Sine(x), [100, 150])) + Sine(200)
    assert autd.send(m)

    for dev in autd.geometry:
        mod = Audit.modulation(autd._ptr, dev.idx)
        mod_expext = [
            85,
            103,
            123,
            142,
            159,
            173,
            181,
            182,
            176,
            164,
            151,
            136,
            122,
            109,
            99,
            90,
            85,
            81,
            80,
            82,
            85,
            89,
            93,
            98,
            102,
            105,
            106,
            105,
            103,
            99,
            94,
            89,
            85,
            80,
            77,
            75,
            75,
            76,
            78,
            81,
            85,
            88,
            91,
            94,
            95,
            94,
            92,
            89,
            85,
            80,
            75,
            71,
            67,
            65,
            65,
            66,
            68,
            72,
            76,
            80,
            85,
            88,
            89,
            88,
            85,
            79,
            71,
            62,
            51,
            41,
            32,
            24,
            18,
            15,
            16,
            20,
            27,
            37,
            51,
            67,
        ]
        assert np.array_equal(mod, mod_expext)
        assert Audit.modulation_frequency_division(autd._ptr, dev.idx) == 40960
