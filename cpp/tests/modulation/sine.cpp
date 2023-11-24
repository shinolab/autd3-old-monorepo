// File: sine.cpp
// Project: modulation
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 24/11/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include <autd3/modulation/sine.hpp>

#include "utils.hpp"

TEST(Modulation, Sine) {
  auto autd = create_controller();

  ASSERT_TRUE(autd.send_async(autd3::modulation::Sine(150).with_amp(0.5).with_offset(0.25).with_phase(autd3::internal::pi / 2)).get());

  for (auto& dev : autd.geometry()) {
    auto mod = autd.link().modulation(dev.idx());
    std::vector<uint8_t> mod_expect{128, 126, 121, 112, 101, 88, 74, 59, 44, 30, 19, 9, 3, 0, 1, 5, 12, 22, 35, 49, 64, 79, 93,  105, 115, 123, 127,
                                    127, 124, 118, 109, 97,  83, 69, 54, 39, 26, 15, 7, 2, 0, 2, 7, 15, 26, 39, 54, 69, 83, 97,  109, 118, 124, 127,
                                    127, 123, 115, 105, 93,  79, 64, 49, 35, 22, 12, 5, 1, 0, 3, 9, 19, 30, 44, 59, 74, 88, 101, 112, 121, 126};
    ASSERT_TRUE(std::ranges::equal(mod, mod_expect));
    ASSERT_EQ(5120, autd.link().modulation_frequency_division(dev.idx()));
  }

  ASSERT_TRUE(autd.send_async(autd3::modulation::Sine(150).with_sampling_configuration(
                                  autd3::internal::SamplingConfiguration::new_with_frequency_division(512)))
                  .get());
  for (auto& dev : autd.geometry()) ASSERT_EQ(512, autd.link().modulation_frequency_division(dev.idx()));
}
