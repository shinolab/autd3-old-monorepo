# This file is autogenerated
import threading
import ctypes
import os
from .autd3capi_def import LinkBuilderPtr


class LinkSimulatorBuilderPtr(ctypes.Structure):
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
            self.dll = ctypes.CDLL(os.path.join(bin_location, f'{bin_prefix}autd3capi_link_simulator{bin_ext}'))
        except Exception:
            return

        self.dll.AUTDLinkSimulator.argtypes = [ctypes.c_uint16] 
        self.dll.AUTDLinkSimulator.restype = LinkSimulatorBuilderPtr

        self.dll.AUTDLinkSimulatorWithAddr.argtypes = [LinkSimulatorBuilderPtr, ctypes.c_char_p, ctypes.c_char_p]  # type: ignore 
        self.dll.AUTDLinkSimulatorWithAddr.restype = LinkSimulatorBuilderPtr

        self.dll.AUTDLinkSimulatorWithTimeout.argtypes = [LinkSimulatorBuilderPtr, ctypes.c_uint64]  # type: ignore 
        self.dll.AUTDLinkSimulatorWithTimeout.restype = LinkSimulatorBuilderPtr

        self.dll.AUTDLinkSimulatorIntoBuilder.argtypes = [LinkSimulatorBuilderPtr]  # type: ignore 
        self.dll.AUTDLinkSimulatorIntoBuilder.restype = LinkBuilderPtr

    def link_simulator(self, port: int) -> LinkSimulatorBuilderPtr:
        return self.dll.AUTDLinkSimulator(port)

    def link_simulator_with_addr(self, simulator: LinkSimulatorBuilderPtr, addr: bytes, err: ctypes.Array[ctypes.c_char]) -> LinkSimulatorBuilderPtr:
        return self.dll.AUTDLinkSimulatorWithAddr(simulator, addr, err)

    def link_simulator_with_timeout(self, simulator: LinkSimulatorBuilderPtr, timeout_ns: int) -> LinkSimulatorBuilderPtr:
        return self.dll.AUTDLinkSimulatorWithTimeout(simulator, timeout_ns)

    def link_simulator_into_builder(self, simulator: LinkSimulatorBuilderPtr) -> LinkBuilderPtr:
        return self.dll.AUTDLinkSimulatorIntoBuilder(simulator)
