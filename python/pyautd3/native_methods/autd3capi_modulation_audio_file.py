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
            self.dll = ctypes.CDLL(os.path.join(bin_location, f'{bin_prefix}autd3capi-modulation-audio-file{bin_ext}'))
        except FileNotFoundError:
            return

        self.dll.AUTDModulationWav.argtypes = [ctypes.c_char_p, ctypes.c_char_p] 
        self.dll.AUTDModulationWav.restype = ctypes.c_void_p

    def modulation_wav(self, path: bytes, err: ctypes.Array[ctypes.c_char]) -> ctypes.c_void_p:
        return self.dll.AUTDModulationWav(path, err)
