// File: defined.hpp
// Project: driver
// Created Date: 25/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 28/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#if __cplusplus >= 202002L
#include <numbers>
#endif

#include <Eigen/Geometry>

namespace autd3::driver {

#ifdef AUTD3_USE_SINGLE_FLOAT
using float_t = float;
#else
using float_t = double;
#endif

#ifdef AUTD3_USE_METER
constexpr float_t METER = 1;
#else
constexpr float_t METER = 1000;
#endif
constexpr float_t MILLIMETER = METER / 1000;

#if __cplusplus >= 202002L
constexpr float_t pi = std::numbers::pi_v<float_t>;
#else
constexpr float_t pi = static_cast<float_t>(3.141592653589793238462643383279502884L);
#endif

using Vector3 = Eigen::Matrix<float_t, 3, 1>;
using Vector4 = Eigen::Matrix<float_t, 4, 1>;
using Matrix4X4 = Eigen::Matrix<float_t, 4, 4>;
using Matrix3X3 = Eigen::Matrix<float_t, 3, 3>;
using Quaternion = Eigen::Quaternion<float_t>;
using Affine3 = Eigen::Transform<float_t, 3, Eigen::Affine>;

constexpr uint8_t VERSION_NUM_MAJOR = 0x88;
constexpr uint8_t VERSION_NUM_MINOR = 0x01;

constexpr uint16_t MAX_CYCLE = 8191;

constexpr uint32_t MOD_SAMPLING_FREQ_DIV_MIN = 1160;
constexpr size_t MOD_BUF_SIZE_MAX = 65536;

constexpr uint32_t FOCUS_STM_SAMPLING_FREQ_DIV_MIN = 1612;
constexpr uint32_t GAIN_STM_SAMPLING_FREQ_DIV_MIN = 276;
constexpr uint32_t GAIN_STM_LEGACY_SAMPLING_FREQ_DIV_MIN = 152;
constexpr size_t FOCUS_STM_BUF_SIZE_MAX = 65536;
constexpr size_t GAIN_STM_BUF_SIZE_MAX = 1024;
constexpr size_t GAIN_STM_LEGACY_BUF_SIZE_MAX = 2048;

constexpr uint16_t SILENCER_CYCLE_MIN = 1044;

}  // namespace autd3::driver
