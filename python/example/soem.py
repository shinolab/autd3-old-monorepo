'''
File: soem.py
Project: example
Created Date: 30/12/2020
Author: Shun Suzuki
-----
Last Modified: 17/04/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2020 Shun Suzuki. All rights reserved.

'''

import os
import ctypes

from pyautd3 import Controller, Geometry
from pyautd3.link import SOEM, OnLostFunc

from samples import runner


def on_lost(msg: ctypes.c_char_p):
    print(msg.decode('utf-8'), end="")
    os._exit(-1)


if __name__ == '__main__':
    geometry = Geometry.Builder().add_device([0., 0., 0.], [0., 0., 0.]).build()

    on_lost_func = OnLostFunc(on_lost)
    link = SOEM().on_lost(on_lost_func).build()

    autd = Controller.open(geometry, link)

    runner.run(autd)
