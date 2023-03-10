'''
File: stm_gain.py
Project: samples
Created Date: 21/07/2021
Author: Shun Suzuki
-----
Last Modified: 08/03/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''

from datetime import timedelta
from pyautd3 import Controller, SilencerConfig
from pyautd3.gain import Focus
from pyautd3.stm import GainSTM
from pyautd3.modulation import Static
import numpy as np


def stm_gain(autd: Controller):
    config = SilencerConfig.none()
    autd.send(config, timeout=timedelta(milliseconds=20))

    m = Static(1.0)

    radius = 30.0
    size = 200
    center = autd.geometry.center + np.array([0., 0., 150.])
    stm = GainSTM()
    for i in range(size):
        theta = 2.0 * np.pi * i / size
        p = radius * np.array([np.cos(theta), np.sin(theta), 0])
        f = Focus(center + p)
        stm.add(f)

    stm.frequency = 1.0

    autd.send(m, stm, timeout=timedelta(milliseconds=20))
