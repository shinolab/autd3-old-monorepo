// File: hardware.hpp
// Project: driver
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 11/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

namespace autd3::driver {
constexpr size_t NUM_TRANS_IN_UNIT = 249;
constexpr size_t NUM_TRANS_X = 18;
constexpr size_t NUM_TRANS_Y = 14;
constexpr double TRANS_SPACING_MM = 10.16;
constexpr double DEVICE_WIDTH = 192.0;
constexpr double DEVICE_HEIGHT = 151.4;

template <typename T>
auto is_missing_transducer(T x, T y) -> std::enable_if_t<std::is_integral_v<T>, bool> {
  return y == 1 && (x == 1 || x == 2 || x == 16);
}

}  // namespace autd3::driver
