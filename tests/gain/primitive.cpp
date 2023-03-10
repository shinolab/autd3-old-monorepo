// File: primitive.cpp
// Project: primitive
// Created Date: 24/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 02/03/2023
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

#include "autd3/autd3_device.hpp"
#include "autd3/core/geometry.hpp"
#include "autd3/gain/primitive.hpp"

using complex = std::complex<autd3::driver::autd3_float_t>;
using autd3::core::propagate;
using autd3::core::Vector3;
using autd3::driver::pi;

TEST(Gain, Null) {
  const auto geometry = autd3::core::Geometry::Builder().add_device(autd3::AUTD3(Vector3::Zero(), Vector3::Zero())).build();

  auto g = autd3::gain::Null();
  for (const auto& [phase, duty] : g.calc(geometry)) {
    ASSERT_EQ(duty, 0.0);
  }
}

TEST(Gain, Focus) {
  const auto geometry = autd3::core::Geometry::Builder().add_device(autd3::AUTD3(Vector3::Zero(), Vector3::Zero())).build();

  const Vector3 f(10, 20, 30);

  auto g = autd3::gain::Focus(f);
  const auto drives = g.calc(geometry);

  const auto expect = std::arg(propagate(geometry[0].position(), geometry[0].z_direction(), 0.0, geometry[0].wavenumber(geometry.sound_speed), f) *
                               std::exp(complex(0.0, drives[0].phase)));
  for (size_t i = 0; i < drives.size(); i++) {
    const auto p = std::arg(propagate(geometry[i].position(), geometry[i].z_direction(), 0.0, geometry[i].wavenumber(geometry.sound_speed), f) *
                            std::exp(complex(0.0, drives[i].phase)));
    ASSERT_EQ(drives[i].amp, 1.0);
    ASSERT_NEAR(p, expect, 2.0 * pi / 256.0);
  }

  auto g1 = autd3::gain::Focus(f, 0.5);
  for (auto& [phase, amp] : g1.calc(geometry)) ASSERT_EQ(amp, 0.5);

  auto g2 = autd3::gain::Focus(f, 0.0);
  for (auto& [phase, amp] : g2.calc(geometry)) ASSERT_EQ(amp, 0.0);

  auto g3 = autd3::gain::Focus(f, 2.0);
  for (auto& [phase, amp] : g3.calc(geometry)) ASSERT_EQ(amp, 2.0);

  auto g4 = autd3::gain::Focus(f, -1.0);
  for (auto& [phase, amp] : g4.calc(geometry)) ASSERT_EQ(amp, -1.0);
}
