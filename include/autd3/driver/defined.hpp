// File: defined.hpp
// Project: driver
// Created Date: 25/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 04/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#if __cplusplus >= 202002L
#include <numbers>
#endif

#if _MSC_VER
#pragma warning(push)
#pragma warning( \
    disable : 4068 6031 6255 6294 26408 26450 26426 26429 26432 26434 26440 26446 26447 26451 26454 26455 26461 26462 26471 26472 26474 26475 26495 26481 26482 26485 26490 26491 26493 26494 26496 26497 26812 26813 26814)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmaybe-uninitialized"
#pragma GCC diagnostic ignored "-Wclass-memaccess"
#endif
#include <Eigen/Dense>
#if _MSC_VER
#pragma warning(pop)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic pop
#endif

namespace autd3::driver {

#ifdef AUTD3_USE_SINGLE_FLOAT
using autd3_float_t = float;
#else
using autd3_float_t = double;
#endif

#if __cplusplus >= 202002L
constexpr autd3_float_t pi = std::numbers::pi_v<autd3_float_t>;
#else
constexpr autd3_float_t pi = static_cast<autd3_float_t>(3.141592653589793238462643383279502884L);
#endif

using Vector3 = Eigen::Matrix<autd3_float_t, 3, 1>;
using Vector4 = Eigen::Matrix<autd3_float_t, 4, 1>;
using Matrix4X4 = Eigen::Matrix<autd3_float_t, 4, 4>;
using Quaternion = Eigen::Quaternion<autd3_float_t>;

constexpr uint8_t VERSION_NUM = 0x87;

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
