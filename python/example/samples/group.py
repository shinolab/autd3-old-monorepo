"""
File: group.py
Project: samples
Created Date: 15/09/2023
Author: Shun Suzuki
-----
Last Modified: 15/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


import numpy as np

from pyautd3 import Controller, Device, Silencer
from pyautd3.gain import Focus, Null
from pyautd3.modulation import Sine, Static


async def group(autd: Controller) -> None:
    config = Silencer()
    await autd.send(config)

    def grouping(dev: Device) -> str | None:
        match dev.idx:
            case 0:
                return "null"
            case 1:
                return "focus"
            case _:
                return None

    autd.group(grouping).set_data("null", Static(), Null()).set_data(
        "focus",
        Sine(150),
        Focus(autd.geometry.center + np.array([0.0, 0.0, 150.0])),
    ).send()
