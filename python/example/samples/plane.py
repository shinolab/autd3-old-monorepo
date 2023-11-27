"""
File: plane.py
Project: samples
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 14/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


import numpy as np

from pyautd3 import Controller, Silencer
from pyautd3.gain import Plane
from pyautd3.modulation import Sine


async def plane(autd: Controller) -> None:
    config = Silencer()
    await autd.send_async(config)

    f = Plane(np.array([0.0, 0.0, 1.0]))
    m = Sine(150)

    await autd.send_async(m, f)
