// File: fourier.cpp
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

  ASSERT_TRUE(autd.send_async(m).get());

  for (auto& dev : autd.geometry()) {
    auto mod = autd.link().modulation(dev.idx());
    std::vector<uint8_t> mod_expect{128, 157, 183, 205, 220, 227, 227, 219, 206, 189, 171, 153, 140, 129, 124, 124, 127, 133, 141, 147,
                                    153, 155, 155, 151, 146, 139, 131, 125, 121, 119, 120, 123, 127, 132, 137, 140, 142, 141, 138, 133,
                                    128, 121, 116, 113, 112, 114, 117, 122, 127, 132, 135, 135, 133, 129, 123, 116, 108, 103, 100, 99,
                                    102, 107, 114, 121, 127, 130, 130, 125, 115, 101, 84,  66,  49,  35,  27,  27,  34,  49,  71,  98};
    ASSERT_TRUE(std::ranges::equal(mod, mod_expect));
    ASSERT_EQ(5120, autd.link().modulation_frequency_division(dev.idx()));
  }
}
