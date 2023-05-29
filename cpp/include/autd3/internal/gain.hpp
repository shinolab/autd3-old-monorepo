// File: gain.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 29/05/2023
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
  Gain(void* ptr) : Body(ptr) {}
  ~Gain() {
    if (_ptr != nullptr) {
      native_methods::AUTDDeleteGain(_ptr);
    }
  }

  [[nodiscard]] virtual void* calc_ptr(const Geometry& _geometry) { return _ptr; }

  void set_released() { _ptr = nullptr; }
};

}  // namespace autd3::internal
