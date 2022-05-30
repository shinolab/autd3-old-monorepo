// File: primitive_test.cpp
// Project: primitive
// Created Date: 24/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 30/05/2022
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

#include "autd3/core/geometry/dynamic_transducer.hpp"
#include "autd3/core/geometry/geometry.hpp"
#include "autd3/core/geometry/legacy_transducer.hpp"
#include "autd3/core/geometry/normal_transducer.hpp"
#include "autd3/gain/primitive.hpp"

using autd3::core::DynamicTransducer;
using autd3::core::LegacyTransducer;
using autd3::core::NormalTransducer;
using autd3::core::propagate;
using autd3::core::Vector3;
using autd3::driver::pi;

TEST(NullTest, LegacyTransducerTest) {
  auto geometry = autd3::core::Geometry();
  geometry.add_device(Vector3::Zero(), Vector3::Zero());

  auto g = autd3::gain::Null<LegacyTransducer>();
  g.build(geometry);

  for (const auto& d : g.drives().data) {
    ASSERT_EQ(d.duty, 0);
    ASSERT_EQ(d.phase, 0);
  }
}

TEST(NullTest, NormalTransducerTest) {
  auto geometry = autd3::core::Geometry<NormalTransducer>();
  geometry.add_device(Vector3::Zero(), Vector3::Zero());

  auto g = autd3::gain::Null<NormalTransducer>();
  g.build(geometry);

  for (const auto& d : g.drives().duties) {
    ASSERT_EQ(d.duty, 0);
  }
  for (const auto& d : g.drives().phases) {
    ASSERT_EQ(d.phase, 0);
  }
}

TEST(NullTest, DynamicTransducerTest) {
  auto geometry = autd3::core::Geometry<DynamicTransducer>();
  geometry.add_device(Vector3::Zero(), Vector3::Zero());

  auto gl = autd3::gain::Null<DynamicTransducer>();
  DynamicTransducer::legacy_mode() = true;
  gl.build(geometry);

  for (const auto& d : gl.drives().legacy_drives) {
    ASSERT_EQ(d.duty, 0);
    ASSERT_EQ(d.phase, 0);
  }

  auto gn = autd3::gain::Null<DynamicTransducer>();
  DynamicTransducer::legacy_mode() = false;
  gn.build(geometry);
  for (const auto& d : gn.drives().duties) {
    ASSERT_EQ(d.duty, 0);
  }
  for (const auto& d : gn.drives().phases) {
    ASSERT_EQ(d.phase, 0);
  }
}

TEST(FocusTest, LegacyTransducerTest) {
  auto geometry = autd3::core::Geometry();
  geometry.add_device(Vector3::Zero(), Vector3::Zero());

  const Vector3 f(10, 20, 30);

  auto g = autd3::gain::Focus<LegacyTransducer>(f);
  g.build(geometry);

  const auto expect =
      std::arg(propagate(geometry[0][0].position(), geometry[0][0].z_direction(), 0.0, geometry[0][0].wavenumber(geometry.sound_speed), f) *
               std::exp(std::complex(0.0, 2.0 * pi * static_cast<double>(g.drives().data[0].phase) / 255.0)));
  for (size_t i = 0; i < g.drives().data.size(); i++) {
    const auto p =
        std::arg(propagate(geometry[0][i].position(), geometry[0][i].z_direction(), 0.0, geometry[0][i].wavenumber(geometry.sound_speed), f) *
                 std::exp(std::complex(0.0, 2.0 * pi * static_cast<double>(g.drives().data[i].phase) / 255.0)));
    ASSERT_EQ(g.drives().data[i].duty, 255);
    ASSERT_NEAR(p, expect, 2.0 * pi / 256.0);
  }

  auto g1 = autd3::gain::Focus<LegacyTransducer>(f, 0.5);
  g1.build(geometry);
  for (size_t i = 0; i < g1.drives().data.size(); i++)
    ASSERT_EQ(g1.drives().data[i].duty, static_cast<uint8_t>(std::round(std::asin(0.5) / pi * 510.0)));

  auto g2 = autd3::gain::Focus<LegacyTransducer>(f, 0.0);
  g2.build(geometry);
  for (size_t i = 0; i < g2.drives().data.size(); i++) ASSERT_EQ(g2.drives().data[i].duty, 0);

  auto g3 = autd3::gain::Focus<LegacyTransducer>(f, 2.0);
  g3.build(geometry);
  for (size_t i = 0; i < g3.drives().data.size(); i++) ASSERT_EQ(g3.drives().data[i].duty, 255);

  auto g4 = autd3::gain::Focus<LegacyTransducer>(f, -1.0);
  g4.build(geometry);
  for (size_t i = 0; i < g4.drives().data.size(); i++) ASSERT_EQ(g4.drives().data[i].duty, 0);
}
