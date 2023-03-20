// File: backend_arrayfire.hpp
// Project: gain
// Created Date: 08/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 14/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <arrayfire.h>

#include "autd3/gain/backend.hpp"

namespace autd3::gain::holo {

/**
 * \brief Backend for Holo using ArrayFire
 */
class ArrayFireBackend final {
 public:
  ArrayFireBackend() = default;
  ~ArrayFireBackend() = default;
  ArrayFireBackend(const ArrayFireBackend& v) noexcept = default;
  ArrayFireBackend& operator=(const ArrayFireBackend& obj) = default;
  ArrayFireBackend(ArrayFireBackend&& obj) = default;
  ArrayFireBackend& operator=(ArrayFireBackend&& obj) = default;

  ArrayFireBackend& device_idx(const af::Backend backend) {
    _backend = backend;
    return *this;
  }

  [[nodiscard]] BackendPtr build() const;

 private:
  af::Backend _backend{af::Backend::AF_BACKEND_DEFAULT};
};

}  // namespace autd3::gain::holo
