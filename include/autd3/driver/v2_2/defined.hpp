// File: defined.hpp
// Project: v2_2
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

namespace autd3::driver::v2_2 {

constexpr uint16_t MAX_CYCLE = 8191;

constexpr uint32_t MOD_SAMPLING_FREQ_DIV_MIN = 2320;
constexpr size_t MOD_BUF_SIZE_MAX = 65536;

constexpr uint32_t POINT_STM_SAMPLING_FREQ_DIV_MIN = 3224;
constexpr uint32_t GAIN_STM_SAMPLING_FREQ_DIV_MIN = 552;
constexpr uint32_t GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN = 304;
constexpr size_t POINT_STM_BUF_SIZE_MAX = 65536;
constexpr size_t GAIN_STM_BUF_SIZE_MAX = 1024;
constexpr size_t GAIN_STM_LEGACY_BUF_SIZE_MAX = 1024;

constexpr uint16_t SILENCER_CYCLE_MIN = 2088;

}  // namespace autd3::driver::v2_2
