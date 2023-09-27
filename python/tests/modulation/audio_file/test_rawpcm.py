'''
File: test_rawpcm.py
Project: audio_file
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 25/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''

from datetime import timedelta
import os
from ...test_autd import create_controller

from pyautd3.modulation.audio_file import RawPCM
from pyautd3.link.audit import Audit

import numpy as np


def test_rawpcm():
    autd = create_controller()

    assert autd.send(RawPCM(os.path.join(os.path.dirname(__file__), "sin150.dat"), 4000))

    for dev in autd.geometry:
        mod = Audit.modulation(autd._ptr, dev.idx)
        mod_expext = [
            107,
            131,
            157,
            184,
            209,
            234,
            255,
            219,
            191,
            166,
            140,
            115,
            92,
            70,
            51,
            33,
            19,
            8,
            2,
            0,
            2,
            8,
            19,
            33,
            51,
            70,
            92,
            115,
            140,
            166,
            191,
            219,
            255,
            234,
            209,
            184,
            157,
            131,
            107,
            85,
            64,
            45,
            28,
            15,
            6,
            1,
            0,
            3,
            12,
            23,
            39,
            57,
            77,
            99,
            123,
            148,
            174,
            200,
            226,
            255,
            226,
            200,
            174,
            148,
            123,
            99,
            77,
            57,
            39,
            23,
            12,
            3,
            0,
            1,
            6,
            15,
            28,
            45,
            64,
            85]
        assert np.array_equal(mod, mod_expext)
        assert Audit.modulation_frequency_division(autd._ptr, dev.idx) == 40960

    assert autd.send(RawPCM(os.path.join(os.path.dirname(__file__), "sin150.dat"), 4000).with_sampling_frequency_division(4096 // 8))
    for dev in autd.geometry:
        assert Audit.modulation_frequency_division(autd._ptr, dev.idx) == 4096

    assert autd.send(RawPCM(os.path.join(os.path.dirname(__file__), "sin150.dat"), 4000).with_sampling_frequency(8e3))
    for dev in autd.geometry:
        assert Audit.modulation_frequency_division(autd._ptr, dev.idx) == 20480

    assert autd.send(RawPCM(os.path.join(os.path.dirname(__file__), "sin150.dat"), 4000).with_sampling_period(timedelta(microseconds=100)))
    for dev in autd.geometry:
        assert Audit.modulation_frequency_division(autd._ptr, dev.idx) == 16384
