'''
File: holo.py
Project: samples
Created Date: 24/05/2021
Author: Shun Suzuki
-----
Last Modified: 02/06/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''


from pyautd3 import AUTD, GSPAT, Sine, SilencerConfig, EigenBackend


def holo(autd: AUTD):
    config = SilencerConfig()
    autd.send(config)

    backend = EigenBackend()

    f = GSPAT(backend)
    f.add([120., 80., 150.], 1.0)
    f.add([60., 80., 150.], 1.0)

    m = Sine(150)

    autd.send(m, f)
