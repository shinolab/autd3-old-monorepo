'''
File: backend_cuda.py
Project: holo
Created Date: 08/06/2023
Author: Shun Suzuki
-----
Last Modified: 21/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

'''

import ctypes
from ctypes import c_double, Array

from pyautd3.native_methods.autd3capi_backend_cuda import (
    NativeMethods as AUTD3BackendCUDA,
)
from pyautd3.native_methods.autd3capi_def import GainPtr
from .backend import Backend
from pyautd3.gain.holo.constraint import AmplitudeConstraint
from pyautd3.autd_error import AUTDError


class CUDABackend(Backend):
    def __init__(self):
        err = ctypes.create_string_buffer(256)
        ptr = AUTD3BackendCUDA().cuda_backend(err)
        if ptr._0 is None:
            raise AUTDError(err)
        super().__init__(ptr)

    def __del__(self):
        if hasattr(self, "_ptr") and self._ptr._0 is not None:
            AUTD3BackendCUDA().cuda_backend_delete(self._ptr)
            self._ptr._0 = None

    def sdp(self, foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        return AUTD3BackendCUDA().gain_holo_cudasdp(self.ptr(), foci, amps, size)

    def sdp_with_alpha(self, ptr: GainPtr, v: float) -> GainPtr:
        return AUTD3BackendCUDA().gain_holo_cudasdp_with_alpha(ptr, v)

    def sdp_with_repeat(self, ptr: GainPtr, v: int) -> GainPtr:
        return AUTD3BackendCUDA().gain_holo_cudasdp_with_repeat(ptr, v)

    def sdp_with_lambda(self, ptr: GainPtr, v: float) -> GainPtr:
        return AUTD3BackendCUDA().gain_holo_cudasdp_with_lambda(ptr, v)

    def sdp_with_constraint(self, ptr: GainPtr, v: AmplitudeConstraint) -> GainPtr:
        return AUTD3BackendCUDA().gain_holo_cudasdp_with_constraint(ptr, v.ptr())

    def evp(self, foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        return AUTD3BackendCUDA().gain_holo_cudaevp(self.ptr(), foci, amps, size)

    def evp_with_gamma(self, ptr: GainPtr, v: float) -> GainPtr:
        return AUTD3BackendCUDA().gain_holo_cudaevp_with_gamma(ptr, v)

    def evp_with_constraint(self, ptr: GainPtr, v: AmplitudeConstraint) -> GainPtr:
        return AUTD3BackendCUDA().gain_holo_cudaevp_with_constraint(ptr, v.ptr())

    def gs(self, foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        return AUTD3BackendCUDA().gain_holo_cudags(self.ptr(), foci, amps, size)

    def gs_with_repeat(self, ptr: GainPtr, v: int) -> GainPtr:
        return AUTD3BackendCUDA().gain_holo_cudags_with_repeat(ptr, v)

    def gs_with_constraint(self, ptr: GainPtr, v: AmplitudeConstraint) -> GainPtr:
        return AUTD3BackendCUDA().gain_holo_cudags_with_constraint(ptr, v.ptr())

    def gspat(self, foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        return AUTD3BackendCUDA().gain_holo_cudagspat(self.ptr(), foci, amps, size)

    def gspat_with_repeat(self, ptr: GainPtr, v: int) -> GainPtr:
        return AUTD3BackendCUDA().gain_holo_cudagspat_with_repeat(ptr, v)

    def gspat_with_constraint(self, ptr: GainPtr, v: AmplitudeConstraint) -> GainPtr:
        return AUTD3BackendCUDA().gain_holo_cudagspat_with_constraint(ptr, v.ptr())

    def naive(self, foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        return AUTD3BackendCUDA().gain_holo_cuda_naive(self.ptr(), foci, amps, size)

    def naive_with_constraint(self, ptr: GainPtr, v: AmplitudeConstraint) -> GainPtr:
        return AUTD3BackendCUDA().gain_holo_cuda_naive_with_constraint(ptr, v.ptr())

    def lm(self, foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        return AUTD3BackendCUDA().gain_holo_cudalm(self.ptr(), foci, amps, size)

    def lm_with_eps1(self, ptr: GainPtr, v: float) -> GainPtr:
        return AUTD3BackendCUDA().gain_holo_cudalm_with_eps_1(ptr, v)

    def lm_with_eps2(self, ptr: GainPtr, v: float) -> GainPtr:
        return AUTD3BackendCUDA().gain_holo_cudalm_with_eps_2(ptr, v)

    def lm_with_tau(self, ptr: GainPtr, v: float) -> GainPtr:
        return AUTD3BackendCUDA().gain_holo_cudalm_with_tau(ptr, v)

    def lm_with_kmax(self, ptr: GainPtr, v: int) -> GainPtr:
        return AUTD3BackendCUDA().gain_holo_cudalm_with_k_max(ptr, v)

    def lm_with_initial(self, ptr: GainPtr, v: Array[c_double], size: int) -> GainPtr:
        return AUTD3BackendCUDA().gain_holo_cudalm_with_initial(ptr, v, size)

    def lm_with_constraint(self, ptr: GainPtr, v: AmplitudeConstraint) -> GainPtr:
        return AUTD3BackendCUDA().gain_holo_cudalm_with_constraint(ptr, v.ptr())
