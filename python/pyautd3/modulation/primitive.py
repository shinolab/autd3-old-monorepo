"""
File: primitive.py
Project: gain
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""

import numpy as np

from pyautd3.native_methods.autd3capi import NativeMethods as Base
from .modulation import Modulation


class Static(Modulation):
    def __init__(self, amp: float = 1.0):
        super().__init__()
        self.ptr = Base().modulation_static(amp)

    def __del__(self):
        super().__del__()


class Custom(Modulation):
    def __init__(self, data: np.ndarray, sampling_freq_div: int):
        super().__init__()
        size = len(data)
        data = data.astype(np.double)
        data_ = np.ctypeslib.as_ctypes(data)

        self.ptr = Base().modulation_custom(data_, size, sampling_freq_div)

    def __del__(self):
        super().__del__()


class Sine(Modulation):
    def __init__(self, freq: int, amp: float = 1.0, offset: float = 0.5):
        super().__init__()
        self.ptr = Base().modulation_sine(freq, amp, offset)

    def __del__(self):
        super().__del__()


class SineSquared(Modulation):
    def __init__(self, freq: int, amp: float = 1.0, offset: float = 0.5):
        super().__init__()
        self.ptr = Base().modulation_sine_squared(freq, amp, offset)

    def __del__(self):
        super().__del__()


class SineLegacy(Modulation):
    def __init__(self, freq: float, amp: float = 1.0, offset: float = 0.5):
        super().__init__()
        self.ptr = Base().modulation_sine_legacy(freq, amp, offset)

    def __del__(self):
        super().__del__()


class Square(Modulation):
    def __init__(
        self, freq: int, low: float = 0.0, high: float = 1.0, duty: float = 0.5
    ):
        super().__init__()
        self.ptr = Base().modulation_square(freq, low, high, duty)

    def __del__(self):
        super().__del__()
