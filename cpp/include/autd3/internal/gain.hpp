// File: gain.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 30/05/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/datagram.hpp"
#include "autd3/internal/geometry.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::internal {

class Gain : public Body {
 public:
  explicit Gain(void* ptr) : Body(ptr) {}
  Gain(const Gain& v) noexcept = default;
  Gain& operator=(const Gain& obj) = default;
  Gain(Gain&& obj) = default;
  Gain& operator=(Gain&& obj) = default;
  ~Gain() override {
    if (_ptr != nullptr) {
      native_methods::AUTDDeleteGain(_ptr);
    }
  }

  [[nodiscard]] void* calc_ptr(const Geometry&) override { return _ptr; }

  void set_released() { _ptr = nullptr; }
};

}  // namespace autd3::internal
