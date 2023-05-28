# This file is autogenerated
import threading
import ctypes
import os




class Singleton(type):
    _instances = {} # type: ignore
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
            self.dll = ctypes.CDLL(os.path.join(bin_location, f'{bin_prefix}autd3capi_link_twincat{bin_ext}'))
        except FileNotFoundError:
            return

        self.dll.AUTDLinkTwinCAT.argtypes = [] 
        self.dll.AUTDLinkTwinCAT.restype = ctypes.c_void_p

        self.dll.AUTDLinkTwinCATTimeout.argtypes = [ctypes.c_void_p, ctypes.c_uint64] 
        self.dll.AUTDLinkTwinCATTimeout.restype = ctypes.c_void_p

        self.dll.AUTDLinkTwinCATBuild.argtypes = [ctypes.c_void_p, ctypes.c_char_p] 
        self.dll.AUTDLinkTwinCATBuild.restype = ctypes.c_void_p

        self.dll.AUTDLinkRemoteTwinCAT.argtypes = [ctypes.c_char_p] 
        self.dll.AUTDLinkRemoteTwinCAT.restype = ctypes.c_void_p

        self.dll.AUTDLinkRemoteTwinCATServerIP.argtypes = [ctypes.c_void_p, ctypes.c_char_p] 
        self.dll.AUTDLinkRemoteTwinCATServerIP.restype = ctypes.c_void_p

        self.dll.AUTDLinkRemoteTwinCATClientAmsNetId.argtypes = [ctypes.c_void_p, ctypes.c_char_p] 
        self.dll.AUTDLinkRemoteTwinCATClientAmsNetId.restype = ctypes.c_void_p

        self.dll.AUTDLinkRemoteTwinCATTimeout.argtypes = [ctypes.c_void_p, ctypes.c_uint64] 
        self.dll.AUTDLinkRemoteTwinCATTimeout.restype = ctypes.c_void_p

        self.dll.AUTDLinkRemoteTwinCATBuild.argtypes = [ctypes.c_void_p, ctypes.c_char_p] 
        self.dll.AUTDLinkRemoteTwinCATBuild.restype = ctypes.c_void_p

    def link_twin_cat(self) -> ctypes.c_void_p:
        return self.dll.AUTDLinkTwinCAT()

    def link_twin_cat_timeout(self, builder: ctypes.c_void_p, timeout_ns: int) -> ctypes.c_void_p:
        return self.dll.AUTDLinkTwinCATTimeout(builder, timeout_ns)

    def link_twin_cat_build(self, builder: ctypes.c_void_p, err: ctypes.Array[ctypes.c_char]) -> ctypes.c_void_p:
        return self.dll.AUTDLinkTwinCATBuild(builder, err)

    def link_remote_twin_cat(self, server_ams_net_id: bytes) -> ctypes.c_void_p:
        return self.dll.AUTDLinkRemoteTwinCAT(server_ams_net_id)

    def link_remote_twin_cat_server_ip(self, builder: ctypes.c_void_p, addr: bytes) -> ctypes.c_void_p:
        return self.dll.AUTDLinkRemoteTwinCATServerIP(builder, addr)

    def link_remote_twin_cat_client_ams_net_id(self, builder: ctypes.c_void_p, id: bytes) -> ctypes.c_void_p:
        return self.dll.AUTDLinkRemoteTwinCATClientAmsNetId(builder, id)

    def link_remote_twin_cat_timeout(self, builder: ctypes.c_void_p, timeout_ns: int) -> ctypes.c_void_p:
        return self.dll.AUTDLinkRemoteTwinCATTimeout(builder, timeout_ns)

    def link_remote_twin_cat_build(self, builder: ctypes.c_void_p, err: ctypes.Array[ctypes.c_char]) -> ctypes.c_void_p:
        return self.dll.AUTDLinkRemoteTwinCATBuild(builder, err)
