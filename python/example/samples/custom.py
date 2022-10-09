'''
File: custom.py
Project: samples
Created Date: 11/10/2021
Author: Shun Suzuki
-----
Last Modified: 22/06/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''


from pyautd3 import AUTD, CustomGain, Sine, NUM_TRANS_IN_UNIT, SilencerConfig
import numpy as np


def focus(autd: AUTD, point):
    point = np.array(point)
    trans_num = autd.num_devices() * NUM_TRANS_IN_UNIT

    amp = np.zeros(trans_num)
    phase = np.zeros(trans_num)

    for dev in range(autd.num_devices()):
        for i in range(NUM_TRANS_IN_UNIT):
            tp = autd.trans_pos(dev, i)
            dist = np.linalg.norm(tp - point)
            wavenum = 2.0 * np.pi / autd.wavelength(dev, i)
            phase[i] = wavenum * dist / (2.0 * np.pi)
            amp[i] = 1.0

    return CustomGain(amp, phase)


def custom(autd: AUTD):
    config = SilencerConfig()
    autd.send(config)

    f = focus(autd, [90., 80., 150.])
    m = Sine(150)

    autd.send(m, f)
