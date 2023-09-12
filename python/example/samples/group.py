"""
File: group.py
Project: samples
Created Date: 28/05/2023
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from pyautd3 import Controller, Silencer
from pyautd3.gain import Focus, Group, Bessel, Null
from pyautd3.modulation import Sine
import numpy as np


def group(autd: Controller):
    config = Silencer()
    autd.send(config)

    m = Sine(150)
    cx = autd.geometry.center[0]
    f = (
        Group(lambda _, tr: "focus" if tr.position[0] < cx else "null")
        .set("focus", Focus(autd.geometry.center + np.array([0.0, 0.0, 150.0])))
        .set("null", Null())
    )
    autd.send((m, f))
