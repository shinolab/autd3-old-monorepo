"""
File: backend_cuda.py
Project: holo
Created Date: 08/06/2023
Author: Shun Suzuki
-----
Last Modified: 08/06/2023
Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
-----
Copyright (c) 2023 Shun Suzuki. All rights reserved.

"""


import ctypes
from pyautd3.native_methods.autd3capi_backend_cuda import (
    NativeMethods as AUTD3BackendCUDA,
)
from .backend import Backend
from pyautd3.autd_error import AUTDError


class CUDABackend(Backend):
    def __init__(self):
        err = ctypes.create_string_buffer(256)
        ptr = AUTD3BackendCUDA().cuda_backend(err)
        if ptr._0 is None:
            raise AUTDError(err)
        super().__init__(ptr)
