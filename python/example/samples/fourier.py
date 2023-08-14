'''
Author: Mingxin Zhang m.zhang@hapis.k.u-tokyo.ac.jp
Date: 2023-08-14 15:43:17
LastEditors: Mingxin Zhang
LastEditTime: 2023-08-14 17:16:36
Copyright (c) 2023 by Mingxin Zhang, All Rights Reserved. 
'''
from pyautd3 import Controller, SilencerConfig
from pyautd3.gain import Focus
from pyautd3.modulation import Sine, Fourier
import numpy as np


def fourier(autd: Controller):
    config = SilencerConfig()
    autd.send(config)

    f = Focus(autd.geometry.center + np.array([0.0, 0.0, 150.0]))
    m = Fourier()
    m.add_component(Sine(100)).add_component(Sine(150))

    autd.send((m, f))