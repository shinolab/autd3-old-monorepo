'''
File: wav.py
Project: samples
Created Date: 14/09/2023
Author: Shun Suzuki
-----
Last Modified: 14/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from pyautd3 import Controller, Silencer
from pyautd3.gain import Focus
from pyautd3.modulation.audio_file import Wav
import numpy as np

import os


def wav(autd: Controller):
    config = Silencer()
    autd.send(config)

    f = Focus(autd.geometry.center + np.array([0.0, 0.0, 150.0]))
    m = Wav(os.path.join(os.path.dirname(__file__), "sin150.wav"))

    autd.send((m, f))
