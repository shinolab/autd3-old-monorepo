// File: radiation_pressure.cpp
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

TEST(Modulation, RadiationPressure) {
  auto autd = create_controller();

  ASSERT_TRUE(autd.send_async(autd3::modulation::Sine(150).with_radiation_pressure()).get());

  for (auto& dev : autd.geometry()) {
    auto mod = autd.link().modulation(dev.idx());
    std::vector<uint8_t> mod_expect{181, 200, 217, 231, 243, 250, 254, 255, 252, 245, 236, 222, 206, 188, 166, 142, 116, 89,  60,  32,
                                    0,   32,  60,  89,  116, 142, 166, 188, 206, 222, 236, 245, 252, 255, 254, 250, 243, 231, 217, 200,
                                    181, 158, 134, 107, 78,  50,  23,  0,   39,  70,  97,  125, 150, 173, 194, 212, 227, 239, 248, 253,
                                    255, 253, 248, 239, 227, 212, 194, 173, 150, 125, 97,  70,  39,  0,   23,  50,  78,  107, 134, 158};
    ASSERT_TRUE(std::ranges::equal(mod, mod_expect));
    ASSERT_EQ(5120, autd.link().modulation_frequency_division(dev.idx()));
  }
}
