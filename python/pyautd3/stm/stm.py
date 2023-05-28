"""
File: stm.py
Project: stm
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""

from typing import Optional

import numpy as np

from pyautd3.autd import Body
from pyautd3.gain.gain import Gain
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi import GainSTMMode


class FocusSTM(Body):
    def __init__(self):
        super().__init__()
        self.ptr = Base().focus_stm()

    def __del__(self):
        Base().delete_focus_stm(self.ptr)

    def add(self, point: np.ndarray, duty_shift: int = 0):
        Base().focus_stm_add(self.ptr, point[0], point[1], point[2], duty_shift)

    @property
    def frequency(self) -> float:
        return float(Base().focus_stm_frequency(self.ptr))

    @frequency.setter
    def frequency(self, freq: float):
        return Base().focus_stm_set_frequency(self.ptr, freq)

    @property
    def start_idx(self) -> Optional[int]:
        idx = int(Base().focus_stm_get_start_idx(self.ptr))
        if idx < 0:
            return None
        return idx

    @start_idx.setter
    def start_idx(self, value: Optional[int]):
        return Base().focus_stm_set_start_idx(self.ptr, -1 if value is None else value)

    @property
    def finish_idx(self) -> Optional[int]:
        idx = int(Base().focus_stm_get_finish_idx(self.ptr))
        if idx < 0:
            return None
        return idx

    @finish_idx.setter
    def finish_idx(self, value: Optional[int]):
        return Base().focus_stm_set_finish_idx(self.ptr, -1 if value is None else value)

    @property
    def sampling_frequency(self) -> float:
        return float(Base().focus_stm_sampling_frequency(self.ptr))

    @property
    def sampling_frequency_division(self) -> int:
        return int(Base().focus_stm_sampling_frequency_division(self.ptr))

    @sampling_frequency_division.setter
    def sampling_frequency_division(self, value: int):
        return Base().focus_stm_set_sampling_frequency_division(self.ptr, value)


class GainSTM(Body):
    def __init__(self):
        super().__init__()
        self.ptr = Base().gain_stm()

    def __del__(self):
        Base().delete_gain_stm(self.ptr)

    def add(self, gain: Gain):
        Base().gain_stm_add(self.ptr, gain.ptr)
        gain._disposed = True

    def set_mode(self, mode: GainSTMMode):
        return Base().gain_stm_set_mode(self.ptr, mode)

    @property
    def frequency(self) -> float:
        return float(Base().gain_stm_frequency(self.ptr))

    @frequency.setter
    def frequency(self, freq: float):
        return Base().gain_stm_set_frequency(self.ptr, freq)

    @property
    def start_idx(self) -> Optional[int]:
        idx = int(Base().gain_stm_get_start_idx(self.ptr))
        if idx < 0:
            return None
        return idx

    @start_idx.setter
    def start_idx(self, value: Optional[int]):
        return Base().gain_stm_set_start_idx(self.ptr, -1 if value is None else value)

    @property
    def finish_idx(self) -> Optional[int]:
        idx = int(Base().gain_stm_get_finish_idx(self.ptr))
        if idx < 0:
            return None
        return idx

    @finish_idx.setter
    def finish_idx(self, value: Optional[int]):
        return Base().gain_stm_set_finish_idx(self.ptr, -1 if value is None else value)

    @property
    def sampling_frequency(self) -> float:
        return float(Base().gain_stm_sampling_frequency(self.ptr))

    @property
    def sampling_frequency_division(self) -> int:
        return int(Base().gain_stm_sampling_frequency_division(self.ptr))

    @sampling_frequency_division.setter
    def sampling_frequency_division(self, value: int):
        return Base().gain_stm_set_sampling_frequency_division(self.ptr, value)
