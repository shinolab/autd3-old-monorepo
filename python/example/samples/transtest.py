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


from pyautd3 import Controller, Device, Drive, EmitIntensity, Phase, Silencer, Transducer
from pyautd3.gain import TransducerTest
from pyautd3.modulation import Sine


async def transtest(autd: Controller) -> None:
    config = Silencer()
    await autd.send_async(config)

    def f(dev: Device, tr: Transducer) -> Drive | None:
        match (dev.idx, tr.idx):
            case (0, 0):
                return Drive(Phase(0), EmitIntensity.maximum())
            case (0, 248):
                return Drive(Phase(0), EmitIntensity.maximum())
            case _:
                return None

    g = TransducerTest(f)
    m = Sine(150)

    await autd.send_async(m, g)
