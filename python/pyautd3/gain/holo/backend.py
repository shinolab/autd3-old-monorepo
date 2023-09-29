'''
File: backend.py
Project: holo
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 29/09/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.

'''

from abc import ABCMeta, abstractmethod
from ctypes import c_double, Array

from pyautd3.native_methods.autd3capi_gain_holo import NativeMethods as GainHolo
from pyautd3.native_methods.autd3capi_def import BackendPtr
from pyautd3.gain.holo.constraint import AmplitudeConstraint
from pyautd3.native_methods.autd3capi_def import GainPtr


class Backend(metaclass=ABCMeta):
    """Calculation backend

    """

    _ptr: BackendPtr

    def __init__(self, ptr: BackendPtr):
        self._ptr = ptr

    def ptr(self) -> BackendPtr:
        return self._ptr

    @abstractmethod
    def sdp(self, foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        raise NotImplementedError()

    @abstractmethod
    def sdp_with_alpha(self, ptr: GainPtr, v: float) -> GainPtr:
        raise NotImplementedError()

    @abstractmethod
    def sdp_with_repeat(self, ptr: GainPtr, v: int) -> GainPtr:
        raise NotImplementedError()

    @abstractmethod
    def sdp_with_lambda(self, ptr: GainPtr, v: float) -> GainPtr:
        raise NotImplementedError()

    @abstractmethod
    def sdp_with_constraint(self, ptr: GainPtr, v: AmplitudeConstraint) -> GainPtr:
        raise NotImplementedError()

    @abstractmethod
    def evp(self, foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        raise NotImplementedError()

    @abstractmethod
    def evp_with_gamma(self, ptr: GainPtr, v: float) -> GainPtr:
        raise NotImplementedError()

    @abstractmethod
    def evp_with_constraint(self, ptr: GainPtr, v: AmplitudeConstraint) -> GainPtr:
        raise NotImplementedError()

    @abstractmethod
    def gs(self, foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        raise NotImplementedError()

    @abstractmethod
    def gs_with_repeat(self, ptr: GainPtr, v: int) -> GainPtr:
        raise NotImplementedError()

    @abstractmethod
    def gs_with_constraint(self, ptr: GainPtr, v: AmplitudeConstraint) -> GainPtr:
        raise NotImplementedError()

    @abstractmethod
    def gspat(self, foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        raise NotImplementedError()

    @abstractmethod
    def gspat_with_repeat(self, ptr: GainPtr, v: int) -> GainPtr:
        raise NotImplementedError()

    @abstractmethod
    def gspat_with_constraint(self, ptr: GainPtr, v: AmplitudeConstraint) -> GainPtr:
        raise NotImplementedError()

    @abstractmethod
    def naive(self, foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        raise NotImplementedError()

    @abstractmethod
    def naive_with_constraint(self, ptr: GainPtr, v: AmplitudeConstraint) -> GainPtr:
        raise NotImplementedError()

    @abstractmethod
    def lm(self, foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        raise NotImplementedError()

    @abstractmethod
    def lm_with_eps1(self, ptr: GainPtr, v: float) -> GainPtr:
        raise NotImplementedError()

    @abstractmethod
    def lm_with_eps2(self, ptr: GainPtr, v: float) -> GainPtr:
        raise NotImplementedError()

    @abstractmethod
    def lm_with_tau(self, ptr: GainPtr, v: float) -> GainPtr:
        raise NotImplementedError()

    @abstractmethod
    def lm_with_kmax(self, ptr: GainPtr, v: int) -> GainPtr:
        raise NotImplementedError()

    @abstractmethod
    def lm_with_initial(self, ptr: GainPtr, v: Array[c_double], size: int) -> GainPtr:
        raise NotImplementedError()

    @abstractmethod
    def lm_with_constraint(self, ptr: GainPtr, v: AmplitudeConstraint) -> GainPtr:
        raise NotImplementedError()


class NalgebraBackend(Backend):
    """Backend using nalgebra

    """

    def __init__(self):
        super().__init__(GainHolo().nalgebra_backend())

    def __del__(self):
        if self._ptr._0 is not None:
            GainHolo().delete_nalgebra_backend(self._ptr)
            self._ptr._0 = None

    def sdp(self, foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        return GainHolo().gain_holo_sdp(self.ptr(), foci, amps, size)

    def sdp_with_alpha(self, ptr: GainPtr, v: float) -> GainPtr:
        return GainHolo().gain_holo_sdp_with_alpha(ptr, v)

    def sdp_with_repeat(self, ptr: GainPtr, v: int) -> GainPtr:
        return GainHolo().gain_holo_sdp_with_repeat(ptr, v)

    def sdp_with_lambda(self, ptr: GainPtr, v: float) -> GainPtr:
        return GainHolo().gain_holo_sdp_with_lambda(ptr, v)

    def sdp_with_constraint(self, ptr: GainPtr, v: AmplitudeConstraint) -> GainPtr:
        return GainHolo().gain_holo_sdp_with_constraint(ptr, v.ptr())

    def evp(self, foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        return GainHolo().gain_holo_evp(self.ptr(), foci, amps, size)

    def evp_with_gamma(self, ptr: GainPtr, v: float) -> GainPtr:
        return GainHolo().gain_holo_evp_with_gamma(ptr, v)

    def evp_with_constraint(self, ptr: GainPtr, v: AmplitudeConstraint) -> GainPtr:
        return GainHolo().gain_holo_evp_with_constraint(ptr, v.ptr())

    def gs(self, foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        return GainHolo().gain_holo_gs(self.ptr(), foci, amps, size)

    def gs_with_repeat(self, ptr: GainPtr, v: int) -> GainPtr:
        return GainHolo().gain_holo_gs_with_repeat(ptr, v)

    def gs_with_constraint(self, ptr: GainPtr, v: AmplitudeConstraint) -> GainPtr:
        return GainHolo().gain_holo_gs_with_constraint(ptr, v.ptr())

    def gspat(self, foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        return GainHolo().gain_holo_gspat(self.ptr(), foci, amps, size)

    def gspat_with_repeat(self, ptr: GainPtr, v: int) -> GainPtr:
        return GainHolo().gain_holo_gspat_with_repeat(ptr, v)

    def gspat_with_constraint(self, ptr: GainPtr, v: AmplitudeConstraint) -> GainPtr:
        return GainHolo().gain_holo_gspat_with_constraint(ptr, v.ptr())

    def naive(self, foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        return GainHolo().gain_holo_naive(self.ptr(), foci, amps, size)

    def naive_with_constraint(self, ptr: GainPtr, v: AmplitudeConstraint) -> GainPtr:
        return GainHolo().gain_holo_naive_with_constraint(ptr, v.ptr())

    def lm(self, foci: Array[c_double], amps: Array[c_double], size: int) -> GainPtr:
        return GainHolo().gain_holo_lm(self.ptr(), foci, amps, size)

    def lm_with_eps1(self, ptr: GainPtr, v: float) -> GainPtr:
        return GainHolo().gain_holo_lm_with_eps_1(ptr, v)

    def lm_with_eps2(self, ptr: GainPtr, v: float) -> GainPtr:
        return GainHolo().gain_holo_lm_with_eps_2(ptr, v)

    def lm_with_tau(self, ptr: GainPtr, v: float) -> GainPtr:
        return GainHolo().gain_holo_lm_with_tau(ptr, v)

    def lm_with_kmax(self, ptr: GainPtr, v: int) -> GainPtr:
        return GainHolo().gain_holo_lm_with_k_max(ptr, v)

    def lm_with_initial(self, ptr: GainPtr, v: Array[c_double], size: int) -> GainPtr:
        return GainHolo().gain_holo_lm_with_initial(ptr, v, size)

    def lm_with_constraint(self, ptr: GainPtr, v: AmplitudeConstraint) -> GainPtr:
        return GainHolo().gain_holo_lm_with_constraint(ptr, v.ptr())
