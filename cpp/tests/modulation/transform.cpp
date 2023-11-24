// File: transform.cpp
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

TEST(Modulation, Transform) {
  auto autd1 = create_controller();
  auto autd2 = create_controller();

  ASSERT_TRUE(autd1.send_async(autd3::modulation::Sine(150)).get());
  ASSERT_TRUE(autd2
                  .send_async(autd3::modulation::Sine(150).with_transform(
                      [](size_t, const autd3::internal::EmitIntensity v) { return autd3::internal::EmitIntensity(v.value() / 2); }))
                  .get());

  for (auto& dev : autd1.geometry()) {
    auto mod_expect = autd1.link().modulation(dev.idx());
    std::ranges::transform(mod_expect, mod_expect.begin(), [](const uint8_t x) { return x / 2; });
    auto mod = autd2.link().modulation(dev.idx());
    ASSERT_TRUE(std::ranges::equal(mod, mod_expect));
    ASSERT_EQ(5120, autd2.link().modulation_frequency_division(dev.idx()));
  }
}
