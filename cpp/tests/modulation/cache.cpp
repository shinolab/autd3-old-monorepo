// File: cache.cpp
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

#include <autd3/modulation/modulation.hpp>
#include <autd3/modulation/sine.hpp>

#include "utils.hpp"

TEST(Modulation, Cache) {
  auto autd1 = create_controller();
  auto autd2 = create_controller();

  ASSERT_TRUE(autd1.send_async(autd3::modulation::Sine(150)).get());
  ASSERT_TRUE(autd2.send_async(autd3::modulation::Sine(150).with_cache()).get());

  for (auto& dev : autd1.geometry()) {
    auto mod = autd2.link().modulation(dev.idx());
    auto mod_expect = autd1.link().modulation(dev.idx());
    ASSERT_TRUE(std::ranges::equal(mod, mod_expect));
    ASSERT_EQ(5120, autd2.link().modulation_frequency_division(dev.idx()));
  }
}

class ForModulationCacheTest final : public autd3::modulation::Modulation, public autd3::modulation::IntoCache<ForModulationCacheTest> {
 public:
  [[nodiscard]] std::vector<autd3::internal::EmitIntensity> calc() const override {
    ++*_cnt;
    return {autd3::internal::EmitIntensity::maximum(), autd3::internal::EmitIntensity::maximum()};
  }

  explicit ForModulationCacheTest(size_t* cnt) noexcept
      : Modulation(autd3::internal::SamplingConfiguration::new_with_frequency_division(5120)), _cnt(cnt) {}

 private:
  size_t* _cnt;
};

TEST(Modulation, CacheCheckOnce) {
  auto autd = create_controller();

  {
    size_t cnt = 0;
    ForModulationCacheTest m(&cnt);
    ASSERT_TRUE(autd.send_async(m).get());
    ASSERT_EQ(cnt, 1);
    ASSERT_TRUE(autd.send_async(m).get());
    ASSERT_EQ(cnt, 2);
  }

  {
    size_t cnt = 0;
    ForModulationCacheTest g(&cnt);
    auto gc = g.with_cache();
    ASSERT_TRUE(autd.send_async(gc).get());
    ASSERT_EQ(cnt, 1);
    ASSERT_TRUE(autd.send_async(gc).get());
    ASSERT_EQ(cnt, 1);
  }
}
