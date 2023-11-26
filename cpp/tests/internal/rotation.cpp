// File: rotation.cpp
// Project: internal
// Created Date: 26/11/2023
// Author: Shun Suzuki
// -----
// Last Modified: 26/11/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include <autd3/internal/controller.hpp>
#include <autd3/internal/rotation.hpp>
#include <autd3/link/audit.hpp>
#include <numbers>

TEST(Internal, Angle) {
  ASSERT_EQ((90.0 * autd3::internal::deg).to_radian(), std::numbers::pi / 2);
  ASSERT_EQ((std::numbers::pi / 2 * autd3::internal::rad).to_radian(), std::numbers::pi / 2);
}

static inline autd3::internal::Controller<autd3::link::Audit> open_with_rotation(const autd3::internal::Quaternion& q) {
  return autd3::internal::ControllerBuilder()
      .add_device(autd3::internal::AUTD3(autd3::internal::Vector3::Zero()).with_rotation(q))
      .open_with_async(autd3::link::Audit::builder())
      .get();
}

TEST(Internal, WithRotation) {
  using autd3::internal::deg;
  using autd3::internal::EulerAngles;
  using autd3::internal::Vector3;

  {
    const auto autd = open_with_rotation(EulerAngles::from_zyz(90.0 * deg, 0.0 * deg, 0.0 * deg));
    ASSERT_EQ(Vector3::UnitY(), autd.geometry()[0][0].x_direction());
    ASSERT_EQ(-Vector3::UnitX(), autd.geometry()[0][0].y_direction());
    ASSERT_EQ(Vector3::UnitZ(), autd.geometry()[0][0].z_direction());
  }
  {
    const auto autd = open_with_rotation(EulerAngles::from_zyz(0.0 * deg, 90.0 * deg, 0.0 * deg));
    ASSERT_EQ(-Vector3::UnitZ(), autd.geometry()[0][0].x_direction());
    ASSERT_EQ(Vector3::UnitY(), autd.geometry()[0][0].y_direction());
    ASSERT_EQ(Vector3::UnitX(), autd.geometry()[0][0].z_direction());
  }
  {
    const auto autd = open_with_rotation(EulerAngles::from_zyz(0.0 * deg, 0.0 * deg, 90.0 * deg));
    ASSERT_EQ(Vector3::UnitY(), autd.geometry()[0][0].x_direction());
    ASSERT_EQ(-Vector3::UnitX(), autd.geometry()[0][0].y_direction());
    ASSERT_EQ(Vector3::UnitZ(), autd.geometry()[0][0].z_direction());
  }
  {
    const auto autd = open_with_rotation(EulerAngles::from_zyz(0.0 * deg, 90.0 * deg, 90.0 * deg));
    ASSERT_EQ(Vector3::UnitY(), autd.geometry()[0][0].x_direction());
    ASSERT_EQ(Vector3::UnitZ(), autd.geometry()[0][0].y_direction());
    ASSERT_EQ(Vector3::UnitX(), autd.geometry()[0][0].z_direction());
  }
  {
    const auto autd = open_with_rotation(EulerAngles::from_zyz(90.0 * deg, 90.0 * deg, 0.0 * deg));
    ASSERT_EQ(-Vector3::UnitZ(), autd.geometry()[0][0].x_direction());
    ASSERT_EQ(-Vector3::UnitX(), autd.geometry()[0][0].y_direction());
    ASSERT_EQ(Vector3::UnitY(), autd.geometry()[0][0].z_direction());
  }
}