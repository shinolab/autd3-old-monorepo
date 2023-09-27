// File: group.cpp
// Project: gain
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 27/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include <autd3/gain/group.hpp>
#include <autd3/gain/null.hpp>
#include <autd3/gain/uniform.hpp>

#include "utils.hpp"

TEST(Gain, Group) {
  auto autd = create_controller();

  const auto cx = autd.geometry().center().x();

  ASSERT_TRUE(autd.send(autd3::gain::Group([cx](const auto&, const auto& tr) -> std::optional<const char*> {
                          if (tr.position().x() < cx) return "uniform";
                          return "null";
                        })
                            .set("uniform", autd3::gain::Uniform(0.5).with_phase(autd3::internal::pi))
                            .set("null", autd3::gain::Null())));

  for (auto& dev : autd.geometry()) {
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 0);
    for (auto& tr : dev) {
      if (tr.position().x() < cx) {
        ASSERT_EQ(680, duties[tr.local_idx()]);
        ASSERT_EQ(2048, phases[tr.local_idx()]);
      } else {
        ASSERT_EQ(8, duties[tr.local_idx()]);
        ASSERT_EQ(0, phases[tr.local_idx()]);
      }
    }
  }
}

TEST(Gain, GroupCheckOnlyForEnabled) {
  auto autd = create_controller();
  autd.geometry()[0].set_enable(false);

  std::vector check(autd.geometry().num_devices(), false);
  ASSERT_TRUE(autd.send(autd3::gain::Group([&check](const auto& dev, const auto& tr) -> std::optional<int> {
                          check[dev.idx()] = true;
                          return 0;
                        }).set(0, autd3::gain::Uniform(0.5).with_phase(autd3::internal::pi))));

  ASSERT_FALSE(check[0]);
  ASSERT_TRUE(check[1]);

  {
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, 0, 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 0; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }
  {
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, 1, 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 680; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 2048; }));
  }
}
