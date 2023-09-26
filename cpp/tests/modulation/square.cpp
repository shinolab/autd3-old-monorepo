// File: square.cpp
// Project: modulation
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 26/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
// 


#include <autd3/modulation/square.hpp>
#include <gtest/gtest.h>

#include "utils.hpp"

TEST(Modulation, Square) {
  auto autd = create_controller();

  ASSERT_TRUE(autd.send(autd3::modulation::Square(200).with_low(0.2).with_high(0.5).with_duty(0.1)));

  for (auto& dev : autd.geometry()) {
    auto mod = autd3::link::Audit::modulation(autd, dev.idx());
    std::vector<uint8_t> mod_expect{85, 85, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32};
    ASSERT_TRUE(std::ranges::equal(mod, mod_expect));
    ASSERT_EQ(40960, autd3::link::Audit::modulation_frequency_division(autd, dev.idx()));
  }

  ASSERT_TRUE(autd.send(autd3::modulation::Square(150).with_sampling_frequency_division(4096 / 8)));
  for (auto& dev : autd.geometry()) ASSERT_EQ(4096, autd3::link::Audit::modulation_frequency_division(autd, dev.idx()));

  ASSERT_TRUE(autd.send(autd3::modulation::Square(150).with_sampling_frequency(8e3)));
  for (auto& dev : autd.geometry()) ASSERT_EQ(20480, autd3::link::Audit::modulation_frequency_division(autd, dev.idx()));

  ASSERT_TRUE(autd.send(autd3::modulation::Square(150).with_sampling_period(std::chrono::microseconds(100))));
  for (auto& dev : autd.geometry()) ASSERT_EQ(16384, autd3::link::Audit::modulation_frequency_division(autd, dev.idx()));
}
