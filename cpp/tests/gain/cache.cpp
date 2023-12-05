// File: cache.cpp
// Project: gain
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 05/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include <autd3/gain/cache.hpp>
#include <autd3/gain/gain.hpp>
#include <autd3/gain/uniform.hpp>
#include <autd3/internal/emit_intensity.hpp>

#include "utils.hpp"

TEST(Gain, Cache) {
  auto autd = create_controller();

  const auto g = autd3::gain::Uniform(0x80).with_phase(autd3::internal::Phase(0x90)).with_cache();

  g.init(autd.geometry());

  ASSERT_TRUE(autd.send_async(g).get());
  for (auto& dev : autd.geometry()) {
    ASSERT_TRUE(std::ranges::all_of(g.drives().at(dev.idx()), [](auto d) {
      return d == autd3::internal::Drive{autd3::internal::Phase(0x90), 0x80};
    }));
    auto [intensities, phases] = autd.link().intensities_and_phases(dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0x80; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0x90; }));
  }
}

class ForCacheTest final : public autd3::gain::Gain, public autd3::gain::IntoCache<ForCacheTest> {
 public:
  explicit ForCacheTest(size_t* cnt) : _cnt(cnt) {}

  [[nodiscard]] std::unordered_map<size_t, std::vector<autd3::internal::Drive>> calc(
      const autd3::internal::geometry::Geometry& geometry) const override {
    ++*_cnt;
    return transform(geometry, [&](const auto&, const auto&) {
      return autd3::internal::Drive{autd3::internal::Phase(0x90), autd3::internal::EmitIntensity(0x80)};
    });
  }

 private:
  size_t* _cnt;
};

TEST(Gain, CacheCheckOnce) {
  auto autd = create_controller();

  {
    size_t cnt = 0;
    ForCacheTest g(&cnt);
    ASSERT_TRUE(autd.send_async(g).get());
    ASSERT_EQ(cnt, 1);
    ASSERT_TRUE(autd.send_async(g).get());
    ASSERT_EQ(cnt, 2);
  }

  {
    size_t cnt = 0;
    ForCacheTest g(&cnt);
    auto gc = g.with_cache();
    ASSERT_EQ(cnt, 0);
    gc.init(autd.geometry());
    ASSERT_EQ(cnt, 1);
    ASSERT_TRUE(autd.send_async(gc).get());
    ASSERT_EQ(cnt, 1);
    ASSERT_TRUE(autd.send_async(gc).get());
    ASSERT_EQ(cnt, 1);
  }
}

TEST(Gain, CacheCheckOnlyForEnabled) {
  auto autd = create_controller();
  autd.geometry()[0].set_enable(false);

  size_t cnt = 0;
  auto g = ForCacheTest(&cnt).with_cache();
  ASSERT_TRUE(autd.send_async(g).get());

  ASSERT_FALSE(g.drives().contains(0));
  ASSERT_TRUE(g.drives().contains(1));

  {
    auto [intensities, phases] = autd.link().intensities_and_phases(0, 0);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }
  {
    auto [intensities, phases] = autd.link().intensities_and_phases(1, 0);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0x80; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0x90; }));
  }
}
