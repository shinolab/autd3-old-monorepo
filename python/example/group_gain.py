"""
File: group_gain.py
Project: example
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""

import asyncio

import numpy as np

from pyautd3 import AUTD3, Controller
from pyautd3.gain import Focus, Group, Null
from pyautd3.link.nop import Nop
from pyautd3.modulation import Sine


async def main() -> None:
    with await Controller.builder().add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0])).open_with(Nop.builder()) as autd:
        cx = autd.geometry.center[0]
        g1 = Focus(autd.geometry.center + np.array([0, 0, 150]))
        g2 = Null()

        g = Group(lambda _, tr: "focus" if tr.position[0] < cx else "null").set_gain("focus", g1).set_gain("null", g2)

        m = Sine(150)

        await autd.send((m, g))

        await autd.close()


if __name__ == "__main__":
    asyncio.run(main())
