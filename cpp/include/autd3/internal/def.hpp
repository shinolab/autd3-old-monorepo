// File: def.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 29/05/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#if __cplusplus >= 202002L
#include <numbers>
#endif

#include <Eigen/Geometry>

namespace autd3::internal {

using Vector3 = Eigen::Matrix<float_t, 3, 1>;
using Vector4 = Eigen::Matrix<float_t, 4, 1>;
using Matrix4X4 = Eigen::Matrix<float_t, 4, 4>;
using Matrix3X3 = Eigen::Matrix<float_t, 3, 3>;
using Quaternion = Eigen::Quaternion<float_t>;
using Affine3 = Eigen::Transform<float_t, 3, Eigen::Affine>;

#if __cplusplus >= 202002L
constexpr float_t pi = std::numbers::pi_v<float_t>;
#else
constexpr float_t pi = static_cast<float_t>(3.141592653589793238462643383279502884L);
#endif

}  // namespace autd3::internal