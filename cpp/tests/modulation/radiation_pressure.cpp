// File: radiation_pressure.cpp
// Project: modulation
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 26/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <autd3/modulation/sine.hpp>
#include <gtest/gtest.h>

#include "utils.hpp"

TEST(Modulation, RadiationPressure) {
  auto autd = create_controller();

  ASSERT_TRUE(autd.send(autd3::modulation::Sine(150).with_radiation_pressure()));

  for (auto& dev : autd.geometry()) {
    auto mod = autd3::link::Audit::modulation(autd, dev.idx());
    std::vector<uint8_t> mod_expect{127, 146, 165, 184, 204, 223, 242, 248, 229, 210, 191, 172, 153, 133, 114, 95,  76,  57,  38,  19,
                                    0,   19,  38,  57,  76,  95,  114, 133, 153, 172, 191, 210, 229, 248, 242, 223, 204, 184, 165, 146,
                                    127, 108, 89,  70,  51,  31,  12,  6,   25,  44,  63,  82,  101, 121, 140, 159, 178, 197, 216, 235,
                                    255, 235, 216, 197, 178, 159, 140, 121, 102, 82,  63,  44,  25,  6,   12,  31,  50,  70,  89,  108};
    ASSERT_TRUE(std::ranges::equal(mod, mod_expect));
    ASSERT_EQ(40960, autd3::link::Audit::modulation_frequency_division(autd, dev.idx()));
  }
}
