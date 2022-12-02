// File: utils.cpp
// Project: driver
// Created Date: 02/12/2022
// Author: Shun Suzuki
// -----
// Last Modified: 02/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 26439 26495 26812)
#endif
#include <gtest/gtest.h>
#if _MSC_VER
#pragma warning(pop)
#endif

#include <autd3/driver/utils.hpp>

#include "autd3/driver/defined.hpp"

TEST(DriverUtils, rem_euclid) {
  using autd3::driver::rem_euclid;
  ASSERT_EQ(rem_euclid(0, 256), 0);
  ASSERT_EQ(rem_euclid(10, 256), 10);
  ASSERT_EQ(rem_euclid(255, 256), 255);
  ASSERT_EQ(rem_euclid(256, 256), 0);
  ASSERT_EQ(rem_euclid(266, 256), 10);
  ASSERT_EQ(rem_euclid(-10, 256), 246);
  ASSERT_EQ(rem_euclid(-266, 256), 246);

  using autd3::driver::pi;
  ASSERT_NEAR(rem_euclid(0.0, 2.0 * pi), 0, 1e-6);
  ASSERT_NEAR(rem_euclid(0.1, 2.0 * pi), 0.1, 1e-6);
  ASSERT_NEAR(rem_euclid(1.9 * pi, 2.0 * pi), 1.9 * pi, 1e-6);
  ASSERT_NEAR(rem_euclid(2.0 * pi, 2.0 * pi), 0.0, 1e-6);
  ASSERT_NEAR(rem_euclid(2.1 * pi, 2.0 * pi), 0.1 * pi, 1e-6);
  ASSERT_NEAR(rem_euclid(-0.1 * pi, 2.0 * pi), 1.9 * pi, 1e-6);
  ASSERT_NEAR(rem_euclid(-2.1 * pi, 2.0 * pi), 1.9 * pi, 1e-6);
}
