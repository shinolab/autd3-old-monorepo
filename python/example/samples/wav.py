"""
File: wav.py
Project: samples
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 14/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from pathlib import Path

import numpy as np

from pyautd3 import Controller, Silencer
from pyautd3.gain import Focus
from pyautd3.modulation.audio_file import Wav


async def wav(autd: Controller) -> None:
    config = Silencer()
    await autd.send_async(config)

    f = Focus(autd.geometry.center + np.array([0.0, 0.0, 150.0]))
    m = Wav(Path(__file__).parent / "sin150.wav")

    await autd.send_async((m, f))
