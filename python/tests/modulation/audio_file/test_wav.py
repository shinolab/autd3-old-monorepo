'''
File: test_wav.py
Project: audio_file
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 20/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from datetime import timedelta
import os
from ...test_autd import create_controller

from pyautd3.modulation.audio_file import Wav
from pyautd3.link.audit import Audit

import numpy as np


def test_wav():
    autd = create_controller()

    assert autd.send(Wav(os.path.join(os.path.dirname(__file__), "sin150.wav")))

    for dev in autd.geometry:
        mod = Audit.modulation(autd._ptr, dev.idx)
        mod_expext = [
            85,
            107,
            131,
            157,
            182,
            209,
            234,
            240,
            216,
            191,
            165,
            140,
            115,
            92,
            71,
            51,
            34,
            20,
            9,
            3,
            0,
            3,
            9,
            20,
            34,
            51,
            71,
            92,
            115,
            140,
            165,
            191,
            216,
            240,
            234,
            209,
            182,
            157,
            131,
            107,
            85,
            64,
            45,
            29,
            16,
            7,
            1,
            1,
            5,
            12,
            24,
            39,
            57,
            78,
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
            78,
            57,
            39,
            24,
            12,
            5,
            1,
            1,
            7,
            16,
            29,
            45,
            64]
        assert np.array_equal(mod, mod_expext)
        assert Audit.modulation_frequency_division(autd._ptr, dev.idx) == 40960

    assert autd.send(Wav(os.path.join(os.path.dirname(__file__), "sin150.wav")).with_sampling_frequency_division(4096 // 8))
    for dev in autd.geometry:
        assert Audit.modulation_frequency_division(autd._ptr, dev.idx) == 4096

    assert autd.send(Wav(os.path.join(os.path.dirname(__file__), "sin150.wav")).with_sampling_frequency(8e3))
    for dev in autd.geometry:
        assert Audit.modulation_frequency_division(autd._ptr, dev.idx) == 20480

    assert autd.send(Wav(os.path.join(os.path.dirname(__file__), "sin150.wav")).with_sampling_period(timedelta(microseconds=100)))
    for dev in autd.geometry:
        assert Audit.modulation_frequency_division(autd._ptr, dev.idx) == 16384
