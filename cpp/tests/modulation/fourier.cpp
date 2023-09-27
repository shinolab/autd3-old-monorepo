// File: fourier.cpp
// Project: modulation
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 27/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include <autd3/modulation/fourier.hpp>
#include <ranges>

#include "utils.hpp"

TEST(Modulation, Fourier) {
  auto autd = create_controller();

  std::vector f{200};
  autd3::modulation::Fourier m =
      (autd3::modulation::Sine(50) + autd3::modulation::Sine(100))
          .add_component(autd3::modulation::Sine(150))
          .add_components_from_iter(f | std::ranges::views::transform([](const auto x) { return autd3::modulation::Sine(x); })) +
      autd3::modulation::Sine(250);

  ASSERT_TRUE(autd.send(m));

  for (auto& dev : autd.geometry()) {
    auto mod = autd3::link::Audit::modulation(autd, dev.idx());
    std::vector<uint8_t> mod_expect{85,  107, 130, 152, 169, 179, 178, 168, 152, 135, 119, 105, 94, 86, 82, 82, 85, 89, 95, 100,
                                    104, 106, 106, 103, 98,  93,  88,  83,  80,  79,  79,  81,  85, 88, 92, 94, 96, 95, 93, 89,
                                    85,  80,  77,  74,  74,  75,  77,  81,  85,  88,  90,  91,  89, 86, 81, 76, 71, 67, 65, 65,
                                    66,  70,  75,  80,  85,  87,  87,  83,  76,  66,  54,  42,  31, 22, 17, 17, 21, 31, 45, 63};
    ASSERT_TRUE(std::ranges::equal(mod, mod_expect));
    ASSERT_EQ(40960, autd3::link::Audit::modulation_frequency_division(autd, dev.idx()));
  }
}
