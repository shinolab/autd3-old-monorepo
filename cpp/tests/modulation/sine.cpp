// File: sine.cpp
// Project: modulation
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 29/11/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include <autd3/modulation/sine.hpp>

#include "utils.hpp"

TEST(Modulation, Sine) {
  auto autd = create_controller();

  ASSERT_TRUE(autd.send_async(autd3::modulation::Sine(150)
                                  .with_intensity(autd3::internal::EmitIntensity::maximum() / 2)
                                  .with_offset(autd3::internal::EmitIntensity::maximum() / 4)
                                  .with_phase(autd3::internal::pi / 2))
                  .get());

  for (auto& dev : autd.geometry()) {
    auto mod = autd.link().modulation(dev.idx());
    std::vector<uint8_t> mod_expect{126, 124, 119, 111, 100, 87, 73, 58, 44, 30, 18, 9, 3, 0, 1, 5, 12, 22, 34, 48, 63, 78, 92,  104, 114, 121, 125,
                                    126, 123, 117, 108, 96,  82, 68, 53, 39, 26, 15, 7, 2, 0, 2, 7, 15, 26, 39, 53, 68, 82, 96,  108, 117, 123, 126,
                                    125, 121, 114, 104, 92,  78, 63, 48, 34, 22, 12, 5, 1, 0, 3, 9, 18, 30, 44, 58, 73, 87, 100, 111, 119, 124};
    ASSERT_TRUE(std::ranges::equal(mod, mod_expect));
    ASSERT_EQ(5120, autd.link().modulation_frequency_division(dev.idx()));
  }

  ASSERT_TRUE(
      autd.send_async(autd3::modulation::Sine(150).with_sampling_config(autd3::internal::SamplingConfiguration::new_with_frequency_division(512)))
          .get());
  for (auto& dev : autd.geometry()) ASSERT_EQ(512, autd.link().modulation_frequency_division(dev.idx()));
}
