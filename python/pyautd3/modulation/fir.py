"""
File: fir.py
Project: modulation
Created Date: 12/10/2023
Author: Shun Suzuki
-----
Last Modified: 13/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


from typing import Generic, TypeVar

from pyautd3.internal.modulation import IModulation
from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import ModulationPtr

M = TypeVar("M", bound=IModulation)


class LPF(IModulation, Generic[M]):
    """Low pass filter."""

    _m: M
    _n_taps: int
    _cutoff: float

    def __init__(self: "LPF", m: M, n_taps: int, cutoff: float) -> None:
        self._m = m
        self._n_taps = n_taps
        self._cutoff = cutoff

    def _modulation_ptr(self: "LPF") -> ModulationPtr:
        return Base().modulation_with_low_pass(self._m._modulation_ptr(), self._n_taps, self._cutoff)


class HPF(IModulation, Generic[M]):
    """High pass filter."""

    _m: M
    _n_taps: int
    _cutoff: float

    def __init__(self: "HPF", m: M, n_taps: int, cutoff: float) -> None:
        self._m = m
        self._n_taps = n_taps
        self._cutoff = cutoff

    def _modulation_ptr(self: "HPF") -> ModulationPtr:
        return Base().modulation_with_high_pass(self._m._modulation_ptr(), self._n_taps, self._cutoff)


class BPF(IModulation, Generic[M]):
    """Band pass filter."""

    _m: M
    _n_taps: int
    _f_low: float
    _f_high: float

    def __init__(self: "BPF", m: M, n_taps: int, f_low: float, f_high: float) -> None:
        self._m = m
        self._n_taps = n_taps
        self._f_low = f_low
        self._f_high = f_high

    def _modulation_ptr(self: "BPF") -> ModulationPtr:
        return Base().modulation_with_band_pass(self._m._modulation_ptr(), self._n_taps, self._f_low, self._f_high)


class BSF(IModulation, Generic[M]):
    """Band stop filter."""

    _m: M
    _n_taps: int
    _f_low: float
    _f_high: float

    def __init__(self: "BSF", m: M, n_taps: int, f_low: float, f_high: float) -> None:
        self._m = m
        self._n_taps = n_taps
        self._f_low = f_low
        self._f_high = f_high

    def _modulation_ptr(self: "BSF") -> ModulationPtr:
        return Base().modulation_with_band_stop(self._m._modulation_ptr(), self._n_taps, self._f_low, self._f_high)


def __with_low_pass(self: M, n_taps: int, cutoff: float) -> LPF:
    return LPF(self, n_taps, cutoff)


def __with_high_pass(self: M, n_taps: int, cutoff: float) -> HPF:
    return HPF(self, n_taps, cutoff)


def __with_band_pass(self: M, n_taps: int, f_low: float, f_high: float) -> BPF:
    return BPF(self, n_taps, f_low, f_high)


def __with_band_stop(self: M, n_taps: int, f_low: float, f_high: float) -> BSF:
    return BSF(self, n_taps, f_low, f_high)


IModulation.with_low_pass = __with_low_pass  # type: ignore[method-assign]
IModulation.with_high_pass = __with_high_pass  # type: ignore[method-assign]
IModulation.with_band_pass = __with_band_pass  # type: ignore[method-assign]
IModulation.with_band_stop = __with_band_stop  # type: ignore[method-assign]
