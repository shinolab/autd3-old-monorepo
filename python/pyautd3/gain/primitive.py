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
from .gain import Gain


class Focus(Gain):
    def __init__(self, pos: np.ndarray, amp: float = 1.0):
        super().__init__()
        self.ptr = Base().gain_focus(pos[0], pos[1], pos[2], amp)

    def __del__(self):
        super().__del__()


class BesselBeam(Gain):
    def __init__(
        self, pos: np.ndarray, dir: np.ndarray, theta_z: float, amp: float = 1.0
    ):
        super().__init__()
        self.ptr = Base().gain_bessel_beam(
            pos[0],
            pos[1],
            pos[2],
            dir[0],
            dir[1],
            dir[2],
            theta_z,
            amp,
        )

    def __del__(self):
        super().__del__()


class PlaneWave(Gain):
    def __init__(self, dir: np.ndarray, amp: float = 1.0):
        super().__init__()
        self.ptr = Base().gain_plane_wave(dir[0], dir[1], dir[2], amp)

    def __del__(self):
        super().__del__()


class Custom(Gain):
    def __init__(self, amp: np.ndarray, phase: np.ndarray):
        super().__init__()
        p_size = len(phase)
        phase_ = np.ctypeslib.as_ctypes(phase.astype(np.double))
        amp_ = np.ctypeslib.as_ctypes(amp.astype(np.double))
        self.ptr = Base().gain_custom(amp_, phase_, p_size)

    def __del__(self):
        super().__del__()


class Null(Gain):
    def __init__(self):
        super().__init__()
        self.ptr = Base().gain_null()

    def __del__(self):
        super().__del__()


class Grouped(Gain):
    def __init__(self):
        super().__init__()
        self.ptr = Base().gain_grouped()

    def __del__(self):
        super().__del__()

    def add(self, dev_idx: int, gain: Gain):
        Base().gain_grouped_add(self.ptr, dev_idx, gain.ptr)
        gain._disposed = True


class TransTest(Gain):
    def __init__(self):
        super().__init__()
        self.ptr = Base().gain_transducer_test()

    def __del__(self):
        super().__del__()

    def set(self, tr_idx: int, amp: float, phase: float):
        Base().gain_transducer_test_set(self.ptr, tr_idx, amp, phase)
