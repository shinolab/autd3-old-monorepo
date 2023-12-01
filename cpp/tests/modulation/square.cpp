// File: square.cpp
// Project: modulation
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 01/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include <autd3/modulation/square.hpp>

#include "utils.hpp"

TEST(Modulation, Square) {
  auto autd = create_controller();

  ASSERT_TRUE(autd.send_async(autd3::modulation::Square(200)
                                  .with_low(32)
                                  .with_high(85)
                                  .with_duty(0.1))
                  .get());

  for (auto& dev : autd.geometry()) {
    auto mod = autd.link().modulation(dev.idx());
    std::vector<uint8_t> mod_expect{85, 85, 32, 32, 32, 32, 32, 32, 32, 32,
                                    32, 32, 32, 32, 32, 32, 32, 32, 32, 32};
    ASSERT_TRUE(std::ranges::equal(mod, mod_expect));
    ASSERT_EQ(5120, autd.link().modulation_frequency_division(dev.idx()));
  }

  ASSERT_TRUE(
      autd.send_async(autd3::modulation::Square(150).with_sampling_config(
                          autd3::internal::SamplingConfiguration::
                              from_frequency_division(512)))
          .get());
  for (auto& dev : autd.geometry())
    ASSERT_EQ(512, autd.link().modulation_frequency_division(dev.idx()));
}
