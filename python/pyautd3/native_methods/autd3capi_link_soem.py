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
        self.dll = ctypes.CDLL(os.path.join(self.bin, f'{self.prefix}autd3capi-link-soem{self.ext}'))

        self.dll.AUTDGetAdapterPointer.argtypes = [ctypes.POINTER(ctypes.c_void_p)] 
        self.dll.AUTDGetAdapterPointer.restype = ctypes.c_int32

        self.dll.AUTDGetAdapter.argtypes = [ctypes.c_void_p, ctypes.c_int32, ctypes.c_char_p, ctypes.c_char_p] 
        self.dll.AUTDGetAdapter.restype = None

        self.dll.AUTDFreeAdapterPointer.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDFreeAdapterPointer.restype = None

        self.dll.AUTDLinkSOEM.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_char_p, ctypes.c_uint16, ctypes.c_uint16, ctypes.c_bool, ctypes.c_void_p, ctypes.c_bool, ctypes.c_uint64] 
        self.dll.AUTDLinkSOEM.restype = None

        self.dll.AUTDLinkSOEMSetLogLevel.argtypes = [ctypes.c_int32] 
        self.dll.AUTDLinkSOEMSetLogLevel.restype = None

        self.dll.AUTDLinkSOEMSetDefaultLogger.argtypes = [ctypes.c_void_p, ctypes.c_void_p] 
        self.dll.AUTDLinkSOEMSetDefaultLogger.restype = None
