'''
File: bessel.py
Project: samples
Created Date: 24/05/2021
Author: Shun Suzuki
-----
Last Modified: 02/06/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''


import math

from pyautd3 import AUTD, BesselBeam, Sine, SilencerConfig


def bessel(autd: AUTD):
    config = SilencerConfig()
    autd.send(config)

    f = BesselBeam([90., 80., 0.], [0., 0., 1.], 13. / 180 * math.pi)
    m = Sine(150)

    autd.send(m, f)
