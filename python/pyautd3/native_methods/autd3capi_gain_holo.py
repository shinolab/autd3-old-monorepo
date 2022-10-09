# This file was automatically generated from header file
import threading
import ctypes
import os


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

        self.dll.AUTDEigenBackend.argtypes = [ctypes.POINTER(ctypes.c_void_p)] 
        self.dll.AUTDEigenBackend.restype = None

        self.dll.AUTDDeleteBackend.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDDeleteBackend.restype = None

        self.dll.AUTDGainHoloSDP.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_void_p, ctypes.c_double, ctypes.c_double, ctypes.c_uint64] 
        self.dll.AUTDGainHoloSDP.restype = None

        self.dll.AUTDGainHoloEVD.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_void_p, ctypes.c_double] 
        self.dll.AUTDGainHoloEVD.restype = None

        self.dll.AUTDGainHoloNaive.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_void_p] 
        self.dll.AUTDGainHoloNaive.restype = None

        self.dll.AUTDGainHoloGS.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_void_p, ctypes.c_uint64] 
        self.dll.AUTDGainHoloGS.restype = None

        self.dll.AUTDGainHoloGSPAT.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_void_p, ctypes.c_uint64] 
        self.dll.AUTDGainHoloGSPAT.restype = None

        self.dll.AUTDGainHoloLM.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_void_p, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_uint64, ctypes.POINTER(ctypes.c_double), ctypes.c_int32] 
        self.dll.AUTDGainHoloLM.restype = None

        self.dll.AUTDGainHoloGreedy.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_void_p, ctypes.c_int32] 
        self.dll.AUTDGainHoloGreedy.restype = None

        self.dll.AUTDGainHoloLSSGreedy.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_void_p, ctypes.c_int32] 
        self.dll.AUTDGainHoloLSSGreedy.restype = None

        self.dll.AUTDGainHoloAPO.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_void_p, ctypes.c_double, ctypes.c_double, ctypes.c_int32, ctypes.c_int32] 
        self.dll.AUTDGainHoloAPO.restype = None

        self.dll.AUTDGainHoloAdd.argtypes = [ctypes.c_void_p, ctypes.c_double, ctypes.c_double, ctypes.c_double, ctypes.c_double] 
        self.dll.AUTDGainHoloAdd.restype = None

        self.dll.AUTDSetConstraint.argtypes = [ctypes.c_void_p, ctypes.c_int32, ctypes.c_void_p] 
        self.dll.AUTDSetConstraint.restype = None
