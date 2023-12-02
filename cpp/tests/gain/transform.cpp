// File: transform.cpp
// Project: gain
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 02/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include <autd3/gain/uniform.hpp>

#include "utils.hpp"

TEST(Gain, Transform) {
  auto autd = create_controller();

  ASSERT_TRUE(autd.send_async(autd3::gain::Uniform(0x80)
                                  .with_phase(autd3::internal::Phase(128))
                                  .with_transform([](const autd3::internal::geometry::Device& dev, const autd3::internal::geometry::Transducer&,
                                                     const autd3::internal::Drive d) -> autd3::internal::Drive {
                                    if (dev.idx() == 0) {
                                      return autd3::internal::Drive{autd3::internal::Phase(d.phase.value() + 32), d.intensity};
                                    }
                                    return autd3::internal::Drive{autd3::internal::Phase(d.phase.value() - 32), d.intensity};
                                  }))
                  .get());

  {
    auto [intensities, phases] = autd.link().intensities_and_phases(0, 0);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0x80; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 128 + 32; }));
  }

  {
    auto [intensities, phases] = autd.link().intensities_and_phases(1, 0);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0x80; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 128 - 32; }));
  }
}

TEST(Gain, TransformCheckOnlyForEnabled) {
  auto autd = create_controller();
  autd.geometry()[0].set_enable(false);

  std::vector cnt(autd.geometry().num_devices(), false);
  ASSERT_TRUE(autd.send_async(autd3::gain::Uniform(0x80)
                                  .with_phase(autd3::internal::Phase(0x90))
                                  .with_transform([&cnt](const autd3::internal::geometry::Device& dev, const autd3::internal::geometry::Transducer&,
                                                         const autd3::internal::Drive d) -> autd3::internal::Drive {
                                    cnt[dev.idx()] = true;
                                    return d;
                                  }))
                  .get());

  ASSERT_FALSE(cnt[0]);
  ASSERT_TRUE(cnt[1]);

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
