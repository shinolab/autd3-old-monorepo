// File: rotation.cpp
// Project: internal
// Created Date: 26/11/2023
// Author: Shun Suzuki
// -----
// Last Modified: 02/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include <autd3/internal/controller.hpp>
#include <autd3/internal/geometry/rotation.hpp>
#include <autd3/link/audit.hpp>
#include <numbers>

#include "utils.hpp"

TEST(Internal, Angle) {
  ASSERT_NEAR((90.0 * autd3::internal::geometry::deg).to_radian(), std::numbers::pi / 2, 1e-6);
  ASSERT_NEAR((std::numbers::pi / 2 * autd3::internal::geometry::rad).to_radian(), std::numbers::pi / 2, 1e-6);
}

static inline autd3::internal::Controller<autd3::link::Audit> open_with_rotation(const autd3::internal::Quaternion& q) {
  return autd3::internal::ControllerBuilder()
      .add_device(autd3::internal::geometry::AUTD3(autd3::internal::Vector3::Zero()).with_rotation(q))
      .open_with_async(autd3::link::Audit::builder())
      .get();
}

TEST(Internal, WithRotation) {
  using autd3::internal::Vector3;
  using autd3::internal::geometry::deg;
  using autd3::internal::geometry::EulerAngles;

  {
    const auto autd = open_with_rotation(EulerAngles::from_zyz(90.0 * deg, 0.0 * deg, 0.0 * deg));
    ASSERT_NEAR_VECTOR3(Vector3::UnitY(), autd.geometry()[0][0].x_direction(), 1e-6);
    ASSERT_NEAR_VECTOR3(-Vector3::UnitX(), autd.geometry()[0][0].y_direction(), 1e-6);
    ASSERT_NEAR_VECTOR3(Vector3::UnitZ(), autd.geometry()[0][0].z_direction(), 1e-6);
  }
  {
    const auto autd = open_with_rotation(EulerAngles::from_zyz(0.0 * deg, 90.0 * deg, 0.0 * deg));
    ASSERT_NEAR_VECTOR3(-Vector3::UnitZ(), autd.geometry()[0][0].x_direction(), 1e-6);
    ASSERT_NEAR_VECTOR3(Vector3::UnitY(), autd.geometry()[0][0].y_direction(), 1e-6);
    ASSERT_NEAR_VECTOR3(Vector3::UnitX(), autd.geometry()[0][0].z_direction(), 1e-6);
  }
  {
    const auto autd = open_with_rotation(EulerAngles::from_zyz(0.0 * deg, 0.0 * deg, 90.0 * deg));
    ASSERT_NEAR_VECTOR3(Vector3::UnitY(), autd.geometry()[0][0].x_direction(), 1e-6);
    ASSERT_NEAR_VECTOR3(-Vector3::UnitX(), autd.geometry()[0][0].y_direction(), 1e-6);
    ASSERT_NEAR_VECTOR3(Vector3::UnitZ(), autd.geometry()[0][0].z_direction(), 1e-6);
  }
  {
    const auto autd = open_with_rotation(EulerAngles::from_zyz(0.0 * deg, 90.0 * deg, 90.0 * deg));
    ASSERT_NEAR_VECTOR3(Vector3::UnitY(), autd.geometry()[0][0].x_direction(), 1e-6);
    ASSERT_NEAR_VECTOR3(Vector3::UnitZ(), autd.geometry()[0][0].y_direction(), 1e-6);
    ASSERT_NEAR_VECTOR3(Vector3::UnitX(), autd.geometry()[0][0].z_direction(), 1e-6);
  }
  {
    const auto autd = open_with_rotation(EulerAngles::from_zyz(90.0 * deg, 90.0 * deg, 0.0 * deg));
    ASSERT_NEAR_VECTOR3(-Vector3::UnitZ(), autd.geometry()[0][0].x_direction(), 1e-6);
    ASSERT_NEAR_VECTOR3(-Vector3::UnitX(), autd.geometry()[0][0].y_direction(), 1e-6);
    ASSERT_NEAR_VECTOR3(Vector3::UnitY(), autd.geometry()[0][0].z_direction(), 1e-6);
  }
}