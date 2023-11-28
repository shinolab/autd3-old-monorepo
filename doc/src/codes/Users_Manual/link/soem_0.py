import ctypes
import os
from datetime import timedelta
from typing import NoReturn

from pyautd3 import TimerStrategy
from pyautd3.link.soem import SOEM, OnErrFunc, SyncMode


def on_lost(msg: ctypes.c_char_p) -> NoReturn:
    print(msg.decode("utf-8"), end="")
    os._exit(-1)


def on_err(msg: ctypes.c_char_p) -> None:
    print(msg.decode("utf-8"), end="")


on_lost_func = OnErrFunc(on_lost)
on_err_func = OnErrFunc(on_err)
SOEM.builder()\
        .with_ifname("")\
        .with_buf_size(32)\
        .with_on_err(on_err_func)\
        .with_state_check_interval(timedelta(milliseconds=100))\
        .with_on_lost(on_lost_func)\
        .with_sync0_cycle(2)\
        .with_send_cycle(2)\
        .with_timer_strategy(TimerStrategy.BusyWait)\
        .with_sync_mode(SyncMode.DC)