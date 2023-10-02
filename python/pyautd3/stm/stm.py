'''
File: stm.py
Project: stm
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 02/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

'''

from abc import ABCMeta, abstractmethod
from datetime import timedelta
import functools
from typing import Optional, List, Tuple, Union
from collections.abc import Iterable
import ctypes

import numpy as np

from pyautd3.autd import Datagram
from pyautd3.geometry import Geometry
from pyautd3.gain.gain import IGain
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import (
    GainPtr,
    GainSTMMode,
    DatagramPtr,
    STMPropsPtr,
)


class STM(Datagram, metaclass=ABCMeta):
    _freq: Optional[float]
    _sampling_freq: Optional[float]
    _sampling_freq_div: Optional[int]
    _sampling_period: Optional[timedelta]
    _start_idx: int
    _finish_idx: int

    def __init__(
        self,
        freq: Optional[float],
        sampling_freq: Optional[float],
        sampling_freq_div: Optional[int],
        sampling_period: Optional[timedelta]
    ):
        super().__init__()
        self._freq = freq
        self._sampling_freq = sampling_freq
        self._sampling_freq_div = sampling_freq_div
        self._sampling_period = sampling_period
        self._start_idx = -1
        self._finish_idx = -1

    @property
    def start_idx(self) -> Optional[int]:
        """Start index of STM

        """

        idx = int(Base().stm_props_start_idx(self._props()))
        if idx < 0:
            return None
        return idx

    @property
    def finish_idx(self) -> Optional[int]:
        """Finish index of STM

        """

        idx = int(Base().stm_props_finish_idx(self._props()))
        if idx < 0:
            return None
        return idx

    def _props(self) -> STMPropsPtr:
        ptr: STMPropsPtr
        if self._freq is not None:
            ptr = Base().stm_props(self._freq)
        if self._sampling_freq is not None:
            ptr = Base().stm_props_with_sampling_freq(self._sampling_freq)
        if self._sampling_freq_div is not None:
            ptr = Base().stm_props_with_sampling_freq_div(self._sampling_freq_div)
        if self._sampling_period is not None:
            ptr = Base().stm_props_with_sampling_period(int(self._sampling_period.total_seconds() * 1000 * 1000 * 1000))
        ptr = Base().stm_props_with_start_idx(ptr, self._start_idx)
        ptr = Base().stm_props_with_finish_idx(ptr, self._finish_idx)
        return ptr

    def _frequency_from_size(self, size: int) -> float:
        return float(Base().stm_props_frequency(self._props(), size))

    def _sampling_frequency_from_size(self, size: int) -> float:
        return float(Base().stm_props_sampling_frequency(self._props(), size))

    def _sampling_frequency_division_from_size(self, size: int) -> int:
        return int(Base().stm_props_sampling_frequency_division(self._props(), size))

    def _sampling_period_from_size(self, size: int) -> timedelta:
        return timedelta(microseconds=int(Base().stm_props_sampling_period(self._props(), size)) / 1000)

    @abstractmethod
    def ptr(self, _: Geometry) -> DatagramPtr:
        pass


class FocusSTM(STM):
    """FocusSTM is an STM for moving a single focal point.

    The sampling timing is determined by hardware, thus the sampling time is precise.

    FocusSTM has following restrictions:
    - The maximum number of sampling points is 65536.
    - The sampling frequency is `pyautd3.AUTD3.fpga_sub_clk_freq()`/N, where `N` is a 32-bit unsigned integer.
    """

    _points: List[float]
    _duty_shifts: List[int]

    def __init__(
        self,
        freq: Optional[float],
        sampling_freq: Optional[float] = None,
        sampling_freq_div: Optional[int] = None,
        sampling_period: Optional[timedelta] = None,
    ):
        """Constructor

        Arguments:
        - `freq` - Frequency of STM [Hz]. The frequency closest to `freq` from the possible frequencies is set.
        """

        super().__init__(freq, sampling_freq, sampling_freq_div, sampling_period)
        self._points = []
        self._duty_shifts = []

    def ptr(self, _: Geometry) -> DatagramPtr:
        points = np.ctypeslib.as_ctypes(np.array(self._points).astype(ctypes.c_double))
        shifts = np.ctypeslib.as_ctypes(
            np.array(self._duty_shifts).astype(ctypes.c_uint8)
        )
        return Base().stm_focus(self._props(), points, shifts, len(self._duty_shifts))

    @staticmethod
    def with_sampling_frequency(sampling_freq: float) -> "FocusSTM":
        """Constructor

        Arguments:
        - `sampling_freq` - Sampling frequency [Hz]. The sampling frequency closest to `sampling_freq` from the possible frequencies is set.
        """

        return FocusSTM(None, sampling_freq, None, None)

    @staticmethod
    def with_sampling_frequency_division(sampling_freq_div: int) -> "FocusSTM":
        """Constructor

        Arguments:
        - `sampling_freq_div` - Sampling frequency division.
        """

        return FocusSTM(None, None, sampling_freq_div, None)

    @staticmethod
    def with_sampling_period(sampling_period: timedelta) -> "FocusSTM":
        """Constructor

        Arguments:
        - `sampling_period` - Sampling period. The sampling period closest to `sampling_period` from the possible periods is set.
        """

        return FocusSTM(None, None, None, sampling_period)

    def add_focus(self, point: np.ndarray, duty_shift: int = 0) -> "FocusSTM":
        """Add focus

        Arguments:
        - `point` - Focal point
        `duty_shift` - Duty shift. Duty ratio of ultrasound will be `50% >> shift`.
        """

        assert len(point) == 3

        self._points.append(point[0])
        self._points.append(point[1])
        self._points.append(point[2])
        self._duty_shifts.append(duty_shift)
        return self

    def add_foci_from_iter(
        self, iterable: Union[Iterable[np.ndarray], Iterable[Tuple[np.ndarray, int]]]
    ) -> "FocusSTM":
        """Add foci

        Arguments:
        - `iterable` - Iterable of focal points or tuples of focal points and duty shifts.
        """

        return functools.reduce(
            lambda acc, x: acc.add_focus(x)
            if isinstance(x, np.ndarray)
            else acc.add_focus(x[0], x[1]),
            iterable,
            self,
        )

    @property
    def frequency(self) -> float:
        return self._frequency_from_size(len(self._duty_shifts))

    @property
    def sampling_frequency(self) -> float:
        return self._sampling_frequency_from_size(len(self._duty_shifts))

    @property
    def sampling_frequency_division(self) -> int:
        return self._sampling_frequency_division_from_size(len(self._duty_shifts))

    @property
    def sampling_period(self) -> timedelta:
        return self._sampling_period_from_size(len(self._duty_shifts))

    def with_start_idx(self, value: Optional[int]) -> "FocusSTM":
        """Set the start index of STM

        Arguments:
        - `value` - Start index of STM.
        """

        self._start_idx = -1 if value is None else value
        return self

    def with_finish_idx(self, value: Optional[int]) -> "FocusSTM":
        """Set the finish index of STM

        Arguments:
        - `value` - Finish index of STM.
        """

        self._finish_idx = -1 if value is None else value
        return self


class GainSTM(STM):
    _gains: List[IGain]
    _mode: GainSTMMode

    def __init__(
        self,
        freq: Optional[float],
        sampling_freq: Optional[float] = None,
        sampling_freq_div: Optional[int] = None,
        sampling_period: Optional[timedelta] = None,
    ):
        super().__init__(freq, sampling_freq, sampling_freq_div, sampling_period)
        self._gains = []
        self._mode = GainSTMMode.PhaseDutyFull

    def ptr(self, geometry: Geometry) -> DatagramPtr:
        gains: np.ndarray = np.ndarray(len(self._gains), dtype=GainPtr)
        for i, g in enumerate(self._gains):
            gains[i]["_0"] = g.gain_ptr(geometry)._0
        return Base().stm_gain(self._props(),
                               gains.ctypes.data_as(ctypes.POINTER(GainPtr)),  # type: ignore
                               len(self._gains), self._mode)

    @staticmethod
    def with_sampling_frequency(sampling_freq: float) -> "GainSTM":
        """Constructor

        Arguments:
        - `sampling_freq` - Sampling frequency [Hz]. The sampling frequency closest to `sampling_freq` from the possible frequencies is set.
        """

        return GainSTM(None, sampling_freq, None)

    @staticmethod
    def with_sampling_frequency_division(sampling_freq_div: int) -> "GainSTM":
        """Constructor

        Arguments:
        - `sampling_freq_div` - Sampling frequency division.
        """

        return GainSTM(None, None, sampling_freq_div)

    @staticmethod
    def with_sampling_period(sampling_period: timedelta) -> "GainSTM":
        """Constructor

        Arguments:
        - `sampling_period` - Sampling period. The sampling period closest to `sampling_period` from the possible periods is set.
        """

        return GainSTM(None, None, None, sampling_period)

    def add_gain(self, gain: IGain) -> "GainSTM":
        """Add gain

        Arguments:
        - `gain` - Gain
        """

        self._gains.append(gain)
        return self

    def add_gains_from_iter(self, iter: Iterable[IGain]) -> "GainSTM":
        """Add gains

        Arguments:
        - `iter` - Iterable of gains
        """

        self._gains.extend(iter)
        return self

    @property
    def frequency(self) -> float:
        return self._frequency_from_size(len(self._gains))

    @property
    def sampling_frequency(self) -> float:
        return self._sampling_frequency_from_size(len(self._gains))

    @property
    def sampling_frequency_division(self) -> int:
        return self._sampling_frequency_division_from_size(len(self._gains))

    @property
    def sampling_period(self) -> timedelta:
        return self._sampling_period_from_size(len(self._gains))

    def with_mode(self, mode: GainSTMMode) -> "GainSTM":
        """Set GainSTMMode

        Arguments:
        - `mode` - GainSTMMode
        """

        self._mode = mode
        return self

    def with_start_idx(self, value: Optional[int]) -> "GainSTM":
        """Set the start index of STM

        Arguments:
        - `value` - Start index of STM.
        """

        self._start_idx = -1 if value is None else value
        return self

    def with_finish_idx(self, value: Optional[int]) -> "GainSTM":
        """Set the finish index of STM

        Arguments:
        - `value` - Finish index of STM.
        """

        self._finish_idx = -1 if value is None else value
        return self
