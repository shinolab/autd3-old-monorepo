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
        self.dll = ctypes.CDLL(os.path.join(self.bin, f'{self.prefix}autd3capi-simulator{self.ext}'))

        self.ERR = - 1

        self.dll.AUTDSimulator.argtypes = [] 
        self.dll.AUTDSimulator.restype = ctypes.c_void_p

        self.dll.AUTDSimulatorPort.argtypes = [ctypes.c_void_p, ctypes.c_uint16] 
        self.dll.AUTDSimulatorPort.restype = ctypes.c_void_p

        self.dll.AUTDSimulatorWindowSize.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.c_uint32] 
        self.dll.AUTDSimulatorWindowSize.restype = ctypes.c_void_p

        self.dll.AUTDSimulatorVsync.argtypes = [ctypes.c_void_p, ctypes.c_bool] 
        self.dll.AUTDSimulatorVsync.restype = ctypes.c_void_p

        self.dll.AUTDSimulatorGpuIdx.argtypes = [ctypes.c_void_p, ctypes.c_int32] 
        self.dll.AUTDSimulatorGpuIdx.restype = ctypes.c_void_p

        self.dll.AUTDSimulatorSettingsPath.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_char_p] 
        self.dll.AUTDSimulatorSettingsPath.restype = ctypes.c_void_p

        self.dll.AUTDSimulatorRun.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDSimulatorRun.restype = ctypes.c_int32

        self.dll.AUTDSimulatorSaveSettings.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_char_p] 
        self.dll.AUTDSimulatorSaveSettings.restype = ctypes.c_bool
