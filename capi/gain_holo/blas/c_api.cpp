// File: c_api.cpp
// Project: blas
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 30/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "../../base/wrapper.hpp"
#include "./blas_backend.h"
#include "autd3/gain/backend_blas.hpp"

void AUTDBLASBackend(void** out) {
  auto* b = backend_create(autd3::gain::holo::BLASBackend::create());
  *out = b;
}
