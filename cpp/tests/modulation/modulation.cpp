// File: modulation.cpp
// Project: modulation
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 09/10/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <autd3/modulation/modulation.hpp>
#include <gtest/gtest.h>

#include "utils.hpp"

class BurstModulation final : public autd3::modulation::Modulation {
 public:
  [[nodiscard]] std::vector<double> calc() const override {
    std::vector<double> buffer(10, 0);
    buffer[0] = 1.0;
    return buffer;
  }

  explicit BurstModulation() noexcept : Modulation(5120) {}
};

TEST(Modulation, Modulation) {
  auto autd = create_controller();

  ASSERT_TRUE(autd.send(BurstModulation()));

  for (auto& dev : autd.geometry()) {
    auto mod = autd.link<autd3::link::Audit>().modulation(dev.idx());
    std::vector<uint8_t> mod_expect{255, 0, 0, 0, 0, 0, 0, 0, 0, 0};
    ASSERT_TRUE(std::ranges::equal(mod, mod_expect));
    ASSERT_EQ(40960, autd.link<autd3::link::Audit>().modulation_frequency_division(dev.idx()));
  }
}
