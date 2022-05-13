// File: kernel.h
// Project: cuda
// Created Date: 13/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 13/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <cuComplex.h>

#include <cstdint>

namespace autd3 {
namespace gain {
namespace holo {

constexpr uint32_t BLOCK_SIZE = 32;

void cu_make_complex(const double* r, const double* i, uint32_t row, uint32_t col, cuDoubleComplex* c);
}  // namespace holo
}  // namespace gain
}  // namespace autd3
