// File: primitive_test.cpp
// Project: primitive
// Created Date: 24/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 08/09/2022
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

#include "autd3/core/geometry.hpp"
#include "autd3/gain/primitive.hpp"

using autd3::core::propagate;
using autd3::core::Vector3;
using autd3::driver::pi;

TEST(NullTest, Test) {
  auto geometry = autd3::core::Geometry();
  geometry.add_device(Vector3::Zero(), Vector3::Zero());

  auto g = autd3::gain::Null();
  g.build(geometry);

  for (const auto& [phase, duty, _] : g.drives()) {
    ASSERT_EQ(duty, 0.0);
  }
}

TEST(FocusTest, Test) {
  auto geometry = autd3::core::Geometry();
  geometry.add_device(Vector3::Zero(), Vector3::Zero());

  const Vector3 f(10, 20, 30);

  auto g = autd3::gain::Focus(f);
  g.build(geometry);

  const auto expect =
      std::arg(propagate(geometry[0][0].position(), geometry[0][0].z_direction(), 0.0, geometry[0][0].wavenumber(geometry.sound_speed), f) *
               std::exp(std::complex(0.0, g.drives()[0].phase)));
  for (size_t i = 0; i < g.drives().size(); i++) {
    const auto p =
        std::arg(propagate(geometry[0][i].position(), geometry[0][i].z_direction(), 0.0, geometry[0][i].wavenumber(geometry.sound_speed), f) *
                 std::exp(std::complex(0.0, g.drives()[i].phase)));
    ASSERT_EQ(g.drives()[i].amp, 1.0);
    ASSERT_NEAR(p, expect, 2.0 * pi / 256.0);
  }

  auto g1 = autd3::gain::Focus(f, 0.5);
  g1.build(geometry);
  for (auto& [phase, amp, _] : g1.drives()) ASSERT_EQ(amp, 0.5);

  auto g2 = autd3::gain::Focus(f, 0.0);
  g2.build(geometry);
  for (auto& [phase, amp, _] : g2.drives()) ASSERT_EQ(amp, 0.0);

  auto g3 = autd3::gain::Focus(f, 2.0);
  g3.build(geometry);
  for (auto& [phase, amp, _] : g3.drives()) ASSERT_EQ(amp, 2.0);

  auto g4 = autd3::gain::Focus(f, -1.0);
  g4.build(geometry);
  for (auto& [phase, amp, _] : g4.drives()) ASSERT_EQ(amp, -1.0);
}
