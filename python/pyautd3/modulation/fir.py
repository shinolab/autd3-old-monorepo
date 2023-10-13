'''
File: fir.py
Project: modulation
Created Date: 12/10/2023
Author: Shun Suzuki
-----
Last Modified: 13/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''


from pyautd3.native_methods.autd3capi import NativeMethods as Base
from pyautd3.native_methods.autd3capi_def import ModulationPtr

from pyautd3.internal.modulation import IModulation


class LPF(IModulation):
    _m: IModulation
    _n_taps: int
    _cutoff: float

    def __init__(self, m: IModulation, n_taps: int, cutoff: float):
        self._m = m
        self._n_taps = n_taps
        self._cutoff = cutoff

    def modulation_ptr(self) -> ModulationPtr:
        return Base().modulation_with_low_pass(self._m.modulation_ptr(), self._n_taps, self._cutoff)


class HPF(IModulation):
    _m: IModulation
    _n_taps: int
    _cutoff: float

    def __init__(self, m: IModulation, n_taps: int, cutoff: float):
        self._m = m
        self._n_taps = n_taps
        self._cutoff = cutoff

    def modulation_ptr(self) -> ModulationPtr:
        return Base().modulation_with_high_pass(self._m.modulation_ptr(), self._n_taps, self._cutoff)


class BPF(IModulation):
    _m: IModulation
    _n_taps: int
    _f_low: float
    _f_high: float

    def __init__(self, m: IModulation, n_taps: int, f_low: float, f_high: float):
        self._m = m
        self._n_taps = n_taps
        self._f_low = f_low
        self._f_high = f_high

    def modulation_ptr(self) -> ModulationPtr:
        return Base().modulation_with_band_pass(self._m.modulation_ptr(), self._n_taps, self._f_low, self._f_high)


class BSF(IModulation):
    _m: IModulation
    _n_taps: int
    _f_low: float
    _f_high: float

    def __init__(self, m: IModulation, n_taps: int, f_low: float, f_high: float):
        self._m = m
        self._n_taps = n_taps
        self._f_low = f_low
        self._f_high = f_high

    def modulation_ptr(self) -> ModulationPtr:
        return Base().modulation_with_band_stop(self._m.modulation_ptr(), self._n_taps, self._f_low, self._f_high)


def __with_low_pass(self, n_taps: int, cutoff: float):
    return LPF(self, n_taps, cutoff)


def __with_high_pass(self, n_taps: int, cutoff: float):
    return HPF(self, n_taps, cutoff)


def __with_band_pass(self, n_taps: int, f_low: float, f_high: float):
    return BPF(self, n_taps, f_low, f_high)


def __with_band_stop(self, n_taps: int, f_low: float, f_high: float):
    return BSF(self, n_taps, f_low, f_high)


IModulation.with_low_pass = __with_low_pass  # type: ignore
IModulation.with_high_pass = __with_high_pass  # type: ignore
IModulation.with_band_pass = __with_band_pass  # type: ignore
IModulation.with_band_stop = __with_band_stop  # type: ignore
