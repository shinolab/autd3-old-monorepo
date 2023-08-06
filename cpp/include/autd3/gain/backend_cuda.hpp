// File: backend_cuda.hpp
// Project: gain
// Created Date: 08/06/2023
// Author: Shun Suzuki
// -----
// Last Modified: 03/08/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/gain/holo.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::gain::holo {

/**
 * @brief Backend using CUDA
 */
class CUDABackend final : public Backend {
 public:
  CUDABackend() : Backend() {
    char err[256];
    _ptr = internal::native_methods::AUTDCUDABackend(err);
    if (_ptr._0 == nullptr) throw internal::AUTDException(err);
  }
  ~CUDABackend() override = default;
};

}  // namespace autd3::gain::holo
