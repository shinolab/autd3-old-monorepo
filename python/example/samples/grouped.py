"""
File: grouped.py
Project: samples
Created Date: 28/05/2023
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from pyautd3 import Controller, SilencerConfig
from pyautd3.gain import Focus, Grouped, Bessel
from pyautd3.modulation import Sine
import numpy as np


def grouped(autd: Controller):
    config = SilencerConfig()
    autd.send(config)

    f1 = Focus(autd.geometry.center_of(0) + np.array([0.0, 0.0, 150.0]))
    f2 = Bessel(autd.geometry.center_of(1), np.array([0.0, 0.0, 1.0]), 13 / 180 * np.pi)

    f = Grouped().add(0, f1).add(1, f2)
    m = Sine(150)

    autd.send((m, f))
