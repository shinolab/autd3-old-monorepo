// File: bessel.cpp
// Project: gain
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 09/10/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
// 

#include <autd3/gain/bessel.hpp>
#include <gtest/gtest.h>

#include "utils.hpp"

TEST(Gain, Bessel) {
  auto autd = create_controller();

  ASSERT_TRUE(autd.send(autd3::gain::Bessel(autd.geometry().center(), autd3::internal::Vector3::UnitZ(), autd3::internal::pi / 4).with_amp(0.5)));

  for (auto& dev : autd.geometry()) {
    auto [duties, phases] = autd.link<autd3::link::Audit>().duties_and_phases(dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 680; }));
    ASSERT_TRUE(std::ranges::any_of(phases, [](auto p) { return p != 0; }));
  }
}
