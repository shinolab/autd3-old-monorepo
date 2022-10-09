'''
File: focus.py
Project: samples
Created Date: 24/05/2021
Author: Shun Suzuki
-----
Last Modified: 02/06/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''

from pyautd3 import AUTD, Focus, Sine, SilencerConfig


def simple(autd: AUTD):
    config = SilencerConfig()
    autd.send(config)

    f = Focus([90., 80., 150.])
    m = Sine(150)

    autd.send(m, f)
