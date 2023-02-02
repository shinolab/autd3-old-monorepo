'''
File: soem.py
Project: example
Created Date: 30/12/2020
Author: Shun Suzuki
-----
Last Modified: 02/02/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2020 Shun Suzuki. All rights reserved.

'''

import os
import ctypes

from pyautd3 import Controller, GeometryBuilder
from pyautd3.link import SOEM, OnLostFunc

from samples import runner


def on_lost(msg: ctypes.c_char_p):
    print(msg.decode('utf-8'), end="")
    os._exit(-1)


if __name__ == '__main__':
    geometry = GeometryBuilder().add_device([0., 0., 0.], [0., 0., 0.]).build()

    on_lost_f = OnLostFunc(on_lost)
    link = SOEM().high_precision(True).on_lost(on_lost_f).build()

    autd = Controller.open(geometry, link)

    autd.ack_check_timeout_ms = 20

    runner.run(autd)
