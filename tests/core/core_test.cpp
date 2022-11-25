// File: core_test.cpp
// Project: core
// Created Date: 24/05/2022
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

#include <autd3/core/acoustics.hpp>
#include <autd3/core/geometry.hpp>
#include <random>

#include "test_utils.hpp"

using autd3::core::Quaternion;
using autd3::core::Vector3;

TEST(TransducerTest, Transducer) {
  const auto rot =
      Eigen::AngleAxis(autd3::driver::pi / 2.0, Vector3::UnitZ()) * Eigen::AngleAxis(0.0, Vector3::UnitY()) * Eigen::AngleAxis(0.0, Vector3::UnitX());

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

TEST(UtilitiesTest, Directivity) {
  constexpr double expects[91] = {
      1,        1,        1,        1,        1,        1,        1,        1,        1,        1,        1,        1,        1,
      1,        1,        1,        1,        1,        1,        1,        1,        0.994632, 0.987783, 0.979551, 0.970031, 0.95932,
      0.947513, 0.934707, 0.920997, 0.906479, 0.891251, 0.875394, 0.85894,  0.841907, 0.824312, 0.806173, 0.787508, 0.768335, 0.748672,
      0.728536, 0.707946, 0.686939, 0.665635, 0.644172, 0.622691, 0.601329, 0.580226, 0.559521, 0.539353, 0.519863, 0.501187, 0.483432,
      0.466559, 0.450499, 0.435179, 0.420529, 0.406476, 0.392949, 0.379878, 0.367189, 0.354813, 0.342697, 0.330862, 0.319348, 0.308198,
      0.297451, 0.287148, 0.277329, 0.268036, 0.259309, 0.251189, 0.243703, 0.236828, 0.230529, 0.22477,  0.219514, 0.214725, 0.210368,
      0.206407, 0.202805, 0.199526, 0.196537, 0.193806, 0.191306, 0.189007, 0.18688,  0.184898, 0.183031, 0.18125,  0.179526, 0.177831};

  for (size_t i = 0; i < 91; i++) ASSERT_NEAR(autd3::core::Directivity::t4010a1(static_cast<double>(i)), expects[i], 1e-3);
}

TEST(UtilitiesTest, propagate) {
  constexpr auto wavenumber = 2.0 * autd3::driver::pi / 2.0;  // lambda = 2.0

  ASSERT_NEAR_COMPLEX(autd3::core::propagate(Vector3::Zero(), Vector3::UnitZ(), 0.0, wavenumber, Vector3(0.0, 0.0, 1.0)), std::complex(-1.0, 0.0),
                      1e-3);

  ASSERT_NEAR_COMPLEX(autd3::core::propagate(Vector3::Zero(), Vector3::UnitZ(), 0.0, wavenumber, Vector3(0.0, 0.0, 2.0)), std::complex(0.5, 0.0),
                      1e-3);

  ASSERT_NEAR_COMPLEX(autd3::core::propagate(Vector3::Zero(), Vector3::UnitZ(), 0.0, wavenumber, Vector3(1.0, 0.0, 0.0)),
                      std::complex(-0.177831, 0.0), 1e-3);

  ASSERT_NEAR_COMPLEX(autd3::core::propagate(Vector3::Zero(), Vector3::UnitZ(), 0.0, wavenumber, Vector3(0.0, 1.0, 0.0)),
                      std::complex(-0.177831, 0.0), 1e-3);

  ASSERT_NEAR_COMPLEX(autd3::core::propagate(Vector3::Zero(), Vector3::UnitX(), 0.0, wavenumber, Vector3(1.0, 0.0, 0.0)), std::complex(-1.0, 0.0),
                      1e-3);
}
