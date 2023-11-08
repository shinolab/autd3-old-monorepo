"""
File: test_wav.py
Project: audio_file
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 11/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from datetime import timedelta
from pathlib import Path

import numpy as np
import pytest

from pyautd3.modulation.audio_file import Wav
from tests.test_autd import create_controller


@pytest.mark.asyncio()
async def test_wav():
    autd = create_controller()

    assert await autd.send(Wav(Path(__file__).parent / "sin150.wav"))

    for dev in autd.geometry:
        mod = autd.link.modulation(dev.idx)
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
            64,
        ]
        assert np.array_equal(mod, mod_expext)
        assert autd.link.modulation_frequency_division(dev.idx) == 5120

    assert await autd.send(Wav(Path(__file__).parent / "sin150.wav").with_sampling_frequency_division(512))
    for dev in autd.geometry:
        assert autd.link.modulation_frequency_division(dev.idx) == 512

    assert await autd.send(Wav(Path(__file__).parent / "sin150.wav").with_sampling_frequency(8e3))
    for dev in autd.geometry:
        assert autd.link.modulation_frequency_division(dev.idx) == 2560

    assert await autd.send(Wav(Path(__file__).parent / "sin150.wav").with_sampling_period(timedelta(microseconds=100)))
    for dev in autd.geometry:
        assert autd.link.modulation_frequency_division(dev.idx) == 2048
