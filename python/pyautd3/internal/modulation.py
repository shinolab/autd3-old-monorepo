"""
File: modulation.py
Project: modulation
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""


from abc import ABCMeta, abstractmethod
from ctypes import create_string_buffer
from datetime import timedelta

from pyautd3.autd_error import AUTDError
from pyautd3.geometry import AUTD3, Geometry
from pyautd3.internal.datagram import Datagram
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import FPGA_SUB_CLK_FREQ, DatagramPtr, ModulationPtr

__all__ = []  # type: ignore[var-annotated]


class IModulation(Datagram, metaclass=ABCMeta):
    def __init__(self: "IModulation") -> None:
        super().__init__()

    def _datagram_ptr(self: "IModulation", _: Geometry) -> DatagramPtr:
        return Base().modulation_into_datagram(self._modulation_ptr())

    @property
    def sampling_frequency_division(self: "IModulation") -> int:
        return int(Base().modulation_sampling_frequency_division(self._modulation_ptr()))

    @property
    def sampling_frequency(self: "IModulation") -> float:
        return AUTD3.fpga_sub_clk_freq() / self.sampling_frequency_division

    def __len__(self: "IModulation") -> int:
        err = create_string_buffer(256)
        n = int(Base().modulation_size(self._modulation_ptr(), err))
        if n < 0:
            raise AUTDError(err)
        return n

    @abstractmethod
    def _modulation_ptr(self: "IModulation") -> ModulationPtr:
        pass


class IModulationWithFreqDiv(IModulation):
    _freq_div: int | None

    def __init__(self: "IModulationWithFreqDiv") -> None:
        super().__init__()
        self._freq_div = None

    def with_sampling_frequency_division(self: "IModulationWithFreqDiv", div: int) -> "IModulationWithFreqDiv":
        """Set sampling frequency division.

        Arguments:
        ---------
            div: Sampling frequency division.
                The sampling frequency will be `pyautd3.AUTD3.fpga_sub_clk_freq()` / `div`.
        """
        self._freq_div = div
        return self

    def with_sampling_frequency(self: "IModulationWithFreqDiv", freq: float) -> "IModulationWithFreqDiv":
        """Set sampling frequency.

        Arguments:
        ---------
            freq: Sampling frequency.
                The sampling frequency closest to `freq` from the possible sampling frequencies is set.
        """
        div = int(FPGA_SUB_CLK_FREQ / freq)
        return self.with_sampling_frequency_division(div)

    def with_sampling_period(self: "IModulationWithFreqDiv", period: timedelta) -> "IModulationWithFreqDiv":
        """Set sampling period.

        Arguments:
        ---------
            period: Sampling period.
                The sampling period closest to `period` from the possible sampling periods is set.
        """
        return self.with_sampling_frequency_division(int(FPGA_SUB_CLK_FREQ / 1000000000.0 * (period.total_seconds() * 1000.0 * 1000.0 * 1000.0)))
