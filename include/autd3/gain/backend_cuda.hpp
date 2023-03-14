// File: backend_cuda.hpp
// Project: gain
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 14/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/gain/backend.hpp"

namespace autd3::gain::holo {

/**
 * \brief Backend for Holo using CUDA
 */
class CUDABackend {
 public:
  CUDABackend() = default;
  ~CUDABackend() = default;
  CUDABackend(const CUDABackend& v) noexcept = default;
  CUDABackend& operator=(const CUDABackend& obj) = default;
  CUDABackend(CUDABackend&& obj) = default;
  CUDABackend& operator=(CUDABackend&& obj) = default;

  CUDABackend& device_idx(const int device_idx) {
    _device_idx = device_idx;
    return *this;
  }

  [[nodiscard]] BackendPtr build() const;

  [[deprecated("Use CUDABackend().build() instead.")]] static BackendPtr create(int device_idx);

 private:
  int _device_idx{0};
};

}  // namespace autd3::gain::holo
