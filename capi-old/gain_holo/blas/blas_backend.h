// File: blas_backend.h
// Project: blas
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 30/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "../../base/header.hpp"

#ifdef __cplusplus
extern "C" {
#endif
EXPORT_AUTD void AUTDBLASBackend(OUT void** out);
#ifdef __cplusplus
}
#endif
