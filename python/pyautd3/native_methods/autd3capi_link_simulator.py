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
        self.dll = ctypes.CDLL(os.path.join(self.bin, f'{self.prefix}autd3capi-link-simulator{self.ext}'))


        self.dll.AUTDLinkSimulator.argtypes = [ctypes.c_uint16] 
        self.dll.AUTDLinkSimulator.restype = ctypes.c_void_p

        self.dll.AUTDLinkSimulatorAddr.argtypes = [ctypes.c_void_p, ctypes.c_char_p] 
        self.dll.AUTDLinkSimulatorAddr.restype = ctypes.c_void_p

        self.dll.AUTDLinkSimulatorTimeout.argtypes = [ctypes.c_void_p, ctypes.c_uint64] 
        self.dll.AUTDLinkSimulatorTimeout.restype = ctypes.c_void_p

        self.dll.AUTDLinkSimulatorBuild.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDLinkSimulatorBuild.restype = ctypes.c_void_p
