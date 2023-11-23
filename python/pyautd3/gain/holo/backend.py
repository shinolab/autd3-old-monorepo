"""
File: backend.py
Project: holo
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 29/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

"""

from abc import ABCMeta, abstractmethod
from ctypes import Array, c_double

from pyautd3.gain.holo.constraint import EmissionConstraint
from pyautd3.native_methods.autd3capi_def import BackendPtr, GainPtr


class Backend(metaclass=ABCMeta):
    """Calculation backend."""

    _ptr: BackendPtr

    def __init__(self: "Backend", ptr: BackendPtr) -> None:
        self._ptr = ptr

    def _backend_ptr(self: "Backend") -> BackendPtr:
        return self._ptr

    @abstractmethod
    def _sdp(self: "Backend", foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        pass

    @abstractmethod
    def _sdp_with_alpha(self: "Backend", ptr: GainPtr, v: float) -> GainPtr:
        pass

    @abstractmethod
    def _sdp_with_repeat(self: "Backend", ptr: GainPtr, v: int) -> GainPtr:
        pass

    @abstractmethod
    def _sdp_with_lambda(self: "Backend", ptr: GainPtr, v: float) -> GainPtr:
        pass

    @abstractmethod
    def _sdp_with_constraint(self: "Backend", ptr: GainPtr, v: EmissionConstraint) -> GainPtr:
        pass

    @abstractmethod
    def _gs(self: "Backend", foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        pass

    @abstractmethod
    def _gs_with_repeat(self: "Backend", ptr: GainPtr, v: int) -> GainPtr:
        pass

    @abstractmethod
    def _gs_with_constraint(self: "Backend", ptr: GainPtr, v: EmissionConstraint) -> GainPtr:
        pass

    @abstractmethod
    def _gspat(self: "Backend", foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        pass

    @abstractmethod
    def _gspat_with_repeat(self: "Backend", ptr: GainPtr, v: int) -> GainPtr:
        pass

    @abstractmethod
    def _gspat_with_constraint(self: "Backend", ptr: GainPtr, v: EmissionConstraint) -> GainPtr:
        pass

    @abstractmethod
    def _naive(self: "Backend", foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        pass

    @abstractmethod
    def _naive_with_constraint(self: "Backend", ptr: GainPtr, v: EmissionConstraint) -> GainPtr:
        pass

    @abstractmethod
    def _lm(self: "Backend", foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        pass

    @abstractmethod
    def _lm_with_eps1(self: "Backend", ptr: GainPtr, v: float) -> GainPtr:
        pass

    @abstractmethod
    def _lm_with_eps2(self: "Backend", ptr: GainPtr, v: float) -> GainPtr:
        pass

    @abstractmethod
    def _lm_with_tau(self: "Backend", ptr: GainPtr, v: float) -> GainPtr:
        pass

    @abstractmethod
    def _lm_with_kmax(self: "Backend", ptr: GainPtr, v: int) -> GainPtr:
        pass

    @abstractmethod
    def _lm_with_initial(self: "Backend", ptr: GainPtr, v: Array[c_double], size: int) -> GainPtr:
        pass

    @abstractmethod
    def _lm_with_constraint(self: "Backend", ptr: GainPtr, v: EmissionConstraint) -> GainPtr:
        pass
