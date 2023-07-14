# This file is autogenerated
import threading
import ctypes
import os
from typing import Any
from .autd3capi_def import BackendPtr, GainPtr


class ConstraintPtr(ctypes.Structure):
    _fields_ = [("_0", ctypes.c_void_p)]


class Singleton(type):
    _instances = {}  # type: ignore
    _lock = threading.Lock()

    def __call__(cls, *args, **kwargs):
        if cls not in cls._instances:
            with cls._lock:
                if cls not in cls._instances:
                    cls._instances[cls] = super(Singleton, cls).__call__(*args, **kwargs)
        return cls._instances[cls]


class NativeMethods(metaclass=Singleton):

    def init_dll(self, bin_location: str, bin_prefix: str, bin_ext: str):
        try:
            self.dll = ctypes.CDLL(os.path.join(bin_location, f'{bin_prefix}autd3capi_gain_holo{bin_ext}'))
        except Exception:
            return

        self.dll.AUTDDefaultBackend.argtypes = [] 
        self.dll.AUTDDefaultBackend.restype = BackendPtr

        self.dll.AUTDDeleteBackend.argtypes = [BackendPtr]  # type: ignore 
        self.dll.AUTDDeleteBackend.restype = None

        self.dll.AUTDGainHoloDotCareConstraint.argtypes = [] 
        self.dll.AUTDGainHoloDotCareConstraint.restype = ConstraintPtr

        self.dll.AUTDGainHoloNormalizeConstraint.argtypes = [] 
        self.dll.AUTDGainHoloNormalizeConstraint.restype = ConstraintPtr

        self.dll.AUTDGainHoloUniformConstraint.argtypes = [ctypes.c_double] 
        self.dll.AUTDGainHoloUniformConstraint.restype = ConstraintPtr

        self.dll.AUTDGainHoloClampConstraint.argtypes = [ctypes.c_double, ctypes.c_double] 
        self.dll.AUTDGainHoloClampConstraint.restype = ConstraintPtr

        self.dll.AUTDGainHoloSDP.argtypes = [BackendPtr, ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double), ctypes.c_uint64]  # type: ignore 
        self.dll.AUTDGainHoloSDP.restype = GainPtr

        self.dll.AUTDGainHoloSDPWithConstraint.argtypes = [GainPtr, ConstraintPtr]  # type: ignore 
        self.dll.AUTDGainHoloSDPWithConstraint.restype = GainPtr

        self.dll.AUTDGainHoloSDPWithAlpha.argtypes = [GainPtr, ctypes.c_double]  # type: ignore 
        self.dll.AUTDGainHoloSDPWithAlpha.restype = GainPtr

        self.dll.AUTDGainHoloSDPWithLambda.argtypes = [GainPtr, ctypes.c_double]  # type: ignore 
        self.dll.AUTDGainHoloSDPWithLambda.restype = GainPtr

        self.dll.AUTDGainHoloSDPWithRepeat.argtypes = [GainPtr, ctypes.c_uint32]  # type: ignore 
        self.dll.AUTDGainHoloSDPWithRepeat.restype = GainPtr

        self.dll.AUTDGainHoloEVP.argtypes = [BackendPtr, ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double), ctypes.c_uint64]  # type: ignore 
        self.dll.AUTDGainHoloEVP.restype = GainPtr

        self.dll.AUTDGainHoloEVPWithConstraint.argtypes = [GainPtr, ConstraintPtr]  # type: ignore 
        self.dll.AUTDGainHoloEVPWithConstraint.restype = GainPtr

        self.dll.AUTDGainHoloEVPWithGamma.argtypes = [GainPtr, ctypes.c_double]  # type: ignore 
        self.dll.AUTDGainHoloEVPWithGamma.restype = GainPtr

        self.dll.AUTDGainHoloGS.argtypes = [BackendPtr, ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double), ctypes.c_uint64]  # type: ignore 
        self.dll.AUTDGainHoloGS.restype = GainPtr

        self.dll.AUTDGainHoloGSWithConstraint.argtypes = [GainPtr, ConstraintPtr]  # type: ignore 
        self.dll.AUTDGainHoloGSWithConstraint.restype = GainPtr

        self.dll.AUTDGainHoloGSWithRepeat.argtypes = [GainPtr, ctypes.c_uint32]  # type: ignore 
        self.dll.AUTDGainHoloGSWithRepeat.restype = GainPtr

        self.dll.AUTDGainHoloGSPAT.argtypes = [BackendPtr, ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double), ctypes.c_uint64]  # type: ignore 
        self.dll.AUTDGainHoloGSPAT.restype = GainPtr

        self.dll.AUTDGainHoloGSPATWithConstraint.argtypes = [GainPtr, ConstraintPtr]  # type: ignore 
        self.dll.AUTDGainHoloGSPATWithConstraint.restype = GainPtr

        self.dll.AUTDGainHoloGSPATWithRepeat.argtypes = [GainPtr, ctypes.c_uint32]  # type: ignore 
        self.dll.AUTDGainHoloGSPATWithRepeat.restype = GainPtr

        self.dll.AUTDGainHoloNaive.argtypes = [BackendPtr, ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double), ctypes.c_uint64]  # type: ignore 
        self.dll.AUTDGainHoloNaive.restype = GainPtr

        self.dll.AUTDGainHoloNaiveWithConstraint.argtypes = [GainPtr, ConstraintPtr]  # type: ignore 
        self.dll.AUTDGainHoloNaiveWithConstraint.restype = GainPtr

        self.dll.AUTDGainHoloGreedy.argtypes = [ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double), ctypes.c_uint64] 
        self.dll.AUTDGainHoloGreedy.restype = GainPtr

        self.dll.AUTDGainHoloGreedyWithConstraint.argtypes = [GainPtr, ConstraintPtr]  # type: ignore 
        self.dll.AUTDGainHoloGreedyWithConstraint.restype = GainPtr

        self.dll.AUTDGainHoloGreedyWithPhaseDiv.argtypes = [GainPtr, ctypes.c_uint32]  # type: ignore 
        self.dll.AUTDGainHoloGreedyWithPhaseDiv.restype = GainPtr

        self.dll.AUTDGainHoloLM.argtypes = [BackendPtr, ctypes.POINTER(ctypes.c_double), ctypes.POINTER(ctypes.c_double), ctypes.c_uint64]  # type: ignore 
        self.dll.AUTDGainHoloLM.restype = GainPtr

        self.dll.AUTDGainHoloLMWithConstraint.argtypes = [GainPtr, ConstraintPtr]  # type: ignore 
        self.dll.AUTDGainHoloLMWithConstraint.restype = GainPtr

        self.dll.AUTDGainHoloLMWithEps1.argtypes = [GainPtr, ctypes.c_double]  # type: ignore 
        self.dll.AUTDGainHoloLMWithEps1.restype = GainPtr

        self.dll.AUTDGainHoloLMWithEps2.argtypes = [GainPtr, ctypes.c_double]  # type: ignore 
        self.dll.AUTDGainHoloLMWithEps2.restype = GainPtr

        self.dll.AUTDGainHoloLMWithTau.argtypes = [GainPtr, ctypes.c_double]  # type: ignore 
        self.dll.AUTDGainHoloLMWithTau.restype = GainPtr

        self.dll.AUTDGainHoloLMWithKMax.argtypes = [GainPtr, ctypes.c_uint32]  # type: ignore 
        self.dll.AUTDGainHoloLMWithKMax.restype = GainPtr

        self.dll.AUTDGainHoloLMWithInitial.argtypes = [GainPtr, ctypes.POINTER(ctypes.c_double), ctypes.c_uint64]  # type: ignore 
        self.dll.AUTDGainHoloLMWithInitial.restype = GainPtr

    def default_backend(self) -> BackendPtr:
        return self.dll.AUTDDefaultBackend()

    def delete_backend(self, backend: BackendPtr) -> None:
        return self.dll.AUTDDeleteBackend(backend)

    def gain_holo_dot_care_constraint(self) -> ConstraintPtr:
        return self.dll.AUTDGainHoloDotCareConstraint()

    def gain_holo_normalize_constraint(self) -> ConstraintPtr:
        return self.dll.AUTDGainHoloNormalizeConstraint()

    def gain_holo_uniform_constraint(self, value: float) -> ConstraintPtr:
        return self.dll.AUTDGainHoloUniformConstraint(value)

    def gain_holo_clamp_constraint(self, min_v: float, max_v: float) -> ConstraintPtr:
        return self.dll.AUTDGainHoloClampConstraint(min_v, max_v)

    def gain_holo_sdp(self, backend: BackendPtr, points: Any, amps: Any, size: int) -> GainPtr:
        return self.dll.AUTDGainHoloSDP(backend, points, amps, size)

    def gain_holo_sdp_with_constraint(self, holo: GainPtr, constraint: ConstraintPtr) -> GainPtr:
        return self.dll.AUTDGainHoloSDPWithConstraint(holo, constraint)

    def gain_holo_sdp_with_alpha(self, holo: GainPtr, alpha: float) -> GainPtr:
        return self.dll.AUTDGainHoloSDPWithAlpha(holo, alpha)

    def gain_holo_sdp_with_lambda(self, holo: GainPtr, lambda_: float) -> GainPtr:
        return self.dll.AUTDGainHoloSDPWithLambda(holo, lambda_)

    def gain_holo_sdp_with_repeat(self, holo: GainPtr, repeat: int) -> GainPtr:
        return self.dll.AUTDGainHoloSDPWithRepeat(holo, repeat)

    def gain_holo_evp(self, backend: BackendPtr, points: Any, amps: Any, size: int) -> GainPtr:
        return self.dll.AUTDGainHoloEVP(backend, points, amps, size)

    def gain_holo_evp_with_constraint(self, holo: GainPtr, constraint: ConstraintPtr) -> GainPtr:
        return self.dll.AUTDGainHoloEVPWithConstraint(holo, constraint)

    def gain_holo_evp_with_gamma(self, holo: GainPtr, gamma: float) -> GainPtr:
        return self.dll.AUTDGainHoloEVPWithGamma(holo, gamma)

    def gain_holo_gs(self, backend: BackendPtr, points: Any, amps: Any, size: int) -> GainPtr:
        return self.dll.AUTDGainHoloGS(backend, points, amps, size)

    def gain_holo_gs_with_constraint(self, holo: GainPtr, constraint: ConstraintPtr) -> GainPtr:
        return self.dll.AUTDGainHoloGSWithConstraint(holo, constraint)

    def gain_holo_gs_with_repeat(self, holo: GainPtr, repeat: int) -> GainPtr:
        return self.dll.AUTDGainHoloGSWithRepeat(holo, repeat)

    def gain_holo_gspat(self, backend: BackendPtr, points: Any, amps: Any, size: int) -> GainPtr:
        return self.dll.AUTDGainHoloGSPAT(backend, points, amps, size)

    def gain_holo_gspat_with_constraint(self, holo: GainPtr, constraint: ConstraintPtr) -> GainPtr:
        return self.dll.AUTDGainHoloGSPATWithConstraint(holo, constraint)

    def gain_holo_gspat_with_repeat(self, holo: GainPtr, repeat: int) -> GainPtr:
        return self.dll.AUTDGainHoloGSPATWithRepeat(holo, repeat)

    def gain_holo_naive(self, backend: BackendPtr, points: Any, amps: Any, size: int) -> GainPtr:
        return self.dll.AUTDGainHoloNaive(backend, points, amps, size)

    def gain_holo_naive_with_constraint(self, holo: GainPtr, constraint: ConstraintPtr) -> GainPtr:
        return self.dll.AUTDGainHoloNaiveWithConstraint(holo, constraint)

    def gain_holo_greedy(self, points: Any, amps: Any, size: int) -> GainPtr:
        return self.dll.AUTDGainHoloGreedy(points, amps, size)

    def gain_holo_greedy_with_constraint(self, holo: GainPtr, constraint: ConstraintPtr) -> GainPtr:
        return self.dll.AUTDGainHoloGreedyWithConstraint(holo, constraint)

    def gain_holo_greedy_with_phase_div(self, holo: GainPtr, div: int) -> GainPtr:
        return self.dll.AUTDGainHoloGreedyWithPhaseDiv(holo, div)

    def gain_holo_lm(self, backend: BackendPtr, points: Any, amps: Any, size: int) -> GainPtr:
        return self.dll.AUTDGainHoloLM(backend, points, amps, size)

    def gain_holo_lm_with_constraint(self, holo: GainPtr, constraint: ConstraintPtr) -> GainPtr:
        return self.dll.AUTDGainHoloLMWithConstraint(holo, constraint)

    def gain_holo_lm_with_eps_1(self, holo: GainPtr, eps: float) -> GainPtr:
        return self.dll.AUTDGainHoloLMWithEps1(holo, eps)

    def gain_holo_lm_with_eps_2(self, holo: GainPtr, eps: float) -> GainPtr:
        return self.dll.AUTDGainHoloLMWithEps2(holo, eps)

    def gain_holo_lm_with_tau(self, holo: GainPtr, tau: float) -> GainPtr:
        return self.dll.AUTDGainHoloLMWithTau(holo, tau)

    def gain_holo_lm_with_k_max(self, holo: GainPtr, k_max: int) -> GainPtr:
        return self.dll.AUTDGainHoloLMWithKMax(holo, k_max)

    def gain_holo_lm_with_initial(self, holo: GainPtr, initial_ptr: Any, len: int) -> GainPtr:
        return self.dll.AUTDGainHoloLMWithInitial(holo, initial_ptr, len)
