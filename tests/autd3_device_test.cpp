// File: autd3_device_test.cpp
// Project: tests
// Created Date: 25/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 25/11/2022
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

#include "autd3/autd3_device.hpp"
#include "test_utils.hpp"

using Vector3 = autd3::core::Vector3;
using Quaternion = autd3::core::Quaternion;

TEST(HARDTest, is_missing_transducer) {
  ASSERT_TRUE(autd3::AUTD3::is_missing_transducer(1, 1));
  ASSERT_TRUE(autd3::AUTD3::is_missing_transducer(2, 1));
  ASSERT_TRUE(autd3::AUTD3::is_missing_transducer(16, 1));

  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(0, 0));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(1, 0));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(2, 0));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(3, 0));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(4, 0));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(5, 0));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(6, 0));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(7, 0));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(8, 0));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(9, 0));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(10, 0));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(11, 0));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(12, 0));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(13, 0));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(14, 0));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(15, 0));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(16, 0));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(17, 0));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(0, 1));
  ASSERT_TRUE(autd3::AUTD3::is_missing_transducer(1, 1));
  ASSERT_TRUE(autd3::AUTD3::is_missing_transducer(2, 1));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(3, 1));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(4, 1));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(5, 1));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(6, 1));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(7, 1));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(8, 1));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(9, 1));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(10, 1));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(11, 1));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(12, 1));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(13, 1));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(14, 1));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(15, 1));
  ASSERT_TRUE(autd3::AUTD3::is_missing_transducer(16, 1));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(17, 1));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(0, 2));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(1, 2));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(2, 2));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(3, 2));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(4, 2));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(5, 2));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(6, 2));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(7, 2));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(8, 2));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(9, 2));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(10, 2));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(11, 2));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(12, 2));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(13, 2));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(14, 2));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(15, 2));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(16, 2));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(17, 2));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(0, 3));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(1, 3));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(2, 3));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(3, 3));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(4, 3));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(5, 3));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(6, 3));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(7, 3));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(8, 3));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(9, 3));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(10, 3));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(11, 3));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(12, 3));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(13, 3));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(14, 3));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(15, 3));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(16, 3));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(17, 3));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(0, 4));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(1, 4));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(2, 4));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(3, 4));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(4, 4));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(5, 4));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(6, 4));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(7, 4));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(8, 4));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(9, 4));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(10, 4));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(11, 4));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(12, 4));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(13, 4));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(14, 4));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(15, 4));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(16, 4));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(17, 4));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(0, 5));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(1, 5));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(2, 5));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(3, 5));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(4, 5));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(5, 5));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(6, 5));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(7, 5));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(8, 5));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(9, 5));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(10, 5));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(11, 5));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(12, 5));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(13, 5));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(14, 5));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(15, 5));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(16, 5));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(17, 5));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(0, 6));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(1, 6));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(2, 6));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(3, 6));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(4, 6));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(5, 6));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(6, 6));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(7, 6));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(8, 6));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(9, 6));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(10, 6));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(11, 6));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(12, 6));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(13, 6));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(14, 6));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(15, 6));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(16, 6));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(17, 6));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(0, 7));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(1, 7));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(2, 7));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(3, 7));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(4, 7));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(5, 7));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(6, 7));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(7, 7));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(8, 7));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(9, 7));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(10, 7));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(11, 7));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(12, 7));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(13, 7));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(14, 7));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(15, 7));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(16, 7));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(17, 7));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(0, 8));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(1, 8));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(2, 8));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(3, 8));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(4, 8));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(5, 8));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(6, 8));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(7, 8));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(8, 8));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(9, 8));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(10, 8));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(11, 8));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(12, 8));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(13, 8));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(14, 8));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(15, 8));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(16, 8));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(17, 8));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(0, 9));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(1, 9));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(2, 9));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(3, 9));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(4, 9));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(5, 9));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(6, 9));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(7, 9));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(8, 9));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(9, 9));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(10, 9));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(11, 9));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(12, 9));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(13, 9));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(14, 9));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(15, 9));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(16, 9));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(17, 9));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(0, 10));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(1, 10));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(2, 10));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(3, 10));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(4, 10));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(5, 10));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(6, 10));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(7, 10));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(8, 10));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(9, 10));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(10, 10));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(11, 10));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(12, 10));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(13, 10));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(14, 10));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(15, 10));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(16, 10));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(17, 10));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(0, 11));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(1, 11));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(2, 11));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(3, 11));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(4, 11));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(5, 11));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(6, 11));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(7, 11));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(8, 11));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(9, 11));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(10, 11));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(11, 11));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(12, 11));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(13, 11));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(14, 11));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(15, 11));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(16, 11));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(17, 11));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(0, 12));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(1, 12));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(2, 12));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(3, 12));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(4, 12));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(5, 12));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(6, 12));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(7, 12));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(8, 12));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(9, 12));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(10, 12));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(11, 12));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(12, 12));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(13, 12));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(14, 12));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(15, 12));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(16, 12));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(17, 12));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(1, 13));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(2, 13));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(3, 13));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(4, 13));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(5, 13));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(6, 13));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(7, 13));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(8, 13));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(9, 13));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(10, 13));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(11, 13));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(12, 13));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(13, 13));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(14, 13));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(15, 13));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(16, 13));
  ASSERT_FALSE(autd3::AUTD3::is_missing_transducer(17, 13));
}

TEST(GeometryTest, num_transducers) {
  autd3::core::Geometry geometry;
  ASSERT_EQ(geometry.num_transducers(), 0);
  geometry.add_device(autd3::AUTD3(Vector3::Zero(), Vector3::Zero()));
  ASSERT_EQ(geometry.num_transducers(), 249);
  geometry.add_device(autd3::AUTD3(Vector3::Zero(), Vector3::Zero()));
  ASSERT_EQ(geometry.num_transducers(), 249 * 2);
}

TEST(GeometryTest, center) {
  autd3::core::Geometry geometry;
  Vector3 expect = Vector3::Zero();
  ASSERT_NEAR_VECTOR3(geometry.center(), expect, 1e-3);

  geometry.add_device(autd3::AUTD3(Vector3(10, 20, 30), Vector3::Zero()));
  for (size_t i = 0; i < 18; i++) {
    for (size_t j = 0; j < 14; j++) {
      if (autd3::AUTD3::is_missing_transducer(i, j)) continue;
      expect += 10.16 * Vector3(static_cast<double>(i), static_cast<double>(j), 0.0) + Vector3(10, 20, 30);
    }
  }
  expect /= 249;
  ASSERT_NEAR_VECTOR3(geometry.center(), expect, 1e-3);
}

TEST(GeometryTest, add_device) {
  autd3::core::Geometry geometry;

  geometry.add_device(autd3::AUTD3(Vector3(10, 20, 30), Vector3::Zero()));
  geometry.add_device(autd3::AUTD3(Vector3(0, 0, 0), Vector3(autd3::driver::pi, autd3::driver::pi, 0)));
  geometry.add_device(autd3::AUTD3(Vector3(0, 0, 0), Vector3(0, autd3::driver::pi, 0)));
  geometry.add_device(autd3::AUTD3(Vector3(0, 0, 0), Vector3(autd3::driver::pi, 0, 0)));
  geometry.add_device(autd3::AUTD3(Vector3(40, 60, 50), Vector3(0, 0, autd3::driver::pi / 2)));

  const Vector3 origin(0, 0, 0);
  const Vector3 right_bottom(10.16 * 17, 0, 0);
  const Vector3 left_top(0, 10.16 * 13, 0);

  ASSERT_NEAR_VECTOR3(geometry[0].position(), (Vector3(10, 20, 30) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[17].position(), (Vector3(10, 20, 30) + right_bottom), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[231].position(), (Vector3(10, 20, 30) + left_top), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[248].position(), (Vector3(10, 20, 30) + right_bottom + left_top), 1e-3);

  ASSERT_NEAR_VECTOR3(geometry[1 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 0].position(), (Vector3(0, 0, 0) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[1 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 17].position(), (Vector3(0, 0, 0) + right_bottom), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[1 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 231].position(), (Vector3(0, 0, 0) - left_top), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[1 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 248].position(), (Vector3(0, 0, 0) + right_bottom - left_top), 1e-3);

  ASSERT_NEAR_VECTOR3(geometry[2 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 0].position(), (Vector3(0, 0, 0) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[2 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 17].position(), (Vector3(0, 0, 0) - right_bottom), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[2 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 231].position(), (Vector3(0, 0, 0) + left_top), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[2 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 248].position(), (Vector3(0, 0, 0) - right_bottom + left_top), 1e-3);

  ASSERT_NEAR_VECTOR3(geometry[3 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 0].position(), (Vector3(0, 0, 0) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[3 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 17].position(), (Vector3(0, 0, 0) - right_bottom), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[3 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 231].position(), (Vector3(0, 0, 0) - left_top), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[3 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 248].position(), (Vector3(0, 0, 0) - right_bottom - left_top), 1e-3);

  ASSERT_NEAR_VECTOR3(geometry[4 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 0].position(), (Vector3(40, 60, 50) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[4 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 17].position(), (Vector3(40, 60, 50) + Vector3(0, 10.16 * 17, 0)), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[4 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 231].position(), (Vector3(40, 60, 50) - Vector3(10.16 * 13, 0, 0)), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[4 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 248].position(),
                      (Vector3(40, 60, 50) + Vector3(0, 10.16 * 17, 0) - Vector3(10.16 * 13, 0, 0)), 1e-3);
}

TEST(GeometryTest, add_device_quaternion) {
  autd3::core::Geometry geometry;

  geometry.add_device(autd3::AUTD3(Vector3(10, 20, 30), Quaternion::Identity()));
  geometry.add_device(autd3::AUTD3(Vector3(0, 0, 0), Quaternion::Identity() * Eigen::AngleAxis(autd3::driver::pi, Vector3::UnitX())));
  geometry.add_device(autd3::AUTD3(Vector3(0, 0, 0), Quaternion::Identity() * Eigen::AngleAxis(autd3::driver::pi, Vector3::UnitY())));
  geometry.add_device(autd3::AUTD3(Vector3(0, 0, 0), Quaternion::Identity() * Eigen::AngleAxis(autd3::driver::pi, Vector3::UnitZ())));
  geometry.add_device(autd3::AUTD3(Vector3(40, 60, 50), Quaternion::Identity() * Eigen::AngleAxis(autd3::driver::pi / 2, Vector3::UnitZ())));

  const Vector3 origin(0, 0, 0);
  const Vector3 right_bottom(10.16 * 17, 0, 0);
  const Vector3 left_top(0, 10.16 * 13, 0);

  ASSERT_NEAR_VECTOR3(geometry[0 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 0].position(), (Vector3(10, 20, 30) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[0 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 17].position(), (Vector3(10, 20, 30) + right_bottom), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[0 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 231].position(), (Vector3(10, 20, 30) + left_top), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[0 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 248].position(), (Vector3(10, 20, 30) + right_bottom + left_top), 1e-3);

  ASSERT_NEAR_VECTOR3(geometry[1 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 0].position(), (Vector3(0, 0, 0) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[1 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 17].position(), (Vector3(0, 0, 0) + right_bottom), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[1 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 231].position(), (Vector3(0, 0, 0) - left_top), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[1 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 248].position(), (Vector3(0, 0, 0) + right_bottom - left_top), 1e-3);

  ASSERT_NEAR_VECTOR3(geometry[2 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 0].position(), (Vector3(0, 0, 0) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[2 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 17].position(), (Vector3(0, 0, 0) - right_bottom), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[2 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 231].position(), (Vector3(0, 0, 0) + left_top), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[2 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 248].position(), (Vector3(0, 0, 0) - right_bottom + left_top), 1e-3);

  ASSERT_NEAR_VECTOR3(geometry[3 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 0].position(), (Vector3(0, 0, 0) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[3 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 17].position(), (Vector3(0, 0, 0) - right_bottom), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[3 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 231].position(), (Vector3(0, 0, 0) - left_top), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[3 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 248].position(), (Vector3(0, 0, 0) - right_bottom - left_top), 1e-3);

  ASSERT_NEAR_VECTOR3(geometry[4 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 0].position(), (Vector3(40, 60, 50) + origin), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[4 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 17].position(), (Vector3(40, 60, 50) + Vector3(0, 10.16 * 17, 0)), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[4 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 231].position(), (Vector3(40, 60, 50) - Vector3(10.16 * 13, 0, 0)), 1e-3);
  ASSERT_NEAR_VECTOR3(geometry[4 * autd3::AUTD3::NUM_TRANS_IN_UNIT + 248].position(),
                      (Vector3(40, 60, 50) + Vector3(0, 10.16 * 17, 0) - Vector3(10.16 * 13, 0, 0)), 1e-3);
}
