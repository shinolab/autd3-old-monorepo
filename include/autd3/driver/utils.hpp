// File: utils.hpp
// Project: driver
// Created Date: 15/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 30/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

namespace autd3::driver {

template <typename T>
auto rem_euclid(T a, T b) noexcept -> std::enable_if_t<std::is_signed_v<T>, T> {
  T m = a % b;
  if (m < 0) m = b < 0 ? m - b : m + b;
  return m;
}

}  // namespace autd3::driver
