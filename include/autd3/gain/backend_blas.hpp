// File: backend_blas.hpp
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
 * \brief Backend for Holo using BLAS
 */
class BLASBackend final {
 public:
  BLASBackend() = default;
  ~BLASBackend() = default;
  BLASBackend(const BLASBackend& v) = default;
  BLASBackend& operator=(const BLASBackend& obj) = default;
  BLASBackend(BLASBackend&& obj) = default;
  BLASBackend& operator=(BLASBackend&& obj) = default;

  [[nodiscard]] BackendPtr build() const;

  [[deprecated("Use BLASBackend().build() instead.")]] static BackendPtr create();
};

}  // namespace autd3::gain::holo
