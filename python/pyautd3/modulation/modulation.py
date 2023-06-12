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


from abc import ABCMeta, abstractmethod

from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import DatagramHeaderPtr, ModulationPtr

from pyautd3.autd import Header


class IModulation(Header, metaclass=ABCMeta):
    def __init__(self):
        super().__init__()

    def ptr(self) -> DatagramHeaderPtr:
        return Base().modulation_into_datagram(self.modulation_ptr())

    @property
    def sampling_frequency_division(self) -> int:
        return int(Base().modulation_sampling_frequency_division(self.modulation_ptr()))

    @property
    def sampling_frequency(self) -> float:
        return float(Base().modulation_sampling_frequency(self.modulation_ptr()))

    @abstractmethod
    def modulation_ptr(self) -> ModulationPtr:
        pass
