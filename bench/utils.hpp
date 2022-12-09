// File: utils.hpp
// Project: bench
// Created Date: 09/12/2022
// Author: Shun Suzuki
// -----
// Last Modified: 09/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <autd3.hpp>

template <typename T>
void setup_autd(autd3::Controller& autd, const T x, const T y) {
  for (T iy = 0; iy < y; iy++)
    for (T ix = 0; ix < x; ix++)
      autd.geometry().add_device(autd3::AUTD3(
          autd3::Vector3(static_cast<double>(ix) * autd3::AUTD3::DEVICE_WIDTH, static_cast<double>(iy) * autd3::AUTD3::DEVICE_HEIGHT, 0.0),
          autd3::Vector3::Zero()));
}
