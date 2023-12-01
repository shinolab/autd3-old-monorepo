// File: modulation.cpp
// Project: modulation
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 01/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include <autd3/modulation/modulation.hpp>

#include "utils.hpp"

class BurstModulation final : public autd3::modulation::Modulation {
 public:
  [[nodiscard]] std::vector<autd3::internal::EmitIntensity> calc() const override {
    std::vector buffer(10, autd3::internal::EmitIntensity::minimum());
    buffer[0] = autd3::internal::EmitIntensity::maximum();
    return buffer;
  }

  explicit BurstModulation() noexcept : Modulation(autd3::internal::SamplingConfiguration::from_frequency_division(5120)) {}
};

TEST(Modulation, Modulation) {
  auto autd = create_controller();

  ASSERT_TRUE(autd.send_async(BurstModulation()).get());

  for (auto& dev : autd.geometry()) {
    auto mod = autd.link().modulation(dev.idx());
    std::vector<uint8_t> mod_expect{255, 0, 0, 0, 0, 0, 0, 0, 0, 0};
    ASSERT_TRUE(std::ranges::equal(mod, mod_expect));
    ASSERT_EQ(5120, autd.link().modulation_frequency_division(dev.idx()));
  }
}
