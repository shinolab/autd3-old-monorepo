// File: lm.cpp
// Project: holo
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 27/11/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include <autd3/internal/controller.hpp>
#include <autd3/link/audit.hpp>

#include "autd3/gain/holo.hpp"

TEST(Gain_Holo, LM) {
  auto autd = autd3::internal::ControllerBuilder()
                  .add_device(autd3::internal::AUTD3(autd3::internal::Vector3::Zero()))
                  .open_with_async(autd3::link::Audit::builder())
                  .get();

  auto backend = std::make_shared<autd3::gain::holo::NalgebraBackend>();
  std::vector<double> p{-30};
  auto g = autd3::gain::holo::LM(std::move(backend))
               .add_focus(autd.geometry().center() + autd3::internal::Vector3(30, 0, 150), 5e3 * autd3::gain::holo::Pascal)
               .add_foci_from_iter(p | std::ranges::views::transform([&](auto x) {
                                     autd3::internal::Vector3 p = autd.geometry().center() + autd3::internal::Vector3(x, 0, 150);
                                     return std::make_pair(p, 5e3 * autd3::gain::holo::Pascal);
                                   }))
               .with_eps1(1e-3)
               .with_eps2(1e-3)
               .with_tau(1e-3)
               .with_k_max(5)
               .with_initial(std::vector{1.0})
               .with_constraint(autd3::gain::holo::AmplitudeConstraint::uniform(0x80));

  ASSERT_TRUE(autd.send_async(g).get());

  for (auto& dev : autd.geometry()) {
    auto [intensities, phases] = autd.link().intensities_and_phases(dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0x80; }));
    ASSERT_TRUE(std::ranges::any_of(phases, [](auto p) { return p != 0; }));
  }
}
