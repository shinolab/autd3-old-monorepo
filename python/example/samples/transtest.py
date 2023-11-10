"""
File: transtest.py
Project: samples
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 14/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from pyautd3 import Controller, Silencer
from pyautd3.gain import TransducerTest
from pyautd3.modulation import Sine


async def transtest(autd: Controller) -> None:
    config = Silencer()
    await autd.send_async(config)

    f = TransducerTest().set_drive(0, 0, 0.0, 1.0).set_drive(0, 248, 0.0, 1.0)
    m = Sine(150)

    await autd.send_async(m, f)
