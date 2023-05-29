// File: link.hpp
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

namespace autd3::internal {

using LogOutCallback = void (*)(const char* msg);
using LogFlushCallback = void (*)();

class Link {
 public:
  Link(void* ptr) : _ptr(ptr) {}

  [[nodiscard]] void* ptr() const { return _ptr; }

 private:
  void* _ptr;
};
}  // namespace autd3::internal
