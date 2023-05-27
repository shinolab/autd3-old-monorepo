# This file was automatically generated from header file
import threading
import ctypes
import os
from enum import IntEnum

class Singleton(type):
    _instances = {}
    _lock = threading.Lock()

    def __call__(cls, *args, **kwargs):
        if cls not in cls._instances:
            with cls._lock:
                if cls not in cls._instances:
                    cls._instances[cls] = super(Singleton, cls).__call__(*args, **kwargs)
        return cls._instances[cls]


class NativeMethods(metaclass=Singleton):
    def init_path(self, bin_location: str, bin_prefix: str, bin_ext: str):
        self.bin = bin_location
        self.prefix = bin_prefix
        self.ext = bin_ext

    def init_dll(self):
        if hasattr(self, 'dll'):
            return
        self.dll = ctypes.CDLL(os.path.join(self.bin, f'{self.prefix}autd3capi-gain-holo{self.ext}'))


        self.dll.AUTDDefaultBackend.argtypes = [] 
        self.dll.AUTDDefaultBackend.restype = ctypes.c_void_p

        self.dll.AUTDGainHoloSDP.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDGainHoloSDP.restype = ctypes.c_void_p

        self.dll.AUTDGainHoloSDPAlpha.argtypes = [ctypes.c_void_p, ctypes.c_double] 
        self.dll.AUTDGainHoloSDPAlpha.restype = None

        self.dll.AUTDGainHoloSDPLambda.argtypes = [ctypes.c_void_p, ctypes.c_double] 
        self.dll.AUTDGainHoloSDPLambda.restype = None

        self.dll.AUTDGainHoloSDPRepeat.argtypes = [ctypes.c_void_p, ctypes.c_uint32] 
        self.dll.AUTDGainHoloSDPRepeat.restype = None

        self.dll.AUTDGainHoloEVP.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDGainHoloEVP.restype = ctypes.c_void_p

        self.dll.AUTDGainHoloEVPGamma.argtypes = [ctypes.c_void_p, ctypes.c_double] 
        self.dll.AUTDGainHoloEVPGamma.restype = None

        self.dll.AUTDGainHoloGS.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDGainHoloGS.restype = ctypes.c_void_p

        self.dll.AUTDGainHoloGSRepeat.argtypes = [ctypes.c_void_p, ctypes.c_uint32] 
        self.dll.AUTDGainHoloGSRepeat.restype = None

        self.dll.AUTDGainHoloGSPAT.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDGainHoloGSPAT.restype = ctypes.c_void_p

        self.dll.AUTDGainHoloGSPATRepeat.argtypes = [ctypes.c_void_p, ctypes.c_uint32] 
        self.dll.AUTDGainHoloGSPATRepeat.restype = None

        self.dll.AUTDGainHoloNaive.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDGainHoloNaive.restype = ctypes.c_void_p

        self.dll.AUTDGainHoloGreedy.argtypes = [] 
        self.dll.AUTDGainHoloGreedy.restype = ctypes.c_void_p

        self.dll.AUTDGainHoloGreedyPhaseDiv.argtypes = [ctypes.c_void_p, ctypes.c_uint32] 
        self.dll.AUTDGainHoloGreedyPhaseDiv.restype = None

        self.dll.AUTDGainHoloLM.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDGainHoloLM.restype = ctypes.c_void_p

        self.dll.AUTDGainHoloLMEps1.argtypes = [ctypes.c_void_p, ctypes.c_double] 
        self.dll.AUTDGainHoloLMEps1.restype = None

        self.dll.AUTDGainHoloLMEps2.argtypes = [ctypes.c_void_p, ctypes.c_double] 
        self.dll.AUTDGainHoloLMEps2.restype = None

        self.dll.AUTDGainHoloLMTau.argtypes = [ctypes.c_void_p, ctypes.c_double] 
        self.dll.AUTDGainHoloLMTau.restype = None

        self.dll.AUTDGainHoloLMKMax.argtypes = [ctypes.c_void_p, ctypes.c_uint32] 
        self.dll.AUTDGainHoloLMKMax.restype = None

        self.dll.AUTDGainHoloLMInitial.argtypes = [ctypes.c_void_p, ctypes.POINTER(ctypes.c_double), ctypes.c_uint64] 
        self.dll.AUTDGainHoloLMInitial.restype = None

        self.dll.AUTDGainHoloAdd.argtypes = [ctypes.c_void_p, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double] 
        self.dll.AUTDGainHoloAdd.restype = None

        self.dll.AUTDGainHoloSetDotCareConstraint.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDGainHoloSetDotCareConstraint.restype = None

        self.dll.AUTDGainHoloSetNormalizeConstraint.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDGainHoloSetNormalizeConstraint.restype = None

        self.dll.AUTDGainHoloSetUniformConstraint.argtypes = [ctypes.c_void_p, ctypes.c_double] 
        self.dll.AUTDGainHoloSetUniformConstraint.restype = None

        self.dll.AUTDGainHoloSetClampConstraint.argtypes = [ctypes.c_void_p, ctypes.c_double, ctypes.c_double] 
        self.dll.AUTDGainHoloSetClampConstraint.restype = None

        self.dll.AUTDDeleteGainHolo.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDDeleteGainHolo.restype = None
