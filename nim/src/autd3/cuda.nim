# File: cuda.nim
# Project: autd3
# Created Date: 13/06/2022
# Author: Shun Suzuki
# -----
# Last Modified: 13/06/2022
# Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
# -----
# Copyright (c) 2022 Shun Suzuki. All rights reserved.
#


import backend

import native_methods/autd3capi_backend_cuda

type BackendCUDA* = object of Backend

func initBackendCUDA*(): BackendCUDA =
    AUTDCUDABackend(result.p.addr)
