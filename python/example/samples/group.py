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
from pyautd3.gain import Focus, Group, Null
from pyautd3.modulation import Sine, Static


async def group_by_device(autd: Controller) -> None:
    config = Silencer()
    await autd.send_async(config)

    def grouping(dev: Device) -> str | None:
        match dev.idx:
            case 0:
                return "null"
            case 1:
                return "focus"
            case _:
                return None

    await (
        autd.group(grouping)
        .set_data("null", Static(), Null())
        .set_data(
            "focus",
            Sine(150),
            Focus(autd.geometry.center + np.array([0.0, 0.0, 150.0])),
        )
        .send_async()
    )


async def group_by_transducer(autd: Controller) -> None:
    config = Silencer()
    await autd.send_async(config)

    cx = autd.geometry.center[0]
    g1 = Focus(autd.geometry.center + np.array([0, 0, 150]))
    g2 = Null()

    g = Group(lambda _, tr: "focus" if tr.position[0] < cx else "null").set_gain("focus", g1).set_gain("null", g2)

    m = Sine(150)

    await autd.send_async((m, g))
