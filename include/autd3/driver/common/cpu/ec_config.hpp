// File: ec_config.hpp
// Project: cpu
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 25/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>

namespace autd3::driver {

constexpr size_t HEADER_SIZE = 128;
constexpr size_t EC_INPUT_FRAME_SIZE = 2;

constexpr uint32_t EC_CYCLE_TIME_BASE_MICRO_SEC = 500;
constexpr uint32_t EC_CYCLE_TIME_BASE_NANO_SEC = EC_CYCLE_TIME_BASE_MICRO_SEC * 1000;

}  // namespace autd3::driver
