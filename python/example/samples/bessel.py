'''
File: bessel.py
Project: samples
Created Date: 24/05/2021
Author: Shun Suzuki
-----
Last Modified: 21/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

'''


import numpy as np

from pyautd3 import Controller, Silencer
from pyautd3.gain import Bessel
from pyautd3.modulation import Sine


def bessel(autd: Controller):
    config = Silencer()
    autd.send(config)

    f = Bessel(autd.geometry.center, np.array([0.0, 0.0, 1.0]), 13.0 / 180 * np.pi)
    m = Sine(150)

    autd.send((m, f))
