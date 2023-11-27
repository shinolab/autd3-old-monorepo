"""
File: backend_nalgebra.py
Project: holo
Created Date: 27/10/2023
Author: Shun Suzuki
-----
Last Modified: 27/10/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""

from ctypes import Array, c_double

from pyautd3.native_methods.autd3capi_def import GainPtr
from pyautd3.native_methods.autd3capi_gain_holo import NativeMethods as GainHolo

from .backend import Backend
from .constraint import EmissionConstraint


class NalgebraBackend(Backend):
    """Backend using nalgebra."""

    def __init__(self: "NalgebraBackend") -> None:
        super().__init__(GainHolo().nalgebra_backend())

    def __del__(self: "NalgebraBackend") -> None:
        if self._ptr._0 is not None:
            GainHolo().delete_nalgebra_backend(self._ptr)
            self._ptr._0 = None

    def _sdp(self: "NalgebraBackend", foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        return GainHolo().gain_holo_sdp(self._backend_ptr(), foci, amps, size)

    def _sdp_with_alpha(self: "NalgebraBackend", ptr: GainPtr, v: float) -> GainPtr:
        return GainHolo().gain_holo_sdp_with_alpha(ptr, v)

    def _sdp_with_repeat(self: "NalgebraBackend", ptr: GainPtr, v: int) -> GainPtr:
        return GainHolo().gain_holo_sdp_with_repeat(ptr, v)

    def _sdp_with_lambda(self: "NalgebraBackend", ptr: GainPtr, v: float) -> GainPtr:
        return GainHolo().gain_holo_sdp_with_lambda(ptr, v)

    def _sdp_with_constraint(self: "NalgebraBackend", ptr: GainPtr, v: EmissionConstraint) -> GainPtr:
        return GainHolo().gain_holo_sdp_with_constraint(ptr, v._constraint_ptr())

    def _gs(self: "NalgebraBackend", foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        return GainHolo().gain_holo_gs(self._backend_ptr(), foci, amps, size)

    def _gs_with_repeat(self: "NalgebraBackend", ptr: GainPtr, v: int) -> GainPtr:
        return GainHolo().gain_holo_gs_with_repeat(ptr, v)

    def _gs_with_constraint(self: "NalgebraBackend", ptr: GainPtr, v: EmissionConstraint) -> GainPtr:
        return GainHolo().gain_holo_gs_with_constraint(ptr, v._constraint_ptr())

    def _gspat(self: "NalgebraBackend", foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        return GainHolo().gain_holo_gspat(self._backend_ptr(), foci, amps, size)

    def _gspat_with_repeat(self: "NalgebraBackend", ptr: GainPtr, v: int) -> GainPtr:
        return GainHolo().gain_holo_gspat_with_repeat(ptr, v)

    def _gspat_with_constraint(self: "NalgebraBackend", ptr: GainPtr, v: EmissionConstraint) -> GainPtr:
        return GainHolo().gain_holo_gspat_with_constraint(ptr, v._constraint_ptr())

    def _naive(self: "NalgebraBackend", foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        return GainHolo().gain_holo_naive(self._backend_ptr(), foci, amps, size)

    def _naive_with_constraint(self: "NalgebraBackend", ptr: GainPtr, v: EmissionConstraint) -> GainPtr:
        return GainHolo().gain_holo_naive_with_constraint(ptr, v._constraint_ptr())

    def _lm(self: "NalgebraBackend", foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        return GainHolo().gain_holo_lm(self._backend_ptr(), foci, amps, size)

    def _lm_with_eps1(self: "NalgebraBackend", ptr: GainPtr, v: float) -> GainPtr:
        return GainHolo().gain_holo_lm_with_eps_1(ptr, v)

    def _lm_with_eps2(self: "NalgebraBackend", ptr: GainPtr, v: float) -> GainPtr:
        return GainHolo().gain_holo_lm_with_eps_2(ptr, v)

    def _lm_with_tau(self: "NalgebraBackend", ptr: GainPtr, v: float) -> GainPtr:
        return GainHolo().gain_holo_lm_with_tau(ptr, v)

    def _lm_with_kmax(self: "NalgebraBackend", ptr: GainPtr, v: int) -> GainPtr:
        return GainHolo().gain_holo_lm_with_k_max(ptr, v)

    def _lm_with_initial(self: "NalgebraBackend", ptr: GainPtr, v: Array[c_double], size: int) -> GainPtr:
        return GainHolo().gain_holo_lm_with_initial(ptr, v, size)

    def _lm_with_constraint(self: "NalgebraBackend", ptr: GainPtr, v: EmissionConstraint) -> GainPtr:
        return GainHolo().gain_holo_lm_with_constraint(ptr, v._constraint_ptr())
