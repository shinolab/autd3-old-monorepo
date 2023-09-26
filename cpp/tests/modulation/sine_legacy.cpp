// File: sine_legacy.cpp
// Project: modulation
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 26/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <autd3/modulation/sine_legacy.hpp>
#include <gtest/gtest.h>

#include "utils.hpp"

TEST(Modulation, SineLegacy) {
  auto autd = create_controller();

  ASSERT_TRUE(autd.send(autd3::modulation::SineLegacy(150).with_amp(0.5).with_offset(0.25)));

  for (auto& dev : autd.geometry()) {
    auto mod = autd3::link::Audit::modulation(autd, dev.idx());
    std::vector<uint8_t> mod_expect{41, 50, 60, 68, 75, 81, 84, 84, 83, 78, 72, 64, 55, 45, 36, 26, 18, 11, 5, 1, 0, 0, 3, 8, 14, 22, 0};
    ASSERT_TRUE(std::ranges::equal(mod, mod_expect));
    ASSERT_EQ(40960, autd3::link::Audit::modulation_frequency_division(autd, dev.idx()));
  }

  ASSERT_TRUE(autd.send(autd3::modulation::SineLegacy(150).with_sampling_frequency_division(4096 / 8)));
  for (auto& dev : autd.geometry()) ASSERT_EQ(4096, autd3::link::Audit::modulation_frequency_division(autd, dev.idx()));

  ASSERT_TRUE(autd.send(autd3::modulation::SineLegacy(150).with_sampling_frequency(8e3)));
  for (auto& dev : autd.geometry()) ASSERT_EQ(20480, autd3::link::Audit::modulation_frequency_division(autd, dev.idx()));

  ASSERT_TRUE(autd.send(autd3::modulation::SineLegacy(150).with_sampling_period(std::chrono::microseconds(100))));
  for (auto& dev : autd.geometry()) ASSERT_EQ(16384, autd3::link::Audit::modulation_frequency_division(autd, dev.idx()));
}
