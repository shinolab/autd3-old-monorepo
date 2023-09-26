// File: plane.cpp
// Project: gain
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 26/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
// 

#include <autd3/gain/plane.hpp>
#include <gtest/gtest.h>

#include "utils.hpp"

TEST(Gain, Plane) {
  auto autd = create_controller();

  ASSERT_TRUE(autd.send(autd3::gain::Plane(autd3::internal::Vector3::UnitZ()).with_amp(0.5)));

  for (auto& dev : autd.geometry()) {
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 680; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }
}
