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

from abc import ABCMeta, abstractmethod
import functools
from typing import Optional, List, Tuple, Union
from collections.abc import Iterable
import ctypes

import numpy as np

from pyautd3.autd import Body
from pyautd3.geometry import Geometry
from pyautd3.gain.gain import IGain
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import (
    GainSTMMode,
    DatagramBodyPtr,
    STMPropsPtr,
)


class STM(Body, metaclass=ABCMeta):
    _freq: Optional[float]
    _sampling_freq: Optional[float]
    _sampling_freq_div: Optional[int]
    _start_idx: int
    _finish_idx: int

    def __init__(
        self,
        freq: Optional[float],
        sampling_freq: Optional[float],
        sampling_freq_div: Optional[int],
    ):
        super().__init__()
        self._freq = freq
        self._sampling_freq = sampling_freq
        self._sampling_freq_div = sampling_freq_div
        self._start_idx = -1
        self._finish_idx = -1

    @property
    def start_idx(self) -> Optional[int]:
        idx = int(Base().stm_props_start_idx(self.props()))
        if idx < 0:
            return None
        return idx

    @property
    def finish_idx(self) -> Optional[int]:
        idx = int(Base().stm_props_finish_idx(self.props()))
        if idx < 0:
            return None
        return idx

    def props(self) -> STMPropsPtr:
        ptr: STMPropsPtr
        if self._freq is not None:
            ptr = Base().stm_props(self._freq)
        if self._sampling_freq is not None:
            ptr = Base().stm_props_with_sampling_freq(self._sampling_freq)
        if self._sampling_freq_div is not None:
            ptr = Base().stm_props_with_sampling_freq_div(self._sampling_freq_div)
        ptr = Base().stm_props_with_start_idx(ptr, self._start_idx)
        ptr = Base().stm_props_with_finish_idx(ptr, self._finish_idx)
        return ptr

    def frequency_from_size(self, size: int) -> float:
        return float(Base().stm_props_frequency(self.props(), size))

    def sampling_frequency_from_size(self, size: int) -> float:
        return float(Base().stm_props_sampling_frequency(self.props(), size))

    def sampling_frequency_division_from_size(self, size: int) -> int:
        return int(Base().stm_props_sampling_frequency_division(self.props(), size))

    @abstractmethod
    def ptr(self, geometry: Geometry) -> DatagramBodyPtr:
        pass


class FocusSTM(STM):
    _points: List[float]
    _duty_shifts: List[int]

    def __init__(
        self,
        freq: Optional[float],
        sampling_freq: Optional[float] = None,
        sampling_freq_div: Optional[int] = None,
    ):
        super().__init__(freq, sampling_freq, sampling_freq_div)
        self._points = []
        self._duty_shifts = []

    def ptr(self, _: Geometry) -> DatagramBodyPtr:
        points = np.ctypeslib.as_ctypes(np.array(self._points).astype(ctypes.c_double))
        shifts = np.ctypeslib.as_ctypes(
            np.array(self._duty_shifts).astype(ctypes.c_uint8)
        )
        return Base().focus_stm(self.props(), points, shifts, len(self._duty_shifts))

    @staticmethod
    def with_sampling_frequency(sampling_freq: float) -> "FocusSTM":
        return FocusSTM(None, sampling_freq, None)

    @staticmethod
    def with_sampling_frequency_division(sampling_freq_div: int) -> "FocusSTM":
        return FocusSTM(None, None, sampling_freq_div)

    def add_focus(self, point: np.ndarray, duty_shift: int = 0) -> "FocusSTM":
        self._points.append(point[0])
        self._points.append(point[1])
        self._points.append(point[2])
        self._duty_shifts.append(duty_shift)
        return self

    def add_foci_from_iter(
        self, iterable: Union[Iterable[np.ndarray], Iterable[Tuple[np.ndarray, int]]]
    ) -> "FocusSTM":
        return functools.reduce(
            lambda acc, x: acc.add_focus(x)
            if isinstance(x, np.ndarray)
            else acc.add_focus(x[0], x[1]),
            iterable,
            self,
        )

    @property
    def frequency(self) -> float:
        return self.frequency_from_size(len(self._duty_shifts))

    @property
    def sampling_frequency(self) -> float:
        return self.sampling_frequency_from_size(len(self._duty_shifts))

    @property
    def sampling_frequency_division(self) -> int:
        return self.sampling_frequency_division_from_size(len(self._duty_shifts))

    def with_start_idx(self, value: Optional[int]) -> "FocusSTM":
        self._start_idx = -1 if value is None else value
        return self

    def with_finish_idx(self, value: Optional[int]) -> "FocusSTM":
        self._finish_idx = -1 if value is None else value
        return self


class GainSTM(STM):
    _gains: List[IGain]
    _mode: Optional[GainSTMMode]

    def __init__(
        self,
        freq: Optional[float],
        sampling_freq: Optional[float] = None,
        sampling_freq_div: Optional[int] = None,
    ):
        super().__init__(freq, sampling_freq, sampling_freq_div)
        self._gains = []
        self._mode = None

    def ptr(self, geometry: Geometry) -> DatagramBodyPtr:
        return functools.reduce(
            lambda acc, gain: Base().gain_stm_add_gain(acc, gain.gain_ptr(geometry)),
            self._gains,
            Base().gain_stm(self.props())
            if self._mode is None
            else Base().gain_stm_with_mode(self.props(), self._mode),
        )

    @staticmethod
    def from_sampling_frequency(sampling_freq: float) -> "GainSTM":
        return GainSTM(None, sampling_freq, None)

    @staticmethod
    def from_sampling_frequency_division(sampling_freq_div: int) -> "GainSTM":
        return GainSTM(None, None, sampling_freq_div)

    def add_gain(self, gain: IGain) -> "GainSTM":
        self._gains.append(gain)
        return self

    def add_gains_from_iter(self, iter: Iterable[IGain]) -> "GainSTM":
        self._gains.extend(iter)
        return self

    @property
    def frequency(self) -> float:
        return self.frequency_from_size(len(self._gains))

    @property
    def sampling_frequency(self) -> float:
        return self.sampling_frequency_from_size(len(self._gains))

    @property
    def sampling_frequency_division(self) -> int:
        return self.sampling_frequency_division_from_size(len(self._gains))

    def with_mode(self, mode: GainSTMMode) -> "GainSTM":
        self._mode = mode
        return self

    def with_start_idx(self, value: Optional[int]) -> "GainSTM":
        self._start_idx = -1 if value is None else value
        return self

    def with_finish_idx(self, value: Optional[int]) -> "GainSTM":
        self._finish_idx = -1 if value is None else value
        return self
