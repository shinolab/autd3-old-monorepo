'''
File: group.py
Project: samples
Created Date: 15/09/2023
Author: Shun Suzuki
-----
Last Modified: 15/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from pyautd3 import Controller, Silencer
from pyautd3.gain import Focus, Null
from pyautd3.modulation import Sine, Static
import numpy as np


def group(autd: Controller):
    config = Silencer()
    autd.send(config)

    def grouping(dev):
        if dev.idx == 0:
            return "null"
        elif dev.idx == 1:
            return "focus"
        else:
            return None

    autd.group(grouping)\
        .set("null", (Static(), Null()))\
        .set("focus", (Sine(150), Focus(autd.geometry.center + np.array([0.0, 0.0, 150.0]))))\
        .send()
