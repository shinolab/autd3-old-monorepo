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
        self.dll = ctypes.CDLL(os.path.join(self.bin, f'{self.prefix}autd3capi-geometry-viewer{self.ext}'))


        self.dll.AUTDGeometryViewer.argtypes = [] 
        self.dll.AUTDGeometryViewer.restype = ctypes.c_void_p

        self.dll.AUTDGeometryViewerSize.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.c_uint32] 
        self.dll.AUTDGeometryViewerSize.restype = ctypes.c_void_p

        self.dll.AUTDGeometryViewerVsync.argtypes = [ctypes.c_void_p, ctypes.c_bool] 
        self.dll.AUTDGeometryViewerVsync.restype = ctypes.c_void_p

        self.dll.AUTDGeometryViewerRun.argtypes = [ctypes.c_void_p, ctypes.c_void_p] 
        self.dll.AUTDGeometryViewerRun.restype = ctypes.c_int32
