'''
File: test_simulator.py
Project: link
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
import os
import pytest

from pyautd3 import Controller, AUTD3, Level, TimerStrategy
from pyautd3.link import SOEM, RemoteSOEM, SyncMode
from pyautd3.link import LogFlushFunc, LogOutputFunc, OnLostFunc


def on_lost_f(msg: ctypes.c_char_p):
    print(msg.decode("utf-8"), end="")
    os._exit(-1)


def on_out_f(msg: ctypes.c_char_p):
    print(msg.decode("utf-8"), end="")


def on_flush_f():
    pass


@pytest.mark.soem
def test_soem():
    list = SOEM.enumerate_adapters()
    print(list)

    on_lost = OnLostFunc(on_lost_f)
    log_out = LogOutputFunc(on_out_f)
    log_flush = LogFlushFunc(on_flush_f)
    autd = Controller.builder()\
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))\
        .open_with(
            SOEM()
            .with_ifname("")
            .with_buf_size(32)
            .with_send_cycle(2)
            .with_sync0_cycle(2)
            .with_on_lost(on_lost)
            .with_timer_strategy(TimerStrategy.Sleep)
            .with_sync_mode(SyncMode.FreeRun)
            .with_state_check_interval(timedelta(milliseconds=100))
            .with_log_level(Level.Off)
            .with_log_func(log_out, log_flush)
            .with_timeout(timedelta(milliseconds=200))
    )

    autd.close()


@pytest.mark.remote_soem
def test_remote_soem():
    autd = Controller.builder()\
        .add_device(AUTD3.from_euler_zyz([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]))\
        .open_with(
            RemoteSOEM("127.0.0.1:8080").with_timeout(timedelta(milliseconds=200))
    )

    autd.close()
