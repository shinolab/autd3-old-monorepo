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
