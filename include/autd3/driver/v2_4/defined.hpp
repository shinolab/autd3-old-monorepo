// File: defined.hpp
// Project: v2_4
// Created Date: 15/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 15/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>

namespace autd3::driver::v2_4 {

constexpr uint16_t MAX_CYCLE = 8191;

constexpr uint32_t MOD_SAMPLING_FREQ_DIV_MIN = 1160;
constexpr size_t MOD_BUF_SIZE_MAX = 65536;

constexpr uint32_t POINT_STM_SAMPLING_FREQ_DIV_MIN = 1612;
constexpr uint32_t GAIN_STM_SAMPLING_FREQ_DIV_MIN = 276;
constexpr uint32_t GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN = 152;
constexpr size_t POINT_STM_BUF_SIZE_MAX = 65536;
constexpr size_t GAIN_STM_BUF_SIZE_MAX = 1024;
constexpr size_t GAIN_STM_LEGACY_BUF_SIZE_MAX = 2048;

constexpr uint16_t SILENCER_CYCLE_MIN = 1044;

}  // namespace autd3::driver::v2_4
