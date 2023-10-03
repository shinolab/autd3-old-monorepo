'''
File: autd_error.py
Project: pyautd3
Created Date: 28/05/2023
Author: Shun Suzuki
-----
Last Modified: 03/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''

import ctypes
from typing import Union


class AUTDError(Exception):
    msg: str

    def __init__(self, err: Union[ctypes.Array[ctypes.c_char], str]):
        self.msg = err if isinstance(err, str) else err.value.decode("utf-8")

    def __str__(self):
        return self.msg

    def __repr__(self):
        return self.msg
