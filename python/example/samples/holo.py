'''
File: holo.py
Project: samples
Created Date: 24/05/2021
Author: Shun Suzuki
-----
Last Modified: 17/04/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''


from pyautd3 import Controller, SilencerConfig
from pyautd3.gain.holo import EigenBackend, GSPAT
from pyautd3.modulation import Sine
import numpy as np


def holo(autd: Controller):
    config = SilencerConfig()
    autd.send(config)

    backend = EigenBackend()

    f = GSPAT(backend)
    center = autd.geometry.center + np.array([0, 0, 150])
    f.add(center - np.array([30., 0., 0.]), 1.0)
    f.add(center + np.array([30., 0., 0.]), 1.0)

    m = Sine(150)

    autd.send(m, f)
