'''
File: stm_gain.py
Project: samples
Created Date: 21/07/2021
Author: Shun Suzuki
-----
Last Modified: 02/06/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''


from pyautd3 import AUTD, Focus, GainSTM, SilencerConfig, Static
import numpy as np


def stm_gain(autd: AUTD):
    config = SilencerConfig.none()
    autd.send(config)

    m = Static(1.0)

    radius = 30.0
    size = 200
    center = np.array([90.0, 80.0, 150.0])
    stm = GainSTM(autd)
    for i in range(size):
        theta = 2.0 * np.pi * i / size
        p = radius * np.array([np.cos(theta), np.sin(theta), 0])

        f = Focus(center + p)
        stm.add(f)

    stm.frequency = 1.0

    autd.send(m, stm)
