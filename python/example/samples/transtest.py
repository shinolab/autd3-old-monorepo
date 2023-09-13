'''
File: transtest.py
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
from pyautd3.gain import TransducerTest
from pyautd3.modulation import Sine


def transtest(autd: Controller):
    config = Silencer()
    autd.send(config)

    f = TransducerTest().set(0, 0, 1.0, 0.0).set(0, 248, 1.0, 0.0)
    m = Sine(150)

    autd.send((m, f))
