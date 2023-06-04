// File: link.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 03/06/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/native_methods.hpp"

namespace autd3::internal {

using LogOutCallback = void (*)(const char* msg);
using LogFlushCallback = void (*)();

class Link {
 public:
  explicit Link(const native_methods::LinkPtr ptr) : _ptr(ptr) {}

  [[nodiscard]] native_methods::LinkPtr ptr() const { return _ptr; }

 protected:
  native_methods::LinkPtr _ptr;
};
}  // namespace autd3::internal
