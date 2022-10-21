'''
File: stm.py
Project: stm
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 21/10/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''

from ctypes import byref
from enum import IntEnum

from pyautd3.autd import Body, Controller
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
    def sampling_frequency(self):
        return Base().dll.AUTDSTMSamplingFrequency(self.ptr)

    @ property
    def sampling_frequency_division(self):
        return Base().dll.AUTDSTMSamplingFrequencyDivision(self.ptr)

    @ sampling_frequency_division.setter
    def sampling_frequency_division(self, value: int):
        return Base().dll.AUTDSTMSetSamplingFrequencyDivision(self.ptr, value)


class PointSTM(STM):
    def __init__(self):
        super().__init__()
        Base().dll.AUTDPointSTM(byref(self.ptr))

    def __del__(self):
        super().__del__()

    def add(self, point, duty_shift: int = 0):
        return Base().dll.AUTDPointSTMAdd(self.ptr, point[0], point[1], point[2], duty_shift)


class Mode(IntEnum):
    PhaseDutyFull = 0x01
    PhaseFull = 0x02
    PhaseHalf = 0x04


class GainSTM(STM):
    def __init__(self, autd: Controller):
        super().__init__()
        Base().dll.AUTDGainSTM(byref(self.ptr), autd.p_cnt)

    def __del__(self):
        super().__del__()

    def add(self, gain: Gain):
        return Base().dll.AUTDGainSTMAdd(self.ptr, gain.ptr)

    @ property
    def mode(self):
        return Mode(Base().dll.AUTDGetGainSTMMode(self.ptr))

    @ mode.setter
    def mode(self, value: Mode):
        Base().dll.AUTDSetGainSTMMode(self.ptr, int(value))
