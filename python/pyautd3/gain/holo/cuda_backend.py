'''
File: cuda_backend.py
Project: holo
Created Date: 21/10/2022
Author: Shun Suzuki
-----
Last Modified: 21/10/2022
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2022 Shun Suzuki. All rights reserved.

'''

from ctypes import byref

from pyautd3.native_methods.autd3capi_backend_cuda import NativeMethods as BackendCUDA
from .backend import Backend


class CUDABackend(Backend):
    def __init__(self):
        super().__init__()
        BackendCUDA().init_dll()
        BackendCUDA().dll.AUTDCUDABackend(byref(self.ptr))

    def __del__(self):
        super().__del__()
