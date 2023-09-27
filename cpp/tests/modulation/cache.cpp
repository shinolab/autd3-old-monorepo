// File: cache.cpp
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

#include <autd3/modulation/modulation.hpp>
#include <autd3/modulation/sine.hpp>

#include "utils.hpp"

TEST(Modulation, Cache) {
  auto autd = create_controller();

  ASSERT_TRUE(autd.send(autd3::modulation::Sine(150).with_cache()));

  for (auto& dev : autd.geometry()) {
    auto mod = autd3::link::Audit::modulation(autd, dev.idx());
    std::vector<uint8_t> mod_expect{85,  107, 132, 157, 183, 210, 236, 245, 219, 192, 166, 140, 115, 92,  70,  50,  33,  19,  8,   2,
                                    0,   2,   8,   19,  33,  50,  70,  92,  115, 140, 166, 192, 219, 245, 236, 210, 183, 157, 132, 107,
                                    85,  63,  44,  28,  15,  6,   0,   0,   3,   11,  23,  39,  57,  77,  100, 123, 148, 174, 201, 227,
                                    255, 227, 201, 174, 148, 123, 100, 77,  57,  39,  23,  11,  3,   0,   0,   6,   15,  28,  44,  63};
    ASSERT_TRUE(std::ranges::equal(mod, mod_expect));
    ASSERT_EQ(40960, autd3::link::Audit::modulation_frequency_division(autd, dev.idx()));
  }
}

class ForModulationCacheTest final : public autd3::modulation::Modulation {
 public:
  [[nodiscard]] std::vector<double> calc() const override {
    ++*_cnt;
    return {1, 1};
  }

  explicit ForModulationCacheTest(size_t* cnt) noexcept : Modulation(5120), _cnt(cnt) {}

  AUTD3_IMPL_WITH_CACHE_MODULATION

 private:
  size_t* _cnt;
};

TEST(Modulation, CacheCheckOnce) {
  auto autd = create_controller();

  {
    size_t cnt = 0;
    ForModulationCacheTest m(&cnt);
    ASSERT_TRUE(autd.send(m));
    ASSERT_EQ(cnt, 1);
    ASSERT_TRUE(autd.send(m));
    ASSERT_EQ(cnt, 2);
  }

  {
    size_t cnt = 0;
    ForModulationCacheTest g(&cnt);
    auto gc = g.with_cache();
    ASSERT_TRUE(autd.send(gc));
    ASSERT_EQ(cnt, 1);
    ASSERT_TRUE(autd.send(gc));
    ASSERT_EQ(cnt, 1);
  }
}
