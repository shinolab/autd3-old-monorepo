'''
File: sync_mode.py
Project: pyautd3
Created Date: 20/03/2023
Author: Shun Suzuki
-----
Last Modified: 20/03/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from enum import Enum


class SyncMode(Enum):
    DC = 0
    FreeRun = 1
