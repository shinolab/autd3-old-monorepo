// File: utils.hpp
// Project: core
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 11/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <type_traits>

namespace autd3::core {

template <typename T>
auto rem_euclid(T a, T b) noexcept -> std::enable_if_t<std::is_signed_v<T>, T> {
  T m = a % b;
  if (m < 0) m = b < 0 ? m - b : m + b;
  return m;
}
}  // namespace autd3::core
