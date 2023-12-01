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


from pathlib import Path

import numpy as np
import pytest

from pyautd3 import SamplingConfiguration
from pyautd3.modulation.audio_file import Wav
from tests.test_autd import create_controller


@pytest.mark.asyncio()
async def test_wav():
    autd = await create_controller()

    assert await autd.send_async(Wav(Path(__file__).parent / "sin150.wav"))

    for dev in autd.geometry:
        mod = autd.link.modulation(dev.idx)
        mod_expect = [
            128,
            157,
            185,
            210,
            230,
            245,
            253,
            254,
            248,
            236,
            217,
            194,
            167,
            137,
            109,
            80,
            54,
            32,
            15,
            5,
            1,
            5,
            15,
            32,
            54,
            80,
            109,
            137,
            167,
            194,
            217,
            236,
            248,
            254,
            253,
            245,
            230,
            210,
            185,
            157,
            128,
            99,
            71,
            46,
            26,
            11,
            3,
            2,
            8,
            20,
            39,
            62,
            89,
            119,
            147,
            176,
            202,
            224,
            241,
            251,
            255,
            251,
            241,
            224,
            202,
            176,
            147,
            119,
            89,
            62,
            39,
            20,
            8,
            2,
            3,
            11,
            26,
            46,
            71,
            99,
        ]
        assert np.array_equal(mod, mod_expect)
        assert autd.link.modulation_frequency_division(dev.idx) == 5120

    assert await autd.send_async(
        Wav(Path(__file__).parent / "sin150.wav").with_sampling_config(
            SamplingConfiguration.from_frequency_division(512),
        ),
    )
    for dev in autd.geometry:
        assert autd.link.modulation_frequency_division(dev.idx) == 512
