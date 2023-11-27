"""
File: focus.py
Project: samples
Created Date: 24/05/2021
Author: Shun Suzuki
-----
Last Modified: 21/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""


import numpy as np

from pyautd3 import Controller, Silencer
from pyautd3.gain import Focus
from pyautd3.modulation import Sine


async def simple(autd: Controller) -> None:
    config = Silencer()
    await autd.send_async(config)

    f = Focus(autd.geometry.center + np.array([0.0, 0.0, 150.0]))
    m = Sine(150)

    await autd.send_async(m, f)
