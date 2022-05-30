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
#include "autd3/core/geometry/normal_phase_transducer.hpp"
#include "autd3/core/geometry/normal_transducer.hpp"
#include "autd3/gain/primitive.hpp"

using autd3::core::DynamicTransducer;
using autd3::core::NormalPhaseTransducer;
using autd3::core::NormalTransducer;
using autd3::core::propagate;
using autd3::core::Vector3;
using autd3::driver::pi;

TEST(NullTest, LegacyTransducerTest) {
  auto geometry = autd3::core::Geometry();
  geometry.add_device(Vector3::Zero(), Vector3::Zero());

  auto g = autd3::gain::Null();
  g.build(geometry);

  for (const auto& [phase, duty] : g.drives().data) {
    ASSERT_EQ(duty, 0);
    ASSERT_EQ(phase, 0);
  }
}

TEST(NullTest, NormalTransducerTest) {
  auto geometry = autd3::core::Geometry<NormalTransducer>();
  geometry.add_device(Vector3::Zero(), Vector3::Zero());

  auto g = autd3::gain::Null<NormalTransducer>();
  g.build(geometry);

  for (const auto& [duty] : g.drives().duties) {
    ASSERT_EQ(duty, 0);
  }
  for (const auto& [phase] : g.drives().phases) {
    ASSERT_EQ(phase, 0);
  }
}

TEST(NullTest, NormalPhaseTransducerTest) {
  auto geometry = autd3::core::Geometry<NormalPhaseTransducer>();
  geometry.add_device(Vector3::Zero(), Vector3::Zero());

  auto g = autd3::gain::Null<NormalPhaseTransducer>();
  g.build(geometry);

  for (const auto& [phase] : g.drives().phases) {
    ASSERT_EQ(phase, 0);
  }
}

TEST(NullTest, DynamicTransducerTest) {
  auto geometry = autd3::core::Geometry<DynamicTransducer>();
  geometry.add_device(Vector3::Zero(), Vector3::Zero());

  auto gl = autd3::gain::Null<DynamicTransducer>();
  DynamicTransducer::legacy_mode() = true;
  gl.build(geometry);

  for (const auto& [phase, duty] : gl.drives().legacy_drives) {
    ASSERT_EQ(duty, 0);
    ASSERT_EQ(phase, 0);
  }

  auto gn = autd3::gain::Null<DynamicTransducer>();
  DynamicTransducer::legacy_mode() = false;
  gn.build(geometry);
  for (const auto& [duty] : gn.drives().duties) {
    ASSERT_EQ(duty, 0);
  }
  for (const auto& [phase] : gn.drives().phases) {
    ASSERT_EQ(phase, 0);
  }
}

TEST(FocusTest, LegacyTransducerTest) {
  auto geometry = autd3::core::Geometry();
  geometry.add_device(Vector3::Zero(), Vector3::Zero());

  const Vector3 f(10, 20, 30);

  auto g = autd3::gain::Focus(f);
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

  auto g1 = autd3::gain::Focus(f, 0.5);
  g1.build(geometry);
  for (auto& [phase, duty] : g1.drives().data) ASSERT_EQ(duty, static_cast<uint8_t>(std::round(std::asin(0.5) / pi * 510.0)));

  auto g2 = autd3::gain::Focus(f, 0.0);
  g2.build(geometry);
  for (auto& [phase, duty] : g2.drives().data) ASSERT_EQ(duty, 0);

  auto g3 = autd3::gain::Focus(f, 2.0);
  g3.build(geometry);
  for (auto& [phase, duty] : g3.drives().data) ASSERT_EQ(duty, 255);

  auto g4 = autd3::gain::Focus(f, -1.0);
  g4.build(geometry);
  for (auto& [phase, duty] : g4.drives().data) ASSERT_EQ(duty, 0);
}

TEST(FocusTest, NormalTransducerTest) {
  auto geometry = autd3::core::Geometry<NormalTransducer>();
  geometry.add_device(Vector3::Zero(), Vector3::Zero());

  for (auto& dev : geometry)
    for (auto& tr : dev) tr.set_cycle(2500);

  const Vector3 f(10, 20, 30);

  auto g = autd3::gain::Focus<NormalTransducer>(f);
  g.build(geometry);

  const auto expect =
      std::arg(propagate(geometry[0][0].position(), geometry[0][0].z_direction(), 0.0, geometry[0][0].wavenumber(geometry.sound_speed), f) *
               std::exp(std::complex(0.0, 2.0 * pi * static_cast<double>(g.drives().phases[0].phase) / static_cast<double>(2500))));
  for (size_t i = 0; i < g.drives().phases.size(); i++) {
    const auto p =
        std::arg(propagate(geometry[0][i].position(), geometry[0][i].z_direction(), 0.0, geometry[0][i].wavenumber(geometry.sound_speed), f) *
                 std::exp(std::complex(0.0, 2.0 * pi * static_cast<double>(g.drives().phases[i].phase) / static_cast<double>(2500))));
    ASSERT_EQ(g.drives().duties[i].duty, 1250);
    ASSERT_NEAR(p, expect, 2.0 * pi / 2500.0);
  }

  auto g1 = autd3::gain::Focus<NormalTransducer>(f, 0.5);
  g1.build(geometry);
  for (auto& [duty] : g1.drives().duties) ASSERT_EQ(duty, static_cast<uint16_t>(std::round(static_cast<double>(2500) * std::asin(0.5) / pi)));

  auto g2 = autd3::gain::Focus<NormalTransducer>(f, 0.0);
  g2.build(geometry);
  for (auto& [duty] : g2.drives().duties) ASSERT_EQ(duty, 0);

  auto g3 = autd3::gain::Focus<NormalTransducer>(f, 2.0);
  g3.build(geometry);
  for (auto& [duty] : g3.drives().duties) ASSERT_EQ(duty, 1250);

  auto g4 = autd3::gain::Focus<NormalTransducer>(f, -1.0);
  g4.build(geometry);
  for (auto& [duty] : g4.drives().duties) ASSERT_EQ(duty, 0);
}

TEST(FocusTest, NormalPhaseTransducerTest) {
  auto geometry = autd3::core::Geometry<NormalPhaseTransducer>();
  geometry.add_device(Vector3::Zero(), Vector3::Zero());

  for (auto& dev : geometry)
    for (auto& tr : dev) tr.set_cycle(2500);

  const Vector3 f(10, 20, 30);

  auto g = autd3::gain::Focus<NormalPhaseTransducer>(f);
  g.build(geometry);

  const auto expect =
      std::arg(propagate(geometry[0][0].position(), geometry[0][0].z_direction(), 0.0, geometry[0][0].wavenumber(geometry.sound_speed), f) *
               std::exp(std::complex(0.0, 2.0 * pi * static_cast<double>(g.drives().phases[0].phase) / static_cast<double>(2500))));
  for (size_t i = 0; i < g.drives().phases.size(); i++) {
    const auto p =
        std::arg(propagate(geometry[0][i].position(), geometry[0][i].z_direction(), 0.0, geometry[0][i].wavenumber(geometry.sound_speed), f) *
                 std::exp(std::complex(0.0, 2.0 * pi * static_cast<double>(g.drives().phases[i].phase) / static_cast<double>(2500))));
    ASSERT_NEAR(p, expect, 2.0 * pi / 2500.0);
  }
}

TEST(FocusTest, DynamicTransducerTest) {
  auto geometry = autd3::core::Geometry<DynamicTransducer>();
  geometry.add_device(Vector3::Zero(), Vector3::Zero());

  {
    DynamicTransducer::legacy_mode() = true;

    const Vector3 f(10, 20, 30);

    auto g = autd3::gain::Focus<DynamicTransducer>(f);
    g.build(geometry);

    const auto expect =
        std::arg(propagate(geometry[0][0].position(), geometry[0][0].z_direction(), 0.0, geometry[0][0].wavenumber(geometry.sound_speed), f) *
                 std::exp(std::complex(0.0, 2.0 * pi * static_cast<double>(g.drives().legacy_drives[0].phase) / 255.0)));
    for (size_t i = 0; i < g.drives().legacy_drives.size(); i++) {
      const auto p =
          std::arg(propagate(geometry[0][i].position(), geometry[0][i].z_direction(), 0.0, geometry[0][i].wavenumber(geometry.sound_speed), f) *
                   std::exp(std::complex(0.0, 2.0 * pi * static_cast<double>(g.drives().legacy_drives[i].phase) / 255.0)));
      ASSERT_EQ(g.drives().legacy_drives[i].duty, 255);
      ASSERT_NEAR(p, expect, 2.0 * pi / 256.0);
    }

    auto g1 = autd3::gain::Focus<DynamicTransducer>(f, 0.5);
    g1.build(geometry);
    for (auto& [phase, duty] : g1.drives().legacy_drives) ASSERT_EQ(duty, static_cast<uint8_t>(std::round(std::asin(0.5) / pi * 510.0)));

    auto g2 = autd3::gain::Focus<DynamicTransducer>(f, 0.0);
    g2.build(geometry);
    for (auto& [phase, duty] : g2.drives().legacy_drives) ASSERT_EQ(duty, 0);

    auto g3 = autd3::gain::Focus<DynamicTransducer>(f, 2.0);
    g3.build(geometry);
    for (auto& [phase, duty] : g3.drives().legacy_drives) ASSERT_EQ(duty, 255);

    auto g4 = autd3::gain::Focus<DynamicTransducer>(f, -1.0);
    g4.build(geometry);
    for (auto& [phase, duty] : g4.drives().legacy_drives) ASSERT_EQ(duty, 0);
  }

  {
    DynamicTransducer::legacy_mode() = false;

    for (auto& dev : geometry)
      for (auto& tr : dev) tr.set_cycle(2500);

    const Vector3 f(10, 20, 30);

    auto g = autd3::gain::Focus<DynamicTransducer>(f);
    g.build(geometry);

    const auto expect =
        std::arg(propagate(geometry[0][0].position(), geometry[0][0].z_direction(), 0.0, geometry[0][0].wavenumber(geometry.sound_speed), f) *
                 std::exp(std::complex(0.0, 2.0 * pi * static_cast<double>(g.drives().phases[0].phase) / static_cast<double>(2500))));
    for (size_t i = 0; i < g.drives().phases.size(); i++) {
      const auto p =
          std::arg(propagate(geometry[0][i].position(), geometry[0][i].z_direction(), 0.0, geometry[0][i].wavenumber(geometry.sound_speed), f) *
                   std::exp(std::complex(0.0, 2.0 * pi * static_cast<double>(g.drives().phases[i].phase) / static_cast<double>(2500))));
      ASSERT_EQ(g.drives().duties[i].duty, 1250);
      ASSERT_NEAR(p, expect, 2.0 * pi / 2500.0);
    }

    auto g1 = autd3::gain::Focus<DynamicTransducer>(f, 0.5);
    g1.build(geometry);
    for (auto& [duty] : g1.drives().duties) ASSERT_EQ(duty, static_cast<uint16_t>(std::round(static_cast<double>(2500) * std::asin(0.5) / pi)));

    auto g2 = autd3::gain::Focus<DynamicTransducer>(f, 0.0);
    g2.build(geometry);
    for (auto& [duty] : g2.drives().duties) ASSERT_EQ(duty, 0);

    auto g3 = autd3::gain::Focus<DynamicTransducer>(f, 2.0);
    g3.build(geometry);
    for (auto& [duty] : g3.drives().duties) ASSERT_EQ(duty, 1250);

    auto g4 = autd3::gain::Focus<DynamicTransducer>(f, -1.0);
    g4.build(geometry);
    for (auto& [duty] : g4.drives().duties) ASSERT_EQ(duty, 0);
  }
}
