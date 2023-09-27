// File: amplitudes.cpp
// Project: internal
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 27/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include <autd3/gain/uniform.hpp>
#include <autd3/internal/amplitudes.hpp>

#include "utils.hpp"

TEST(Internal, Amplitudes) {
  auto autd = autd3::internal::Controller::builder()
                  .advanced_phase()
                  .add_device(autd3::internal::AUTD3(autd3::internal::Vector3::Zero(), autd3::internal::Vector3::Zero()))
                  .add_device(autd3::internal::AUTD3(autd3::internal::Vector3::Zero(), autd3::internal::Quaternion::Identity()))
                  .open_with(autd3::link::Audit());

  for (auto& dev : autd.geometry()) {
    auto m = autd3::link::Audit::modulation(autd, dev.idx());
    ASSERT_TRUE(std::ranges::all_of(m, [](auto d) { return d == 0; }));
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 0; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }

  ASSERT_TRUE(autd.send(autd3::gain::Uniform(1).with_phase(autd3::internal::pi)));
  for (auto& dev : autd.geometry()) {
    auto m = autd3::link::Audit::modulation(autd, dev.idx());
    ASSERT_TRUE(std::ranges::all_of(m, [](auto d) { return d == 0; }));
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 0; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 2048; }));
  }

  ASSERT_TRUE(autd.send(autd3::internal::Amplitudes(1.0)));
  for (auto& dev : autd.geometry()) {
    auto m = autd3::link::Audit::modulation(autd, dev.idx());
    ASSERT_TRUE(std::ranges::all_of(m, [](auto d) { return d == 0; }));
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 2048; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 2048; }));
  }
}
