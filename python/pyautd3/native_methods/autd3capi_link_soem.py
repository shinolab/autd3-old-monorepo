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
        self.dll = ctypes.CDLL(os.path.join(self.bin, f'{self.prefix}autd3capi-link-soem{self.ext}'))


        self.dll.AUTDGetAdapterPointer.argtypes = [ctypes.POINTER(ctypes.c_uint32)] 
        self.dll.AUTDGetAdapterPointer.restype = ctypes.c_void_p

        self.dll.AUTDGetAdapter.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.c_char_p, ctypes.c_char_p] 
        self.dll.AUTDGetAdapter.restype = None

        self.dll.AUTDFreeAdapterPointer.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDFreeAdapterPointer.restype = None

        self.dll.AUTDLinkSOEM.argtypes = [] 
        self.dll.AUTDLinkSOEM.restype = ctypes.c_void_p

        self.dll.AUTDLinkSOEMSendCycle.argtypes = [ctypes.c_void_p, ctypes.c_uint16] 
        self.dll.AUTDLinkSOEMSendCycle.restype = ctypes.c_void_p

        self.dll.AUTDLinkSOEMSync0Cycle.argtypes = [ctypes.c_void_p, ctypes.c_uint16] 
        self.dll.AUTDLinkSOEMSync0Cycle.restype = ctypes.c_void_p

        self.dll.AUTDLinkSOEMBufSize.argtypes = [ctypes.c_void_p, ctypes.c_uint32] 
        self.dll.AUTDLinkSOEMBufSize.restype = ctypes.c_void_p

        self.dll.AUTDLinkSOEMTimerStrategy.argtypes = [ctypes.c_void_p, TimerStrategy] 
        self.dll.AUTDLinkSOEMTimerStrategy.restype = ctypes.c_void_p

        self.dll.AUTDLinkSOEMSyncMode.argtypes = [ctypes.c_void_p, SyncMode] 
        self.dll.AUTDLinkSOEMSyncMode.restype = ctypes.c_void_p

        self.dll.AUTDLinkSOEMIfname.argtypes = [ctypes.c_void_p, ctypes.c_char_p] 
        self.dll.AUTDLinkSOEMIfname.restype = ctypes.c_void_p

        self.dll.AUTDLinkSOEMStateCheckInterval.argtypes = [ctypes.c_void_p, ctypes.c_uint32] 
        self.dll.AUTDLinkSOEMStateCheckInterval.restype = ctypes.c_void_p

        self.dll.AUTDLinkSOEMOnLost.argtypes = [ctypes.c_void_p, ctypes.c_void_p] 
        self.dll.AUTDLinkSOEMOnLost.restype = ctypes.c_void_p

        self.dll.AUTDLinkSOEMLogLevel.argtypes = [ctypes.c_void_p, Level] 
        self.dll.AUTDLinkSOEMLogLevel.restype = ctypes.c_void_p

        self.dll.AUTDLinkSOEMLogFunc.argtypes = [ctypes.c_void_p, Level, ctypes.c_void_p, ctypes.c_void_p] 
        self.dll.AUTDLinkSOEMLogFunc.restype = ctypes.c_void_p

        self.dll.AUTDLinkSOEMTimeout.argtypes = [ctypes.c_void_p, ctypes.c_uint64] 
        self.dll.AUTDLinkSOEMTimeout.restype = ctypes.c_void_p

        self.dll.AUTDLinkSOEMBuild.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDLinkSOEMBuild.restype = ctypes.c_void_p

        self.dll.AUTDLinkRemoteSOEM.argtypes = [ctypes.c_char_p, ctypes.c_uint16] 
        self.dll.AUTDLinkRemoteSOEM.restype = ctypes.c_void_p

        self.dll.AUTDLinkRemoteSOEMTimeout.argtypes = [ctypes.c_void_p, ctypes.c_uint64] 
        self.dll.AUTDLinkRemoteSOEMTimeout.restype = ctypes.c_void_p

        self.dll.AUTDLinkRemoteSOEMBuild.argtypes = [ctypes.c_void_p] 
        self.dll.AUTDLinkRemoteSOEMBuild.restype = ctypes.c_void_p

class TimerStrategy(IntEnum):
    Sleep = 0
    NativeTimer = 1
    BusyWait = 2

    @classmethod
    def from_param(cls, obj):
            return int(obj)

class SyncMode(IntEnum):
    FreeRun = 0
    DC = 1

    @classmethod
    def from_param(cls, obj):
            return int(obj)
