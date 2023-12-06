// File: datagram.cpp
// Project: internal
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 06/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include <autd3/datagram/debug.hpp>
#include <autd3/datagram/mod_delay.hpp>
#include <autd3/gain/uniform.hpp>
#include <autd3/internal/datagram.hpp>

#include "utils.hpp"

TEST(Internal, Silencer) {
  auto autd = create_controller();

  for (auto& dev : autd.geometry()) {
    ASSERT_EQ(256, autd.link().silencer_step_intensity(dev.idx()));
    ASSERT_EQ(256, autd.link().silencer_step_phase(dev.idx()));
  }

  ASSERT_TRUE(autd.send_async(autd3::internal::Silencer(10, 20)).get());
  for (auto& dev : autd.geometry()) {
    ASSERT_EQ(10, autd.link().silencer_step_intensity(dev.idx()));
    ASSERT_EQ(20, autd.link().silencer_step_phase(dev.idx()));
  }

  ASSERT_TRUE(autd.send_async(autd3::internal::Silencer::disable()).get());
  for (auto& dev : autd.geometry()) {
    ASSERT_EQ(0xFFFF, autd.link().silencer_step_intensity(dev.idx()));
    ASSERT_EQ(0xFFFF, autd.link().silencer_step_phase(dev.idx()));
  }

  ASSERT_TRUE(autd.send_async(autd3::internal::Silencer()).get());
  for (auto& dev : autd.geometry()) {
    ASSERT_EQ(256, autd.link().silencer_step_intensity(dev.idx()));
    ASSERT_EQ(256, autd.link().silencer_step_phase(dev.idx()));
  }
}

TEST(Internal, ConfigureDebugOutputIdx) {
  auto autd = create_controller();

  for (auto& dev : autd.geometry()) {
    ASSERT_EQ(0xFF, autd.link().debug_output_idx(dev.idx()));
  }

  ASSERT_TRUE(autd.send_async(autd3::datagram::ConfigureDebugOutputIdx([](const autd3::internal::geometry::Device& dev) { return &dev[0]; })).get());
  for (auto& dev : autd.geometry()) {
    ASSERT_EQ(0, autd.link().debug_output_idx(dev.idx()));
  }

  ASSERT_TRUE(autd.send_async(autd3::datagram::ConfigureDebugOutputIdx(
                                  [](const autd3::internal::geometry::Device& dev) { return dev.idx() == 0 ? &dev[10] : nullptr; }))
                  .get());
  ASSERT_EQ(10, autd.link().debug_output_idx(0));
  ASSERT_EQ(0xFF, autd.link().debug_output_idx(1));
}

TEST(Internal, Clear) {
  auto autd = create_controller();

  ASSERT_TRUE(autd.send_async(autd3::gain::Uniform(0x80).with_phase(autd3::internal::Phase(0x90))).get());
  for (auto& dev : autd.geometry()) {
    auto m = autd.link().modulation(dev.idx());
    ASSERT_TRUE(std::ranges::all_of(m, [](auto d) { return d == 0xFF; }));
    auto [intensities, phases] = autd.link().intensities_and_phases(dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0x80; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0x90; }));
  }

  ASSERT_TRUE(autd.send_async(autd3::internal::Clear()).get());
  for (auto& dev : autd.geometry()) {
    auto m = autd.link().modulation(dev.idx());
    ASSERT_TRUE(std::ranges::all_of(m, [](auto d) { return d == 0xFF; }));
    auto [intensities, phases] = autd.link().intensities_and_phases(dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }
}

TEST(Internal, Synchronize) {
  auto autd = autd3::internal::ControllerBuilder()
                  .add_device(autd3::internal::geometry::AUTD3(autd3::internal::Vector3::Zero()))
                  .add_device(autd3::internal::geometry::AUTD3(autd3::internal::Vector3::Zero()))
                  .open_with_async(autd3::link::Audit::builder())
                  .get();

  ASSERT_TRUE(autd.send_async(autd3::internal::Synchronize()).get());
}

TEST(Internal, ConfigureModDelay) {
  auto autd = create_controller();

  for (auto& dev : autd.geometry()) {
    ASSERT_TRUE(std::ranges::all_of(autd.link().mod_delays(dev.idx()), [](auto d) { return d == 0; }));
  }

  ASSERT_TRUE(autd.send_async(autd3::datagram::ConfigureModDelay([](const autd3::internal::geometry::Device& dev,
                                                                    const autd3::internal::geometry::Transducer& tr) -> uint16_t { return 1; }))
                  .get());
  for (auto& dev : autd.geometry()) {
    ASSERT_TRUE(std::ranges::all_of(autd.link().mod_delays(dev.idx()), [](auto d) { return d == 1; }));
  }
}
