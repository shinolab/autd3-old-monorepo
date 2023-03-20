'''
File: timer_strategy.py
Project: pyautd3
Created Date: 20/03/2023
Author: Shun Suzuki
-----
Last Modified: 20/03/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from enum import IntEnum


class TimerStrategy(IntEnum):
    Sleep = 0
    BusyWait = 1
    NativeTimer = 2
