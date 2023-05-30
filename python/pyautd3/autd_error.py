"""
File: autd_error.py
Project: pyautd3
Created Date: 28/05/2023
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""

import ctypes


class AUTDError(Exception):
    msg: str

    def __init__(self, err: ctypes.Array[ctypes.c_char]):
        self.msg = err.value.decode("utf-8")

    def __str__(self):
        return self.msg

    def __repr__(self):
        return self.msg
