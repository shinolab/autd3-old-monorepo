// File: gs.cpp
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

#include <autd3/internal/controller.hpp>
#include <autd3/link/audit.hpp>

#include "autd3/gain/holo.hpp"

TEST(Gain_Holo, GS) {
  auto autd = autd3::internal::Controller::builder()
                  .add_device(autd3::internal::AUTD3(autd3::internal::Vector3::Zero(), autd3::internal::Vector3::Zero()))
                  .open_with(autd3::link::Audit::builder());

  auto backend = std::make_shared<autd3::gain::holo::NalgebraBackend>();
  std::vector<double> p{-30};
  auto g = autd3::gain::holo::GS(std::move(backend))
               .add_focus(autd.geometry().center() + autd3::internal::Vector3(30, 0, 150), 0.5)
               .add_foci_from_iter(p | std::ranges::views::transform([&](auto x) {
                                     autd3::internal::Vector3 p = autd.geometry().center() + autd3::internal::Vector3(x, 0, 150);
                                     return std::make_pair(p, 0.5);
                                   }))
               .with_repeat(100)
               .with_constraint(autd3::gain::holo::AmplitudeConstraint::uniform(0.5));

  ASSERT_TRUE(autd.send(g));

  for (auto& dev : autd.geometry()) {
    auto [duties, phases] = autd.link<autd3::link::Audit>().duties_and_phases(dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 680; }));
    ASSERT_TRUE(std::ranges::any_of(phases, [](auto p) { return p != 0; }));
  }
}

#ifdef ENABLE_BACKEND_CUDA

#include "autd3/gain/holo/backend_cuda.hpp"

TEST(Gain_Holo, GSWithCUDA) {
  auto autd = autd3::internal::Controller::builder()
                  .add_device(autd3::internal::AUTD3(autd3::internal::Vector3::Zero(), autd3::internal::Vector3::Zero()))
                  .open_with(autd3::link::Audit::builder());

  auto backend = std::make_shared<autd3::gain::holo::CUDABackend>();
  std::vector<double> p{-30};
  auto g = autd3::gain::holo::GS(std::move(backend))
               .add_focus(autd.geometry().center() + autd3::internal::Vector3(30, 0, 150), 0.5)
               .add_foci_from_iter(p | std::ranges::views::transform([&](auto x) {
                                     autd3::internal::Vector3 p = autd.geometry().center() + autd3::internal::Vector3(x, 0, 150);
                                     return std::make_pair(p, 0.5);
                                   }))
               .with_repeat(100)
               .with_constraint(autd3::gain::holo::AmplitudeConstraint::uniform(0.5));

  ASSERT_TRUE(autd.send(g));

  for (auto& dev : autd.geometry()) {
    auto [duties, phases] = autd.link<autd3::link::Audit>().duties_and_phases(dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 680; }));
    ASSERT_TRUE(std::ranges::any_of(phases, [](auto p) { return p != 0; }));
  }
}
#endif
