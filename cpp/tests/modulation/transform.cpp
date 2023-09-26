// File: transform.cpp
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

TEST(Modulation, Transform) {
  auto autd = create_controller();

  const auto m = autd3::modulation::Sine(150).with_transform([](size_t, const double v)  { return v/2; });
  ASSERT_TRUE(autd.send(m));

  for (auto& dev : autd.geometry()) {
    auto mod = autd3::link::Audit::modulation(autd, dev.idx());
    std::vector<uint8_t> mod_expect{41, 50, 60, 69, 76, 81, 84, 84, 82, 78, 71, 63, 54, 44, 34, 25, 16, 9, 4, 1, 0, 1, 4, 9,  16, 25, 34,
                                    44, 54, 63, 71, 78, 82, 84, 84, 81, 76, 69, 60, 50, 41, 31, 22, 14, 7, 3, 0, 0, 1, 5, 11, 19, 28, 37,
                                    47, 57, 66, 73, 79, 83, 85, 83, 79, 73, 66, 57, 47, 37, 28, 19, 11, 5, 1, 0, 0, 3, 7, 14, 22, 31};
    ASSERT_TRUE(std::ranges::equal(mod, mod_expect));
    ASSERT_EQ(40960, autd3::link::Audit::modulation_frequency_division(autd, dev.idx()));
  }
}
