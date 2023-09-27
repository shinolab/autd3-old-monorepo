// File: special.cpp
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
#include <autd3/internal/special.hpp>
#include <ranges>

#include "utils.hpp"

TEST(Internal, Stop) {
  auto autd = create_controller();

  ASSERT_TRUE(autd.send(autd3::gain::Uniform(1).with_phase(autd3::internal::pi)));
  for (auto& dev : autd.geometry()) {
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 2048; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 2048; }));
  }

  ASSERT_TRUE(autd.send(autd3::internal::Stop()));
  for (auto& dev : autd.geometry()) {
    auto [duties, _] = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 0; }));
  }
}
