'''
File: modulation.py
Project: modulation
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

'''


from abc import ABCMeta, abstractmethod
from ctypes import create_string_buffer
from datetime import timedelta
from typing import Optional

from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import FPGA_SUB_CLK_FREQ, DatagramPtr, ModulationPtr

from pyautd3.autd_error import AUTDError
from pyautd3.autd import Datagram
from pyautd3.geometry import Geometry, AUTD3


class IModulation(Datagram, metaclass=ABCMeta):
    def __init__(self):
        super().__init__()

    def _ptr(self, _: Geometry) -> DatagramPtr:
        return Base().modulation_into_datagram(self.modulation_ptr())

    @property
    def sampling_frequency_division(self) -> int:
        return int(Base().modulation_sampling_frequency_division(self.modulation_ptr()))

    @property
    def sampling_frequency(self) -> float:
        return AUTD3.fpga_sub_clk_freq() / self.sampling_frequency_division

    def __len__(self):
        err = create_string_buffer(256)
        n = Base().modulation_size(self.modulation_ptr(), err)
        if n < 0:
            raise AUTDError(err)
        return int(n)

    @abstractmethod
    def modulation_ptr(self) -> ModulationPtr:
        pass


class IModulationWithFreqDiv(IModulation):
    _freq_div: Optional[int]

    def __init__(self):
        super().__init__()
        self._freq_div = None

    def with_sampling_frequency_division(self, div: int):
        """Set sampling frequency division

        Arguments:
        - `div` - Sampling frequency division.
                The sampling frequency will be `pyautd3.AUTD3.fpga_sub_clk_freq()` / `div`.
        """

        self._freq_div = div
        return self

    def with_sampling_frequency(self, freq: float):
        """Set sampling frequency

        Arguments:
        - `freq` - Sampling frequency.
                The sampling frequency closest to `freq` from the possible sampling frequencies is set.
        """

        div = int(FPGA_SUB_CLK_FREQ / freq)
        return self.with_sampling_frequency_division(div)

    def with_sampling_period(self, period: timedelta):
        """Set sampling period

        Arguments:
        - `period` - Sampling period.
                The sampling period closest to `period` from the possible sampling periods is set.
        """

        return self.with_sampling_frequency_division(int(FPGA_SUB_CLK_FREQ / 1000000000. * (period.total_seconds() * 1000. * 1000. * 1000.)))
