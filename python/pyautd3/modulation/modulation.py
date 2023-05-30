"""
File: modulation.py
Project: modulation
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 28/05/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""


from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.autd import Header


class Modulation(Header):
    def __init__(self):
        super().__init__()

    def __del__(self):
        Base().delete_modulation(self.ptr)

    @property
    def sampling_frequency_division(self) -> int:
        return int(Base().modulation_sampling_frequency_division(self.ptr))

    @sampling_frequency_division.setter
    def sampling_frequency_division(self, value: int):
        return Base().modulation_set_sampling_frequency_division(self.ptr, value)

    @property
    def sampling_frequency(self) -> float:
        return float(Base().modulation_sampling_frequency(self.ptr))
