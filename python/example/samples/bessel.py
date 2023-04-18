'''
File: bessel.py
Project: samples
Created Date: 24/05/2021
Author: Shun Suzuki
-----
Last Modified: 17/04/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''


import math

from pyautd3 import Controller, SilencerConfig
from pyautd3.gain import BesselBeam
from pyautd3.modulation import Sine


def bessel(autd: Controller):
    config = SilencerConfig()
    autd.send(config)

    f = BesselBeam(autd.geometry.center, [0., 0., 1.], 13. / 180 * math.pi)
    m = Sine(150)

    autd.send(m, f)
