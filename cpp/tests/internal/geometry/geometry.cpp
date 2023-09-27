// File: geometry.cpp
// Project: geometry
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 27/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include <autd3/internal/geometry/geometry.hpp>
#include <ranges>

#include "utils.hpp"

TEST(Internal_Geometry, AUTD3) {
  ASSERT_EQ(10.16, autd3::internal::AUTD3::TRANS_SPACING);
  ASSERT_EQ(192.0, autd3::internal::AUTD3::DEVICE_WIDTH);
  ASSERT_EQ(151.4, autd3::internal::AUTD3::DEVICE_HEIGHT);
  ASSERT_EQ(18, autd3::internal::AUTD3::NUM_TRANS_IN_X);
  ASSERT_EQ(14, autd3::internal::AUTD3::NUM_TRANS_IN_Y);
  ASSERT_EQ(249, autd3::internal::AUTD3::NUM_TRANS_IN_UNIT);
  ASSERT_EQ(163.84e6, autd3::internal::AUTD3::FPGA_CLK_FREQ);
  ASSERT_EQ(20.48e6, autd3::internal::AUTD3::FPGA_SUB_CLK_FREQ);
}

TEST(Internal_Geometry, GeometryNumDevices) {
  auto autd = create_controller();
  ASSERT_EQ(autd.geometry().num_devices(), 2);
}

TEST(Internal_Geometry, GeometryCenter) {
  auto autd = create_controller();
  ASSERT_EQ(autd.geometry().center(), autd3::internal::Vector3(86.62522088353406, 66.71325301204821, 0));
}
