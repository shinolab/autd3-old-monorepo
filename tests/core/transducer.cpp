// File: transducer.cpp
// Project: core
// Created Date: 02/12/2022
// Author: Shun Suzuki
// -----
// Last Modified: 21/12/2022
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

#include <autd3/core/transducer.hpp>

#include "test_utils.hpp"

using autd3::core::Quaternion;
using autd3::core::Vector3;
using autd3::driver::autd3_float_t;

TEST(CoreTransducer, Transducer) {
  const auto rot = Eigen::AngleAxis(autd3::driver::pi / autd3_float_t{2}, Vector3::UnitZ()) * Eigen::AngleAxis(autd3_float_t{0}, Vector3::UnitY()) *
                   Eigen::AngleAxis(autd3_float_t{0}, Vector3::UnitX());

  autd3::core::Transducer tr(1, Vector3(10, 20, 30), rot);

  ASSERT_NEAR_VECTOR3(tr.position(), Vector3(10, 20, 30), 1e-3);
  ASSERT_NEAR_VECTOR3(tr.x_direction(), Vector3(0, 1, 0), 1e-3);
  ASSERT_NEAR_VECTOR3(tr.y_direction(), Vector3(-1, 0, 0), 1e-3);
  ASSERT_NEAR_VECTOR3(tr.z_direction(), Vector3(0, 0, 1), 1e-3);

  ASSERT_EQ(tr.id(), 1);

  tr.set_cycle(3000);
  ASSERT_EQ(tr.cycle(), 3000);
  tr.set_frequency(70e3);
  ASSERT_NEAR(tr.frequency(), 70e3, 15.0);

  ASSERT_NEAR(tr.wavelength(), 4.857142857142857142857142857L, 1e-3);
  ASSERT_NEAR(tr.wavenumber(), 1.293596975007561871293279075L, 1e-3);
}
