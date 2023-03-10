'''
File: stm.py
Project: stm
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 08/03/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''

from ctypes import byref
from enum import IntEnum

from pyautd3.autd import Body
from pyautd3.gain.gain import Gain
from pyautd3.native_methods.autd3capi import NativeMethods as Base


class STM(Body):
    def __init__(self):
        super().__init__()

    def __del__(self):
        Base().dll.AUTDDeleteSTM(self.ptr)

    @ property
    def frequency(self):
        return Base().dll.AUTDSTMFrequency(self.ptr)

    @ frequency.setter
    def frequency(self, freq: float):
        return Base().dll.AUTDSTMSetFrequency(self.ptr, freq)

    @ property
    def start_idx(self):
        return Base().dll.AUTDSTMGetStartIdx(self.ptr)

    @ start_idx.setter
    def start_idx(self, value: int):
        return Base().dll.AUTDSTMSetStartIdx(self.ptr, value)

    @ property
    def finish_idx(self):
        return Base().dll.AUTDSTMGetFinishIdx(self.ptr)

    @ finish_idx.setter
    def finish_idx(self, value: int):
        return Base().dll.AUTDSTMSetFinishIdx(self.ptr, value)

    @ property
    def sampling_frequency(self):
        return Base().dll.AUTDSTMSamplingFrequency(self.ptr)

    @ property
    def sampling_frequency_division(self):
        return Base().dll.AUTDSTMSamplingFrequencyDivision(self.ptr)

    @ sampling_frequency_division.setter
    def sampling_frequency_division(self, value: int):
        return Base().dll.AUTDSTMSetSamplingFrequencyDivision(self.ptr, value)


class FocusSTM(STM):
    def __init__(self):
        super().__init__()
        Base().dll.AUTDFocusSTM(byref(self.ptr))

    def __del__(self):
        super().__del__()

    def add(self, point, duty_shift: int = 0):
        Base().dll.AUTDFocusSTMAdd(self.ptr, point[0], point[1], point[2], duty_shift)


class Mode(IntEnum):
    PhaseDutyFull = 0x01
    PhaseFull = 0x02
    PhaseHalf = 0x04


class GainSTM(STM):
    def __init__(self, mode: Mode = Mode.PhaseDutyFull):
        super().__init__()
        Base().dll.AUTDGainSTM(byref(self.ptr), int(mode))
        self._gains = []

    def __del__(self):
        super().__del__()

    def add(self, gain: Gain):
        Base().dll.AUTDGainSTMAdd(self.ptr, gain.ptr)
        self._gains.append(gain)
