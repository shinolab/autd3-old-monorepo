'''
File: stm_point.py
Project: samples
Created Date: 24/05/2021
Author: Shun Suzuki
-----
Last Modified: 02/06/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''


import numpy as np
from pyautd3 import AUTD, PointSTM, Static, SilencerConfig


def stm_point(autd: AUTD):
    config = SilencerConfig.none()
    autd.send(config)

    m = Static(1.0)

    stm = PointSTM()
    radius = 30.0
    size = 200
    center = [90.0, 80.0, 150.0]
    for i in range(size):
        theta = 2.0 * np.pi * i / size
        p = radius * np.array([np.cos(theta), np.sin(theta), 0])
        stm.add(center + p)

    stm.frequency = 1.0

    autd.send(m, stm)
