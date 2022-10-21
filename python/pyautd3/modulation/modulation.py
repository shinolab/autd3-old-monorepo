'''
File: modulation.py
Project: modulation
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 21/10/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''


from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.autd import Header


class Modulation(Header):
    def __init__(self):
        super().__init__()

    def __del__(self):
        Base().dll.AUTDDeleteModulation(self.ptr)

    @ property
    def sampling_frequency_division(self):
        return Base().dll.AUTDModulationSamplingFrequencyDivision(self.ptr)

    @ sampling_frequency_division.setter
    def sampling_frequency_division(self, value: int):
        return Base().dll.AUTDModulationSetSamplingFrequencyDivision(self.ptr, value)

    @ property
    def sampling_frequency(self):
        return Base().dll.AUTDModulationSamplingFrequency(self.ptr)
