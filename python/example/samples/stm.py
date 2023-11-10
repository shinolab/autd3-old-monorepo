"""
File: stm.py
Project: samples
Created Date: 29/08/2023
Author: Shun Suzuki
-----
Last Modified: 25/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


import numpy as np

from pyautd3 import Controller, Silencer
from pyautd3.gain import Focus
from pyautd3.modulation import Static
from pyautd3.stm import FocusSTM, GainSTM


async def stm_focus(autd: Controller) -> None:
    config = Silencer.disable()
    await autd.send_async(config)

    m = Static()

    radius = 30.0
    size = 200
    center = autd.geometry.center + np.array([0.0, 0.0, 150.0])
    stm = FocusSTM(1.0).add_foci_from_iter(
        center + radius * np.array([np.cos(theta), np.sin(theta), 0]) for theta in (2.0 * np.pi * i / size for i in range(size))
    )

    await autd.send_async(m, stm)


async def stm_gain(autd: Controller) -> None:
    config = Silencer.disable()
    await autd.send_async(config)

    m = Static()

    radius = 30.0
    size = 50
    center = autd.geometry.center + np.array([0.0, 0.0, 150.0])
    stm = GainSTM(1.0).add_gains_from_iter(
        Focus(center + radius * np.array([np.cos(theta), np.sin(theta), 0])) for theta in (2.0 * np.pi * i / size for i in range(size))
    )

    await autd.send_async(m, stm)
