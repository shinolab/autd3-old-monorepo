// File: datagram.cpp
// Project: internal
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 11/11/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include <autd3/gain/uniform.hpp>
#include <autd3/internal/datagram.hpp>

#include "utils.hpp"

TEST(Internal, Silencer) {
  auto autd = create_controller();

  for (auto& dev : autd.geometry()) ASSERT_EQ(10, autd.link<autd3::link::Audit>().silencer_step(dev.idx()));

  ASSERT_TRUE(autd.send_async(autd3::internal::Silencer(20)).get());
  for (auto& dev : autd.geometry()) ASSERT_EQ(20, autd.link<autd3::link::Audit>().silencer_step(dev.idx()));

  ASSERT_TRUE(autd.send_async(autd3::internal::Silencer::disable()).get());
  for (auto& dev : autd.geometry()) ASSERT_EQ(0xFFFF, autd.link<autd3::link::Audit>().silencer_step(dev.idx()));

  ASSERT_TRUE(autd.send_async(autd3::internal::Silencer()).get());
  for (auto& dev : autd.geometry()) ASSERT_EQ(10, autd.link<autd3::link::Audit>().silencer_step(dev.idx()));
}

TEST(Internal, Clear) {
  auto autd = create_controller();

  ASSERT_TRUE(autd.send_async(autd3::gain::Uniform(1).with_phase(autd3::internal::pi)).get());
  for (auto& dev : autd.geometry()) {
    auto m = autd.link<autd3::link::Audit>().modulation(dev.idx());
    ASSERT_TRUE(std::ranges::all_of(m, [](auto d) { return d == 0; }));
    auto [duties, phases] = autd.link<autd3::link::Audit>().duties_and_phases(dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 256; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 256; }));
  }

  ASSERT_TRUE(autd.send_async(autd3::internal::Clear()).get());
  for (auto& dev : autd.geometry()) {
    auto m = autd.link<autd3::link::Audit>().modulation(dev.idx());
    ASSERT_TRUE(std::ranges::all_of(m, [](auto d) { return d == 0; }));
    auto [duties, phases] = autd.link<autd3::link::Audit>().duties_and_phases(dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 0; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }
}

TEST(Internal, UpdateFlags) {
  auto autd = create_controller();

  for (auto& dev : autd.geometry()) {
    dev.force_fan(true);
    ASSERT_EQ(0, autd.link<autd3::link::Audit>().fpga_flags(dev.idx()));
  }

  ASSERT_TRUE(autd.send_async(autd3::internal::UpdateFlags()).get());
  for (auto& dev : autd.geometry()) {
    ASSERT_EQ(1, autd.link<autd3::link::Audit>().fpga_flags(dev.idx()));
  }
}

TEST(Internal, Synchronize) {
  auto autd = autd3::internal::Controller::builder()
                  .add_device(autd3::internal::AUTD3(autd3::internal::Vector3::Zero(), autd3::internal::Vector3::Zero()))
                  .add_device(autd3::internal::AUTD3(autd3::internal::Vector3::Zero(), autd3::internal::Quaternion::Identity()))
                  .open_with_async(autd3::link::Audit::builder())
                  .get();

  ASSERT_TRUE(autd.send_async(autd3::internal::Synchronize()).get());
}

TEST(Internal, ConfigureModDelay) {
  auto autd = create_controller();

  for (auto& dev : autd.geometry()) {
    ASSERT_TRUE(std::ranges::all_of(autd.link<autd3::link::Audit>().mod_delays(dev.idx()), [](auto d) { return d == 0; }));
    for (auto& tr : dev) tr.set_mod_delay(1);
    ASSERT_TRUE(std::ranges::all_of(autd.link<autd3::link::Audit>().mod_delays(dev.idx()), [](auto d) { return d == 0; }));
  }

  ASSERT_TRUE(autd.send_async(autd3::internal::ConfigureModDelay()).get());
  for (auto& dev : autd.geometry()) {
    ASSERT_TRUE(std::ranges::all_of(autd.link<autd3::link::Audit>().mod_delays(dev.idx()), [](auto d) { return d == 1; }));
  }
}

TEST(Internal, ConfigureAmpFilter) {
  auto autd = create_controller();

  for (auto& dev : autd.geometry()) {
    ASSERT_TRUE(std::ranges::all_of(autd.link<autd3::link::Audit>().duty_filters(dev.idx()), [](auto d) { return d == 0; }));
    for (auto& tr : dev) tr.set_amp_filter(-1);
    ASSERT_TRUE(std::ranges::all_of(autd.link<autd3::link::Audit>().duty_filters(dev.idx()), [](auto d) { return d == 0; }));
  }

  ASSERT_TRUE(autd.send_async(autd3::internal::ConfigureAmpFilter()).get());
  for (auto& dev : autd.geometry()) {
    ASSERT_TRUE(std::ranges::all_of(autd.link<autd3::link::Audit>().duty_filters(dev.idx()), [](auto d) { return d == -256; }));
  }
}

TEST(Internal, ConfigurePhaseFilter) {
  auto autd = create_controller();

  for (auto& dev : autd.geometry()) {
    ASSERT_TRUE(std::ranges::all_of(autd.link<autd3::link::Audit>().phase_filters(dev.idx()), [](auto d) { return d == 0; }));
    for (auto& tr : dev) tr.set_phase_filter(-autd3::internal::pi);
    ASSERT_TRUE(std::ranges::all_of(autd.link<autd3::link::Audit>().phase_filters(dev.idx()), [](auto d) { return d == 0; }));
  }

  ASSERT_TRUE(autd.send_async(autd3::internal::ConfigurePhaseFilter()).get());
  for (auto& dev : autd.geometry()) {
    ASSERT_TRUE(std::ranges::all_of(autd.link<autd3::link::Audit>().phase_filters(dev.idx()), [](auto d) { return d == -256; }));
  }
}
