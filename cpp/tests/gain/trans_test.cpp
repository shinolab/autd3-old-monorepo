// File: trans_test.cpp
// Project: gain
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 11/11/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include <autd3/gain/trans_test.hpp>
#include <ranges>

#include "utils.hpp"

TEST(Gain, TransTest) {
  auto autd = create_controller();

  ASSERT_TRUE(autd.send_async(autd3::gain::TransducerTest().set(0, 0, autd3::internal::pi, 0.5).set(1, 248, autd3::internal::pi, 0.5)).get());

  {
    auto [duties, phases] = autd.link<autd3::link::Audit>().duties_and_phases(0, 0);
    ASSERT_EQ(85, duties[0]);
    ASSERT_EQ(256, phases[0]);
    ASSERT_TRUE(std::ranges::all_of(duties | std::ranges::views::drop(1), [](auto d) { return d == 0; }));
    ASSERT_TRUE(std::ranges::all_of(phases | std::ranges::views::drop(1), [](auto p) { return p == 0; }));
  }

  {
    auto [duties, phases] = autd.link<autd3::link::Audit>().duties_and_phases(1, 0);
    const auto idx = autd.geometry()[1].num_transducers() - 1;
    ASSERT_EQ(85, duties[idx]);
    ASSERT_EQ(256, phases[idx]);
    ASSERT_TRUE(std::ranges::all_of(duties | std::ranges::views::take(idx), [](auto d) { return d == 0; }));
    ASSERT_TRUE(std::ranges::all_of(phases | std::ranges::views::take(idx), [](auto p) { return p == 0; }));
  }
}
