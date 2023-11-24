// File: trans_test.cpp
// Project: gain
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 24/11/2023
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

  ASSERT_TRUE(autd.send_async(autd3::gain::TransducerTest()
                                  .set(autd.geometry()[0][0], autd3::internal::pi, 0x80)
                                  .set(autd.geometry()[1][248], autd3::internal::pi, 0x80))
                  .get());

  {
    auto [intensities, phases] = autd.link().intensities_and_phases(0, 0);
    ASSERT_EQ(0x80, intensities[0]);
    ASSERT_EQ(128, phases[0]);
    ASSERT_TRUE(std::ranges::all_of(intensities | std::ranges::views::drop(1), [](auto d) { return d == 0; }));
    ASSERT_TRUE(std::ranges::all_of(phases | std::ranges::views::drop(1), [](auto p) { return p == 0; }));
  }

  {
    auto [intensities, phases] = autd.link().intensities_and_phases(1, 0);
    const auto idx = autd.geometry()[1].num_transducers() - 1;
    ASSERT_EQ(0x80, intensities[idx]);
    ASSERT_EQ(128, phases[idx]);
    ASSERT_TRUE(std::ranges::all_of(intensities | std::ranges::views::take(idx), [](auto d) { return d == 0; }));
    ASSERT_TRUE(std::ranges::all_of(phases | std::ranges::views::take(idx), [](auto p) { return p == 0; }));
  }
}
