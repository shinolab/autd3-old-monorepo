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
from collections.abc import Callable
from ctypes import create_string_buffer
from datetime import timedelta
from typing import TYPE_CHECKING, TypeVar

from pyautd3.autd_error import AUTDError
from pyautd3.geometry import AUTD3, Geometry
from pyautd3.internal.datagram import Datagram
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import FPGA_CLK_FREQ, DatagramPtr, ModulationPtr

if TYPE_CHECKING:
    from pyautd3.modulation.cache import Cache
    from pyautd3.modulation.fir import BPF, BSF, HPF, LPF
    from pyautd3.modulation.radiation_pressure import RadiationPressure
    from pyautd3.modulation.transform import Transform

__all__ = []  # type: ignore[var-annotated]

M = TypeVar("M", bound="IModulation")
MF = TypeVar("MF", bound="IModulationWithFreqDiv")


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
        return AUTD3.fpga_clk_freq() / self.sampling_frequency_division

    def __len__(self: "IModulation") -> int:
        err = create_string_buffer(256)
        n = int(Base().modulation_size(self._modulation_ptr(), err))
        if n < 0:
            raise AUTDError(err)
        return n

    @abstractmethod
    def _modulation_ptr(self: "IModulation") -> ModulationPtr:
        pass

    def with_cache(self: M) -> "Cache":  # type: ignore[empty-body]
        # This function is implemented in cache.py
        pass

    def with_transform(self: M, f: Callable[[int, float], float]) -> "Transform":  # type: ignore[empty-body]
        # This function is implemented in transform.py
        pass

    def with_radiation_pressure(self: M) -> "RadiationPressure":  # type: ignore[empty-body]
        # This function is implemented in radiation_pressure.py
        pass

    def with_low_pass(self: M, n_taps: int, cutoff: float) -> "LPF":  # type: ignore[empty-body]
        # This function is implemented in fir.py
        pass

    def with_high_pass(self: M, n_taps: int, cutoff: float) -> "HPF":  # type: ignore[empty-body]
        # This function is implemented in fir.py
        pass

    def with_band_pass(self: M, n_taps: int, f_low: float, f_high: float) -> "BPF":  # type: ignore[empty-body]
        # This function is implemented in fir.py
        pass

    def with_band_stop(self: M, n_taps: int, f_low: float, f_high: float) -> "BSF":  # type: ignore[empty-body]
        # This function is implemented in fir.py
        pass


class IModulationWithFreqDiv(IModulation):
    _freq_div: int | None

    def __init__(self: "IModulationWithFreqDiv") -> None:
        super().__init__()
        self._freq_div = None

    def with_sampling_frequency_division(self: MF, div: int) -> MF:
        """Set sampling frequency division.

        Arguments:
        ---------
            div: Sampling frequency division.
                The sampling frequency will be `pyautd3.AUTD3.fpga_clk_freq()` / `div`.
        """
        self._freq_div = div
        return self

    def with_sampling_frequency(self: MF, freq: float) -> MF:
        """Set sampling frequency.

        Arguments:
        ---------
            freq: Sampling frequency.
                The sampling frequency closest to `freq` from the possible sampling frequencies is set.
        """
        div = int(FPGA_CLK_FREQ / freq)
        return self.with_sampling_frequency_division(div)

    def with_sampling_period(self: MF, period: timedelta) -> MF:
        """Set sampling period.

        Arguments:
        ---------
            period: Sampling period.
                The sampling period closest to `period` from the possible sampling periods is set.
        """
        return self.with_sampling_frequency_division(int(FPGA_CLK_FREQ / 1000000000.0 * (period.total_seconds() * 1000.0 * 1000.0 * 1000.0)))
