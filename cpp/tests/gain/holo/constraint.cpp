// File: constraint.cpp
// Project: holo
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 28/11/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include "autd3/gain/holo.hpp"
#include "utils.hpp"

TEST(Gain_Holo, ConstraintUniform) {
  auto autd = create_controller();

  auto backend = std::make_shared<autd3::gain::holo::NalgebraBackend>();
  auto g = autd3::gain::holo::Naive(std::move(backend))
               .add_focus(autd.geometry().center() + autd3::internal::Vector3(30, 0, 150), 5e3 * autd3::gain::holo::Pascal)
               .add_focus(autd.geometry().center() + autd3::internal::Vector3(30, 0, 150), 5e3 * autd3::gain::holo::Pascal)
               .with_constraint(autd3::gain::holo::EmissionConstraint::uniform(0x80));

  ASSERT_TRUE(autd.send_async(g).get());

  for (auto& dev : autd.geometry()) {
    auto [intensities, phases] = autd.link().intensities_and_phases(dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0x80; }));
    ASSERT_TRUE(std::ranges::any_of(phases, [](auto p) { return p != 0; }));
  }
}

TEST(Gain_Holo, ConstraintNormalize) {
  auto autd = create_controller();

  auto backend = std::make_shared<autd3::gain::holo::NalgebraBackend>();
  auto g = autd3::gain::holo::Naive(std::move(backend))
               .add_focus(autd.geometry().center() + autd3::internal::Vector3(30, 0, 150), 5e3 * autd3::gain::holo::Pascal)
               .add_focus(autd.geometry().center() + autd3::internal::Vector3(30, 0, 150), 5e3 * autd3::gain::holo::Pascal)
               .with_constraint(autd3::gain::holo::EmissionConstraint::normalize());

  ASSERT_TRUE(autd.send_async(g).get());

  for (auto& dev : autd.geometry()) {
    auto [intensities, phases] = autd.link().intensities_and_phases(dev.idx(), 0);
    ASSERT_TRUE(std::ranges::any_of(intensities, [](auto d) { return d != 0; }));
    ASSERT_TRUE(std::ranges::any_of(phases, [](auto p) { return p != 0; }));
  }
}

TEST(Gain_Holo, ConstraintClamp) {
  auto autd = create_controller();

  auto backend = std::make_shared<autd3::gain::holo::NalgebraBackend>();
  auto g = autd3::gain::holo::Naive(std::move(backend))
               .add_focus(autd.geometry().center() + autd3::internal::Vector3(30, 0, 150), 5e3 * autd3::gain::holo::Pascal)
               .add_focus(autd.geometry().center() + autd3::internal::Vector3(30, 0, 150), 5e3 * autd3::gain::holo::Pascal)
               .with_constraint(autd3::gain::holo::EmissionConstraint::clamp(67, 85));

  ASSERT_TRUE(autd.send_async(g).get());

  for (auto& dev : autd.geometry()) {
    auto [intensities, phases] = autd.link().intensities_and_phases(dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return 67 <= d && d <= 85; }));
    ASSERT_TRUE(std::ranges::any_of(phases, [](auto p) { return p != 0; }));
  }
}

TEST(Gain_Holo, ConstraintDontCare) {
  auto autd = create_controller();

  auto backend = std::make_shared<autd3::gain::holo::NalgebraBackend>();
  auto g = autd3::gain::holo::Naive(std::move(backend))
               .add_focus(autd.geometry().center() + autd3::internal::Vector3(30, 0, 150), 5e3 * autd3::gain::holo::Pascal)
               .add_focus(autd.geometry().center() + autd3::internal::Vector3(30, 0, 150), 5e3 * autd3::gain::holo::Pascal)
               .with_constraint(autd3::gain::holo::EmissionConstraint::dont_care());

  ASSERT_TRUE(autd.send_async(g).get());

  for (auto& dev : autd.geometry()) {
    auto [intensities, phases] = autd.link().intensities_and_phases(dev.idx(), 0);
    ASSERT_TRUE(std::ranges::any_of(intensities, [](auto d) { return d != 0; }));
    ASSERT_TRUE(std::ranges::any_of(phases, [](auto p) { return p != 0; }));
  }
}
