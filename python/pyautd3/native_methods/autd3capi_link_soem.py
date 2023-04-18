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

        self.dll.AUTDLinkSOEM.argtypes = [ctypes.POINTER(ctypes.c_void_p)] 
        self.dll.AUTDLinkSOEM.restype = None

        self.dll.AUTDLinkSOEMIfname.argtypes = [ctypes.c_void_p, ctypes.c_char_p] 
        self.dll.AUTDLinkSOEMIfname.restype = None

        self.dll.AUTDLinkSOEMBufSize.argtypes = [ctypes.c_void_p, ctypes.c_uint64] 
        self.dll.AUTDLinkSOEMBufSize.restype = None

        self.dll.AUTDLinkSOEMSync0Cycle.argtypes = [ctypes.c_void_p, ctypes.c_uint16] 
        self.dll.AUTDLinkSOEMSync0Cycle.restype = None

        self.dll.AUTDLinkSOEMSendCycle.argtypes = [ctypes.c_void_p, ctypes.c_uint16] 
        self.dll.AUTDLinkSOEMSendCycle.restype = None

        self.dll.AUTDLinkSOEMFreerun.argtypes = [ctypes.c_void_p, ctypes.c_bool] 
        self.dll.AUTDLinkSOEMFreerun.restype = None

        self.dll.AUTDLinkSOEMOnLost.argtypes = [ctypes.c_void_p, ctypes.c_void_p] 
        self.dll.AUTDLinkSOEMOnLost.restype = None

        self.dll.AUTDLinkSOEMTimerStrategy.argtypes = [ctypes.c_void_p, ctypes.c_uint8] 
        self.dll.AUTDLinkSOEMTimerStrategy.restype = None

        self.dll.AUTDLinkSOEMStateCheckInterval.argtypes = [ctypes.c_void_p, ctypes.c_uint64] 
        self.dll.AUTDLinkSOEMStateCheckInterval.restype = None

        self.dll.AUTDLinkSOEMLogLevel.argtypes = [ctypes.c_void_p, ctypes.c_int32] 
        self.dll.AUTDLinkSOEMLogLevel.restype = None

        self.dll.AUTDLinkSOEMLogFunc.argtypes = [ctypes.c_void_p, ctypes.c_void_p, ctypes.c_void_p] 
        self.dll.AUTDLinkSOEMLogFunc.restype = None

        self.dll.AUTDLinkSOEMTimeout.argtypes = [ctypes.c_void_p, ctypes.c_uint64] 
        self.dll.AUTDLinkSOEMTimeout.restype = None

        self.dll.AUTDLinkSOEMBuild.argtypes = [ctypes.POINTER(ctypes.c_void_p), ctypes.c_void_p] 
        self.dll.AUTDLinkSOEMBuild.restype = None

        self.dll.AUTDLinkSOEMDelete.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDLinkSOEMDelete.restype = None
