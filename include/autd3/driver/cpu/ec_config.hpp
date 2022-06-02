// File: ec_config.hpp
// Project: cpu
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 30/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>

namespace autd3::driver {

constexpr size_t HEADER_SIZE = 128;
constexpr size_t BODY_SIZE = 498;
constexpr size_t EC_OUTPUT_FRAME_SIZE = HEADER_SIZE + BODY_SIZE;
constexpr size_t EC_INPUT_FRAME_SIZE = 2;

constexpr uint32_t EC_SYNC0_CYCLE_TIME_MICRO_SEC = 500;
constexpr uint32_t EC_SYNC0_CYCLE_TIME_NANO_SEC = EC_SYNC0_CYCLE_TIME_MICRO_SEC * 1000;

constexpr size_t EC_DEVICE_PER_FRAME = 2;
constexpr size_t EC_FRAME_LENGTH = 14 + 2 + (10 + EC_OUTPUT_FRAME_SIZE + EC_INPUT_FRAME_SIZE + 2) * EC_DEVICE_PER_FRAME + 4;
constexpr double EC_SPEED_BPS = 100.0 * 1000.0 * 1000.0;
constexpr double EC_TRAFFIC_DELAY = EC_FRAME_LENGTH * 8.0 / EC_SPEED_BPS;

}  // namespace autd3::driver
