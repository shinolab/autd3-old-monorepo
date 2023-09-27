// File: def.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 27/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <Eigen/Geometry>
#include <numbers>

namespace autd3::internal {

using Vector3 = Eigen::Matrix<double, 3, 1>;
using Vector4 = Eigen::Matrix<double, 4, 1>;
using Matrix4X4 = Eigen::Matrix<double, 4, 4>;
using Matrix3X3 = Eigen::Matrix<double, 3, 3>;
using Quaternion = Eigen::Quaternion<double>;
using Affine3 = Eigen::Transform<double, 3, Eigen::Affine>;

/**
 * @brief Mathematical constant pi
 */
constexpr double pi = std::numbers::pi_v<double>;

}  // namespace autd3::internal
