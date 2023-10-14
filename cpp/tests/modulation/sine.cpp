// File: sine.cpp
// Project: modulation
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 09/10/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <autd3/modulation/sine.hpp>
#include <gtest/gtest.h>

#include "utils.hpp"

TEST(Modulation, Sine) {
  auto autd = create_controller();

  ASSERT_TRUE(autd.send(autd3::modulation::Sine(150).with_amp(0.5).with_offset(0.25).with_phase(autd3::internal::pi / 2)));

  for (auto& dev : autd.geometry()) {
    auto mod = autd.link<autd3::link::Audit>().modulation(dev.idx());
    std::vector<uint8_t> mod_expect{85, 83, 79, 73, 66, 57, 47, 37, 28, 19, 11, 5, 1, 0, 0, 3, 7,  14, 22, 31, 41, 50, 60, 69, 76, 81, 84,
                                    84, 82, 78, 71, 63, 54, 44, 34, 25, 16, 9,  4, 1, 0, 1, 4, 9,  16, 25, 34, 44, 54, 63, 71, 78, 82, 84,
                                    84, 81, 76, 69, 60, 50, 41, 31, 22, 14, 7,  3, 0, 0, 1, 5, 11, 19, 28, 37, 47, 57, 66, 73, 79, 83};
    ASSERT_TRUE(std::ranges::equal(mod, mod_expect));
    ASSERT_EQ(40960, autd.link<autd3::link::Audit>().modulation_frequency_division(dev.idx()));
  }

  ASSERT_TRUE(autd.send(autd3::modulation::Sine(150).with_sampling_frequency_division(4096 / 8)));
  for (auto& dev : autd.geometry()) ASSERT_EQ(4096, autd.link<autd3::link::Audit>().modulation_frequency_division(dev.idx()));

  ASSERT_TRUE(autd.send(autd3::modulation::Sine(150).with_sampling_frequency(8e3)));
  for (auto& dev : autd.geometry()) ASSERT_EQ(20480, autd.link<autd3::link::Audit>().modulation_frequency_division(dev.idx()));

  ASSERT_TRUE(autd.send(autd3::modulation::Sine(150).with_sampling_period(std::chrono::microseconds(100))));
  for (auto& dev : autd.geometry()) ASSERT_EQ(16384, autd.link<autd3::link::Audit>().modulation_frequency_division(dev.idx()));
}
