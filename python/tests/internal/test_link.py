'''
File: test_link.py
Project: internal
Created Date: 20/09/2023
Author: Shun Suzuki
-----
Last Modified: 20/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


import ctypes
from datetime import timedelta
from pyautd3 import Controller, AUTD3, Level
from pyautd3.link import Debug, LogOutputFunc, LogFlushFunc


def on_out(msg: ctypes.c_char_p):
    print(msg.decode("utf-8"), end="")


def on_flush():
    pass


def test_log():
    log_out = LogOutputFunc(on_out)
    log_flush = LogFlushFunc(on_flush)
    autd = Controller.builder()\
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))\
        .add_device(AUTD3.from_quaternion([0.0, 0.0, 0.0], [1.0, 0.0, 0.0, 0.0]))\
        .open_with(Debug().with_timeout(timedelta(milliseconds=20)).with_log().with_log_func(log_out, log_flush).with_log_level(Level.Info))

    autd.close()
