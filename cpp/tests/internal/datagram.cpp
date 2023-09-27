// File: datagram.cpp
// Project: internal
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 27/09/2023
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

  for (auto& dev : autd.geometry()) ASSERT_EQ(10, autd3::link::Audit::silencer_step(autd, dev.idx()));

  ASSERT_TRUE(autd.send(autd3::internal::Silencer(20)));
  for (auto& dev : autd.geometry()) ASSERT_EQ(20, autd3::link::Audit::silencer_step(autd, dev.idx()));

  ASSERT_TRUE(autd.send(autd3::internal::Silencer::disable()));
  for (auto& dev : autd.geometry()) ASSERT_EQ(0xFFFF, autd3::link::Audit::silencer_step(autd, dev.idx()));

  ASSERT_TRUE(autd.send(autd3::internal::Silencer()));
  for (auto& dev : autd.geometry()) ASSERT_EQ(10, autd3::link::Audit::silencer_step(autd, dev.idx()));
}

TEST(Internal, Clear) {
  auto autd = create_controller();

  ASSERT_TRUE(autd.send(autd3::gain::Uniform(1).with_phase(autd3::internal::pi)));
  for (auto& dev : autd.geometry()) {
    auto m = autd3::link::Audit::modulation(autd, dev.idx());
    ASSERT_TRUE(std::ranges::all_of(m, [](auto d) { return d == 0; }));
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 2048; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 2048; }));
  }

  ASSERT_TRUE(autd.send(autd3::internal::Clear()));
  for (auto& dev : autd.geometry()) {
    auto m = autd3::link::Audit::modulation(autd, dev.idx());
    ASSERT_TRUE(std::ranges::all_of(m, [](auto d) { return d == 0; }));
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 0; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }
}

TEST(Internal, UpdateFlags) {
  auto autd = create_controller();

  for (auto& dev : autd.geometry()) {
    dev.force_fan(true);
    ASSERT_EQ(0, autd3::link::Audit::fpga_flags(autd, dev.idx()));
  }

  ASSERT_TRUE(autd.send(autd3::internal::UpdateFlags()));
  for (auto& dev : autd.geometry()) {
    ASSERT_EQ(1, autd3::link::Audit::fpga_flags(autd, dev.idx()));
  }
}

TEST(Internal, Synchronize) {
  auto autd = autd3::internal::Controller::builder()
                  .advanced()
                  .add_device(autd3::internal::AUTD3(autd3::internal::Vector3::Zero(), autd3::internal::Vector3::Zero()))
                  .add_device(autd3::internal::AUTD3(autd3::internal::Vector3::Zero(), autd3::internal::Quaternion::Identity()))
                  .open_with(autd3::link::Audit());

  for (auto& dev : autd.geometry()) {
    ASSERT_TRUE(std::ranges::all_of(autd3::link::Audit::cycles(autd, dev.idx()), [](auto c) { return c == 4096; }));
    for (auto& tr : dev) tr.set_cycle(4000);
    ASSERT_TRUE(std::ranges::all_of(autd3::link::Audit::cycles(autd, dev.idx()), [](auto c) { return c == 4096; }));
  }

  ASSERT_TRUE(autd.send(autd3::internal::Synchronize()));
  for (auto& dev : autd.geometry()) {
    ASSERT_TRUE(std::ranges::all_of(autd3::link::Audit::cycles(autd, dev.idx()), [](auto c) { return c == 4000; }));
  }
}

TEST(Internal, ConfigureModDelay) {
  auto autd = create_controller();

  for (auto& dev : autd.geometry()) {
    ASSERT_TRUE(std::ranges::all_of(autd3::link::Audit::mod_delays(autd, dev.idx()), [](auto d) { return d == 0; }));
    for (auto& tr : dev) tr.set_mod_delay(1);
    ASSERT_TRUE(std::ranges::all_of(autd3::link::Audit::mod_delays(autd, dev.idx()), [](auto d) { return d == 0; }));
  }

  ASSERT_TRUE(autd.send(autd3::internal::ConfigureModDelay()));
  for (auto& dev : autd.geometry()) {
    ASSERT_TRUE(std::ranges::all_of(autd3::link::Audit::mod_delays(autd, dev.idx()), [](auto d) { return d == 1; }));
  }
}

TEST(Internal, ConfigureAmpFilter) {
  auto autd = create_controller();

  for (auto& dev : autd.geometry()) {
    ASSERT_TRUE(std::ranges::all_of(autd3::link::Audit::duty_filters(autd, dev.idx()), [](auto d) { return d == 0; }));
    for (auto& tr : dev) tr.set_amp_filter(-1);
    ASSERT_TRUE(std::ranges::all_of(autd3::link::Audit::duty_filters(autd, dev.idx()), [](auto d) { return d == 0; }));
  }

  ASSERT_TRUE(autd.send(autd3::internal::ConfigureAmpFilter()));
  for (auto& dev : autd.geometry()) {
    ASSERT_TRUE(std::ranges::all_of(autd3::link::Audit::duty_filters(autd, dev.idx()), [](auto d) { return d == -2048; }));
  }
}

TEST(Internal, ConfigurePhaseFilter) {
  auto autd = create_controller();

  for (auto& dev : autd.geometry()) {
    ASSERT_TRUE(std::ranges::all_of(autd3::link::Audit::phase_filters(autd, dev.idx()), [](auto d) { return d == 0; }));
    for (auto& tr : dev) tr.set_phase_filter(-autd3::internal::pi);
    ASSERT_TRUE(std::ranges::all_of(autd3::link::Audit::phase_filters(autd, dev.idx()), [](auto d) { return d == 0; }));
  }

  ASSERT_TRUE(autd.send(autd3::internal::ConfigurePhaseFilter()));
  for (auto& dev : autd.geometry()) {
    ASSERT_TRUE(std::ranges::all_of(autd3::link::Audit::phase_filters(autd, dev.idx()), [](auto d) { return d == -2048; }));
  }
}
