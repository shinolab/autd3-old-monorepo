"""
File: stm_focus.py
Project: samples
Created Date: 24/05/2021
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""


from pyautd3 import Controller, SilencerConfig
from pyautd3.stm import FocusSTM
from pyautd3.modulation import Static
import numpy as np


def stm_focus(autd: Controller):
    config = SilencerConfig.none()
    autd.send(config)

    m = Static()

    stm = FocusSTM(1.0)
    radius = 30.0
    size = 200
    center = autd.geometry.center + np.array([0.0, 0.0, 150.0])
    for i in range(size):
        theta = 2.0 * np.pi * i / size
        p = radius * np.array([np.cos(theta), np.sin(theta), 0])
        stm.add_focus(center + p)

    autd.send((m, stm))
