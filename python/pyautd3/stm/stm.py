"""
File: stm.py
Project: stm
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 10/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""

import ctypes
import functools
from abc import ABCMeta
from collections.abc import Iterable
from ctypes import c_uint8
from datetime import timedelta

import numpy as np
from numpy.typing import ArrayLike

from pyautd3.emit_intensity import EmitIntensity
from pyautd3.gain.gain import IGain
from pyautd3.geometry import Geometry
from pyautd3.internal.datagram import Datagram
from pyautd3.internal.utils import _validate_ptr, _validate_sampling_config
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import (
    DatagramPtr,
    GainPtr,
    GainSTMMode,
    STMPropsPtr,
)
from pyautd3.sampling_config import SamplingConfiguration

__all__ = ["FocusSTM", "GainSTM"]  # type: ignore[var-annotated]


class STM(Datagram, metaclass=ABCMeta):
    _freq: float | None
    _period: timedelta | None
    _sampling_config: SamplingConfiguration | None
    _start_idx: int
    _finish_idx: int

    def __init__(
        self: "STM",
        freq: float | None,
        period: timedelta | None,
        sampling_config: SamplingConfiguration | None,
    ) -> None:
        super().__init__()
        self._freq = freq
        self._period = period
        self._sampling_config = sampling_config
        self._start_idx = -1
        self._finish_idx = -1

    @property
    def start_idx(self: "STM") -> int | None:
        """Start index of STM."""
        idx = int(Base().stm_props_start_idx(self._props()))
        if idx < 0:
            return None
        return idx

    @property
    def finish_idx(self: "STM") -> int | None:
        """Finish index of STM."""
        idx = int(Base().stm_props_finish_idx(self._props()))
        if idx < 0:
            return None
        return idx

    def _props(self: "STM") -> STMPropsPtr:
        ptr: STMPropsPtr
        if self._freq is not None:
            ptr = Base().stm_props_new(self._freq)
        if self._period is not None:
            ptr = Base().stm_props_from_period(int(self._period.total_seconds() * 1000 * 1000 * 1000))
        if self._sampling_config is not None:
            ptr = Base().stm_props_from_sampling_config(self._sampling_config._internal)
        ptr = Base().stm_props_with_start_idx(ptr, self._start_idx)
        return Base().stm_props_with_finish_idx(ptr, self._finish_idx)

    def _frequency_from_size(self: "STM", size: int) -> float:
        return float(Base().stm_props_frequency(self._props(), size))

    def _period_from_size(self: "STM", size: int) -> timedelta:
        return timedelta(seconds=int(Base().stm_props_period(self._props(), size)) / 1000.0 / 1000.0 / 1000.0)

    def _sampling_config_from_size(self: "STM", size: int) -> SamplingConfiguration:
        return SamplingConfiguration.__private_new__(_validate_sampling_config(Base().stm_props_sampling_config(self._props(), size)))


class FocusSTM(STM):
    """FocusSTM is an STM for moving a single focal point.

    The sampling timing is determined by hardware, thus the sampling time is precise.

    FocusSTM has following restrictions:
    - The maximum number of sampling points is 65536.
    - The sampling frequency is `pyautd3.AUTD3.fpga_clk_freq()`/N, where `N` is a 32-bit unsigned integer.
    """

    _points: list[float]
    _intensities: list[EmitIntensity]

    def __init__(
        self: "FocusSTM",
        freq: float | None,
        *,
        period: timedelta | None = None,
        sampling_config: SamplingConfiguration | None = None,
    ) -> None:
        """Constructor.

        Arguments:
        ---------
            freq: Frequency of STM [Hz]. The frequency closest to `freq` from the possible frequencies is set.
            period: only for internal use.
            sampling_config: only for internal use.
        """
        super().__init__(freq, period, sampling_config)
        self._points = []
        self._intensities = []

    def _datagram_ptr(self: "FocusSTM", _: Geometry) -> DatagramPtr:
        points = np.ctypeslib.as_ctypes(np.array(self._points).astype(ctypes.c_double))
        intensities = np.fromiter((i.value for i in self._intensities), dtype=c_uint8)  # type: ignore[type-var,call-overload]
        return _validate_ptr(
            Base().stm_focus(
                self._props(),
                points,
                intensities.ctypes.data_as(ctypes.POINTER(c_uint8)),  # type: ignore[arg-type]
                len(self._intensities),
            ),
        )

    @staticmethod
    def from_period(period: timedelta) -> "FocusSTM":
        """Constructor.

        Arguments:
        ---------
            period: Period.
        """
        return FocusSTM(None, period=period)

    @staticmethod
    def from_sampling_config(config: SamplingConfiguration) -> "FocusSTM":
        """Constructor.

        Arguments:
        ---------
            config: Sampling configuration
        """
        return FocusSTM(
            None,
            sampling_config=config,
        )

    def add_focus(self: "FocusSTM", point: ArrayLike, intensity: EmitIntensity | None = None) -> "FocusSTM":
        """Add focus.

        Arguments:
        ---------
            point: Focal point
            intensity: Emission intensity
        """
        point = np.array(point)
        self._points.append(point[0])
        self._points.append(point[1])
        self._points.append(point[2])
        self._intensities.append(intensity if intensity is not None else EmitIntensity(0xFF))
        return self

    def add_foci_from_iter(self: "FocusSTM", iterable: Iterable[np.ndarray] | Iterable[tuple[np.ndarray, int]]) -> "FocusSTM":
        """Add foci.

        Arguments:
        ---------
            iterable: Iterable of focal points or tuples of focal points and duty shifts.
        """
        return functools.reduce(
            lambda acc, x: acc.add_focus(x) if isinstance(x, np.ndarray) else acc.add_focus(x[0], x[1]),
            iterable,
            self,
        )

    @property
    def frequency(self: "FocusSTM") -> float:
        """Frequency [Hz]."""
        return self._frequency_from_size(len(self._intensities))

    @property
    def period(self: "FocusSTM") -> timedelta:
        """Period."""
        return self._period_from_size(len(self._intensities))

    @property
    def sampling_config(self: "FocusSTM") -> SamplingConfiguration:
        """Sampling frequency [Hz]."""
        return self._sampling_config_from_size(len(self._intensities))

    def with_start_idx(self: "FocusSTM", value: int | None) -> "FocusSTM":
        """Set the start index of STM.

        Arguments:
        ---------
            value: Start index of STM.
        """
        self._start_idx = -1 if value is None else value
        return self

    def with_finish_idx(self: "FocusSTM", value: int | None) -> "FocusSTM":
        """Set the finish index of STM.

        Arguments:
        ---------
            value: Finish index of STM.
        """
        self._finish_idx = -1 if value is None else value
        return self


class GainSTM(STM):
    """GainSTM is an STM for moving any Gain."""

    _gains: list[IGain]
    _mode: GainSTMMode

    def __init__(
        self: "GainSTM",
        freq: float | None,
        *,
        period: timedelta | None = None,
        sampling_config: SamplingConfiguration | None = None,
    ) -> None:
        """Constructor.

        Arguments:
        ---------
            freq: Frequency of STM [Hz]. The frequency closest to `freq` from the possible frequencies is set.
            period: only for internal use.
            sampling_config: only for internal use.
        """
        super().__init__(freq, period, sampling_config)
        self._gains = []
        self._mode = GainSTMMode.PhaseIntensityFull

    def _datagram_ptr(self: "GainSTM", geometry: Geometry) -> DatagramPtr:
        gains: np.ndarray = np.ndarray(len(self._gains), dtype=GainPtr)
        for i, g in enumerate(self._gains):
            gains[i]["_0"] = g._gain_ptr(geometry)._0
        return _validate_ptr(
            Base().stm_gain(
                self._props(),
                gains.ctypes.data_as(ctypes.POINTER(GainPtr)),  # type: ignore[arg-type]
                len(self._gains),
                self._mode,
            ),
        )

    @staticmethod
    def from_sampling_config(config: SamplingConfiguration) -> "GainSTM":
        """Constructor.

        Arguments:
        ---------
            config: Sampling configuration
        """
        return GainSTM(None, sampling_config=config)

    @staticmethod
    def from_period(period: timedelta) -> "GainSTM":
        """Constructor.

        Arguments:
        ---------
            period: Period.
        """
        return GainSTM(None, period=period)

    def add_gain(self: "GainSTM", gain: IGain) -> "GainSTM":
        """Add gain.

        Arguments:
        ---------
            gain: Gain
        """
        self._gains.append(gain)
        return self

    def add_gains_from_iter(self: "GainSTM", iterable: Iterable[IGain]) -> "GainSTM":
        """Add gains.

        Arguments:
        ---------
            iterable: Iterable of gains
        """
        self._gains.extend(iterable)
        return self

    @property
    def frequency(self: "GainSTM") -> float:
        """Frequency [Hz]."""
        return self._frequency_from_size(len(self._gains))

    @property
    def period(self: "GainSTM") -> timedelta:
        """Period."""
        return self._period_from_size(len(self._gains))

    @property
    def sampling_config(self: "GainSTM") -> SamplingConfiguration:
        """Sampling configuration."""
        return self._sampling_config_from_size(len(self._gains))

    def with_mode(self: "GainSTM", mode: GainSTMMode) -> "GainSTM":
        """Set GainSTMMode.

        Arguments:
        ---------
            mode: GainSTMMode
        """
        self._mode = mode
        return self

    def with_start_idx(self: "GainSTM", value: int | None) -> "GainSTM":
        """Set the start index of STM.

        Arguments:
        ---------
            value: Start index of STM.
        """
        self._start_idx = -1 if value is None else value
        return self

    def with_finish_idx(self: "GainSTM", value: int | None) -> "GainSTM":
        """Set the finish index of STM.

        Arguments:
        ---------
            value: Finish index of STM.
        """
        self._finish_idx = -1 if value is None else value
        return self
