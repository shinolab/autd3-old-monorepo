// File: constraint.cpp
// Project: holo
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 09/10/2023
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
               .add_focus(autd.geometry().center() + autd3::internal::Vector3(30, 0, 150), 0.5)
               .add_focus(autd.geometry().center() + autd3::internal::Vector3(30, 0, 150), 0.5)
               .with_constraint(autd3::gain::holo::AmplitudeConstraint::uniform(0.5));

  ASSERT_TRUE(autd.send(g));

  for (auto& dev : autd.geometry()) {
    auto [duties, phases] = autd.link<autd3::link::Audit>().duties_and_phases(dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 680; }));
    ASSERT_TRUE(std::ranges::any_of(phases, [](auto p) { return p != 0; }));
  }
}

TEST(Gain_Holo, ConstraintNormalize) {
  auto autd = create_controller();

  auto backend = std::make_shared<autd3::gain::holo::NalgebraBackend>();
  auto g = autd3::gain::holo::Naive(std::move(backend))
               .add_focus(autd.geometry().center() + autd3::internal::Vector3(30, 0, 150), 0.5)
               .add_focus(autd.geometry().center() + autd3::internal::Vector3(30, 0, 150), 0.5)
               .with_constraint(autd3::gain::holo::AmplitudeConstraint::normalize());

  ASSERT_TRUE(autd.send(g));

  for (auto& dev : autd.geometry()) {
    auto [duties, phases] = autd.link<autd3::link::Audit>().duties_and_phases(dev.idx(), 0);
    ASSERT_TRUE(std::ranges::any_of(duties, [](auto d) { return d != 0; }));
    ASSERT_TRUE(std::ranges::any_of(phases, [](auto p) { return p != 0; }));
  }
}

TEST(Gain_Holo, ConstraintClamp) {
  auto autd = create_controller();

  auto backend = std::make_shared<autd3::gain::holo::NalgebraBackend>();
  auto g = autd3::gain::holo::Naive(std::move(backend))
               .add_focus(autd.geometry().center() + autd3::internal::Vector3(30, 0, 150), 0.5)
               .add_focus(autd.geometry().center() + autd3::internal::Vector3(30, 0, 150), 0.5)
               .with_constraint(autd3::gain::holo::AmplitudeConstraint::clamp(0.4, 0.5));

  ASSERT_TRUE(autd.send(g));

  for (auto& dev : autd.geometry()) {
    auto [duties, phases] = autd.link<autd3::link::Audit>().duties_and_phases(dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return 536 <= d && d <= 680; }));
    ASSERT_TRUE(std::ranges::any_of(phases, [](auto p) { return p != 0; }));
  }
}

TEST(Gain_Holo, ConstraintDontCare) {
  auto autd = create_controller();

  auto backend = std::make_shared<autd3::gain::holo::NalgebraBackend>();
  auto g = autd3::gain::holo::Naive(std::move(backend))
               .add_focus(autd.geometry().center() + autd3::internal::Vector3(30, 0, 150), 0.5)
               .add_focus(autd.geometry().center() + autd3::internal::Vector3(30, 0, 150), 0.5)
               .with_constraint(autd3::gain::holo::AmplitudeConstraint::dont_care());

  ASSERT_TRUE(autd.send(g));

  for (auto& dev : autd.geometry()) {
    auto [duties, phases] = autd.link<autd3::link::Audit>().duties_and_phases(dev.idx(), 0);
    ASSERT_TRUE(std::ranges::any_of(duties, [](auto d) { return d != 0; }));
    ASSERT_TRUE(std::ranges::any_of(phases, [](auto p) { return p != 0; }));
  }
}
