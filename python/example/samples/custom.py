'''
File: custom.py
Project: samples
Created Date: 11/10/2021
Author: Shun Suzuki
-----
Last Modified: 08/03/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''

from datetime import timedelta
from pyautd3 import Controller, SilencerConfig
from pyautd3.gain import Custom
from pyautd3.modulation import Sine

import numpy as np


def focus(autd: Controller, point):
    point = np.array(point)

    amp = np.zeros(autd.geometry.num_transducers)
    phase = np.zeros(autd.geometry.num_transducers)

    for tr in autd.geometry:
        tp = tr.position
        dist = np.linalg.norm(tp - point)
        phase[tr.id] = 2.0 * np.pi * dist / tr.wavelength
        amp[tr.id] = 1.0

    return Custom(amp, phase)


def custom(autd: Controller):
    config = SilencerConfig()
    autd.send(config, timeout=timedelta(milliseconds=20))

    f = focus(autd, autd.geometry.center + np.array([0., 0., 150.]))
    m = Sine(150)

    autd.send(m, f, timeout=timedelta(milliseconds=20))
