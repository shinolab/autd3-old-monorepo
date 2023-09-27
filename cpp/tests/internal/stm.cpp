// File: stm.cpp
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

#include <autd3/gain/focus.hpp>
#include <autd3/gain/uniform.hpp>
#include <autd3/internal/stm.hpp>
#include <ranges>

#include "utils.hpp"

TEST(STMTest, FocusSTM) {
  auto autd = create_controller();

  constexpr double radius = 30.0;
  constexpr int size = 2;
  autd3::internal::Vector3 center = autd.geometry().center() + autd3::internal::Vector3(0, 0, 150);
  auto stm = autd3::internal::FocusSTM(1).add_foci_from_iter(
      std::views::iota(0) | std::views::take(size) | std::views::transform([&](auto i) {
        const double theta = 2 * autd3::internal::pi * i / size;
        return autd3::internal::ControlPoint{center + autd3::internal::Vector3(radius * cos(theta), radius * sin(theta), 0), 0};
      }));
  ASSERT_TRUE(autd.send(stm));
  for (const auto& dev : autd.geometry()) {
    ASSERT_FALSE(autd3::link::Audit::is_stm_gain_mode(autd, dev.idx()));
  }

  ASSERT_EQ(1, stm.frequency());
  ASSERT_EQ(2, stm.sampling_frequency());
  ASSERT_EQ(10240000u, stm.sampling_frequency_division());
  ASSERT_EQ(std::chrono::microseconds(500000), stm.sampling_period());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(81920000u, autd3::link::Audit::stm_frequency_division(autd, dev.idx()));
  }

  ASSERT_EQ(std::nullopt, stm.start_idx());
  ASSERT_EQ(std::nullopt, stm.finish_idx());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(-1, autd3::link::Audit::stm_start_idx(autd, dev.idx()));
    ASSERT_EQ(-1, autd3::link::Audit::stm_finish_idx(autd, dev.idx()));
  }

  stm.with_start_idx(0);
  ASSERT_TRUE(autd.send(stm));
  ASSERT_EQ(0, stm.start_idx());
  ASSERT_EQ(std::nullopt, stm.finish_idx());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(0, autd3::link::Audit::stm_start_idx(autd, dev.idx()));
    ASSERT_EQ(-1, autd3::link::Audit::stm_finish_idx(autd, dev.idx()));
  }

  stm.with_start_idx(std::nullopt).with_finish_idx(0);
  ASSERT_TRUE(autd.send(stm));
  ASSERT_EQ(std::nullopt, stm.start_idx());
  ASSERT_EQ(0, stm.finish_idx());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(-1, autd3::link::Audit::stm_start_idx(autd, dev.idx()));
    ASSERT_EQ(0, autd3::link::Audit::stm_finish_idx(autd, dev.idx()));
  }

  stm = autd3::internal::FocusSTM::with_sampling_frequency_division(512).add_focus(center).add_focus(center);
  ASSERT_TRUE(autd.send(stm));
  ASSERT_EQ(20000.0, stm.frequency());
  ASSERT_EQ(2 * 20000.0, stm.sampling_frequency());
  ASSERT_EQ(512u, stm.sampling_frequency_division());
  ASSERT_EQ(std::chrono::microseconds(25), stm.sampling_period());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(4096u, autd3::link::Audit::stm_frequency_division(autd, dev.idx()));
  }

  stm = autd3::internal::FocusSTM::with_sampling_frequency(20e3).add_focus(center).add_focus(center);
  ASSERT_TRUE(autd.send(stm));
  ASSERT_EQ(10000, stm.frequency());
  ASSERT_EQ(2 * 10000, stm.sampling_frequency());
  ASSERT_EQ(1024u, stm.sampling_frequency_division());
  ASSERT_EQ(std::chrono::microseconds(50), stm.sampling_period());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(4096u * 2, autd3::link::Audit::stm_frequency_division(autd, dev.idx()));
  }

  stm = autd3::internal::FocusSTM::with_sampling_period(std::chrono::microseconds(25)).add_focus(center).add_focus(center);
  ASSERT_TRUE(autd.send(stm));
  ASSERT_EQ(20000.0, stm.frequency());
  ASSERT_EQ(2 * 20000.0, stm.sampling_frequency());
  ASSERT_EQ(512u, stm.sampling_frequency_division());
  ASSERT_EQ(std::chrono::microseconds(25), stm.sampling_period());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(4096u, autd3::link::Audit::stm_frequency_division(autd, dev.idx()));
  }

  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(2u, autd3::link::Audit::stm_cycle(autd, dev.idx()));
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 0);
    ASSERT_TRUE(std::ranges::any_of(duties, [](auto d) { return d != 0; }));
    ASSERT_TRUE(std::ranges::any_of(phases, [](auto p) { return p != 0; }));

    std::tie(duties, phases) = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 1);
    ASSERT_TRUE(std::ranges::any_of(duties, [](auto d) { return d != 0; }));
    ASSERT_TRUE(std::ranges::any_of(phases, [](auto p) { return p != 0; }));
  }
}

TEST(STMTest, GainSTMLegacy) {
  auto autd = autd3::internal::Controller::builder()
                  .add_device(autd3::internal::AUTD3(autd3::internal::Vector3::Zero(), autd3::internal::Vector3::Zero()))
                  .add_device(autd3::internal::AUTD3(autd3::internal::Vector3::Zero(), autd3::internal::Quaternion::Identity()))
                  .open_with(autd3::link::Audit());

  constexpr double radius = 30.0;
  constexpr int size = 2;
  autd3::internal::Vector3 center = autd.geometry().center() + autd3::internal::Vector3(0, 0, 150);
  auto stm = autd3::internal::GainSTM(1).add_gains_from_iter(std::views::iota(0) | std::views::take(size) | std::views::transform([&](auto i) {
                                                               const double theta = 2 * autd3::internal::pi * i / size;
                                                               return autd3::gain::Focus(
                                                                   center + autd3::internal::Vector3(radius * cos(theta), radius * sin(theta), 0));
                                                             }));
  ASSERT_TRUE(autd.send(stm));
  for (const auto& dev : autd.geometry()) {
    ASSERT_TRUE(autd3::link::Audit::is_stm_gain_mode(autd, dev.idx()));
  }

  ASSERT_EQ(1, stm.frequency());
  ASSERT_EQ(2, stm.sampling_frequency());
  ASSERT_EQ(10240000u, stm.sampling_frequency_division());
  ASSERT_EQ(std::chrono::microseconds(500000), stm.sampling_period());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(81920000u, autd3::link::Audit::stm_frequency_division(autd, dev.idx()));
  }

  ASSERT_EQ(std::nullopt, stm.start_idx());
  ASSERT_EQ(std::nullopt, stm.finish_idx());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(-1, autd3::link::Audit::stm_start_idx(autd, dev.idx()));
    ASSERT_EQ(-1, autd3::link::Audit::stm_finish_idx(autd, dev.idx()));
  }

  stm.with_start_idx(0);
  ASSERT_TRUE(autd.send(stm));
  ASSERT_EQ(0, stm.start_idx());
  ASSERT_EQ(std::nullopt, stm.finish_idx());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(0, autd3::link::Audit::stm_start_idx(autd, dev.idx()));
    ASSERT_EQ(-1, autd3::link::Audit::stm_finish_idx(autd, dev.idx()));
  }

  stm.with_start_idx(std::nullopt).with_finish_idx(0);
  ASSERT_TRUE(autd.send(stm));
  ASSERT_EQ(std::nullopt, stm.start_idx());
  ASSERT_EQ(0, stm.finish_idx());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(-1, autd3::link::Audit::stm_start_idx(autd, dev.idx()));
    ASSERT_EQ(0, autd3::link::Audit::stm_finish_idx(autd, dev.idx()));
  }

  stm = autd3::internal::GainSTM::with_sampling_frequency_division(512).add_gain(autd3::gain::Uniform(1)).add_gain(autd3::gain::Uniform(0.5));
  ASSERT_TRUE(autd.send(stm));
  ASSERT_EQ(20000.0, stm.frequency());
  ASSERT_EQ(2 * 20000.0, stm.sampling_frequency());
  ASSERT_EQ(512u, stm.sampling_frequency_division());
  ASSERT_EQ(std::chrono::microseconds(25), stm.sampling_period());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(4096u, autd3::link::Audit::stm_frequency_division(autd, dev.idx()));
  }

  stm = autd3::internal::GainSTM::with_sampling_frequency(20e3).add_gain(autd3::gain::Uniform(1)).add_gain(autd3::gain::Uniform(0.5));
  ASSERT_TRUE(autd.send(stm));
  ASSERT_EQ(10000, stm.frequency());
  ASSERT_EQ(2 * 10000, stm.sampling_frequency());
  ASSERT_EQ(1024u, stm.sampling_frequency_division());
  ASSERT_EQ(std::chrono::microseconds(50), stm.sampling_period());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(4096u * 2, autd3::link::Audit::stm_frequency_division(autd, dev.idx()));
  }

  stm = autd3::internal::GainSTM::with_sampling_period(std::chrono::microseconds(25))
            .add_gain(autd3::gain::Uniform(1))
            .add_gain(autd3::gain::Uniform(0.5));
  ASSERT_TRUE(autd.send(stm));
  ASSERT_EQ(20000.0, stm.frequency());
  ASSERT_EQ(2 * 20000.0, stm.sampling_frequency());
  ASSERT_EQ(512u, stm.sampling_frequency_division());
  ASSERT_EQ(std::chrono::microseconds(25), stm.sampling_period());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(4096u, autd3::link::Audit::stm_frequency_division(autd, dev.idx()));
  }

  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(2u, autd3::link::Audit::stm_cycle(autd, dev.idx()));
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 2048; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));

    std::tie(duties, phases) = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 1);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 680; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }

  stm.with_mode(autd3::internal::native_methods::GainSTMMode::PhaseFull);
  ASSERT_TRUE(autd.send(stm));
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(2u, autd3::link::Audit::stm_cycle(autd, dev.idx()));
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 2048; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));

    std::tie(duties, phases) = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 1);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 2048; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }

  stm.with_mode(autd3::internal::native_methods::GainSTMMode::PhaseHalf);
  ASSERT_TRUE(autd.send(stm));
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(2u, autd3::link::Audit::stm_cycle(autd, dev.idx()));
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 2048; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));

    std::tie(duties, phases) = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 1);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 2048; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }
}

TEST(STMTest, GainSTMAdvanced) {
  auto autd = autd3::internal::Controller::builder()
                  .advanced()
                  .add_device(autd3::internal::AUTD3(autd3::internal::Vector3::Zero(), autd3::internal::Vector3::Zero()))
                  .add_device(autd3::internal::AUTD3(autd3::internal::Vector3::Zero(), autd3::internal::Quaternion::Identity()))
                  .open_with(autd3::link::Audit());

  constexpr double radius = 30.0;
  constexpr int size = 2;
  autd3::internal::Vector3 center = autd.geometry().center() + autd3::internal::Vector3(0, 0, 150);
  auto stm = autd3::internal::GainSTM(1).add_gains_from_iter(std::views::iota(0) | std::views::take(size) | std::views::transform([&](auto i) {
                                                               const double theta = 2 * autd3::internal::pi * i / size;
                                                               return autd3::gain::Focus(
                                                                   center + autd3::internal::Vector3(radius * cos(theta), radius * sin(theta), 0));
                                                             }));
  ASSERT_TRUE(autd.send(stm));
  for (const auto& dev : autd.geometry()) {
    ASSERT_TRUE(autd3::link::Audit::is_stm_gain_mode(autd, dev.idx()));
  }

  ASSERT_EQ(1, stm.frequency());
  ASSERT_EQ(2, stm.sampling_frequency());
  ASSERT_EQ(10240000u, stm.sampling_frequency_division());
  ASSERT_EQ(std::chrono::microseconds(500000), stm.sampling_period());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(81920000u, autd3::link::Audit::stm_frequency_division(autd, dev.idx()));
  }

  ASSERT_EQ(std::nullopt, stm.start_idx());
  ASSERT_EQ(std::nullopt, stm.finish_idx());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(-1, autd3::link::Audit::stm_start_idx(autd, dev.idx()));
    ASSERT_EQ(-1, autd3::link::Audit::stm_finish_idx(autd, dev.idx()));
  }

  stm.with_start_idx(0);
  ASSERT_TRUE(autd.send(stm));
  ASSERT_EQ(0, stm.start_idx());
  ASSERT_EQ(std::nullopt, stm.finish_idx());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(0, autd3::link::Audit::stm_start_idx(autd, dev.idx()));
    ASSERT_EQ(-1, autd3::link::Audit::stm_finish_idx(autd, dev.idx()));
  }

  stm.with_start_idx(std::nullopt).with_finish_idx(0);
  ASSERT_TRUE(autd.send(stm));
  ASSERT_EQ(std::nullopt, stm.start_idx());
  ASSERT_EQ(0, stm.finish_idx());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(-1, autd3::link::Audit::stm_start_idx(autd, dev.idx()));
    ASSERT_EQ(0, autd3::link::Audit::stm_finish_idx(autd, dev.idx()));
  }

  stm = autd3::internal::GainSTM::with_sampling_frequency_division(512).add_gain(autd3::gain::Uniform(1)).add_gain(autd3::gain::Uniform(0.5));
  ASSERT_TRUE(autd.send(stm));
  ASSERT_EQ(20000.0, stm.frequency());
  ASSERT_EQ(2 * 20000.0, stm.sampling_frequency());
  ASSERT_EQ(512u, stm.sampling_frequency_division());
  ASSERT_EQ(std::chrono::microseconds(25), stm.sampling_period());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(4096u, autd3::link::Audit::stm_frequency_division(autd, dev.idx()));
  }

  stm = autd3::internal::GainSTM::with_sampling_frequency(20e3).add_gain(autd3::gain::Uniform(1)).add_gain(autd3::gain::Uniform(0.5));
  ASSERT_TRUE(autd.send(stm));
  ASSERT_EQ(10000, stm.frequency());
  ASSERT_EQ(2 * 10000, stm.sampling_frequency());
  ASSERT_EQ(1024u, stm.sampling_frequency_division());
  ASSERT_EQ(std::chrono::microseconds(50), stm.sampling_period());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(4096u * 2, autd3::link::Audit::stm_frequency_division(autd, dev.idx()));
  }

  stm = autd3::internal::GainSTM::with_sampling_period(std::chrono::microseconds(25))
            .add_gain(autd3::gain::Uniform(1))
            .add_gain(autd3::gain::Uniform(0.5));
  ASSERT_TRUE(autd.send(stm));
  ASSERT_EQ(20000.0, stm.frequency());
  ASSERT_EQ(2 * 20000.0, stm.sampling_frequency());
  ASSERT_EQ(512u, stm.sampling_frequency_division());
  ASSERT_EQ(std::chrono::microseconds(25), stm.sampling_period());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(4096u, autd3::link::Audit::stm_frequency_division(autd, dev.idx()));
  }

  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(2u, autd3::link::Audit::stm_cycle(autd, dev.idx()));
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 2048; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));

    std::tie(duties, phases) = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 1);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 683; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }

  stm.with_mode(autd3::internal::native_methods::GainSTMMode::PhaseFull);
  ASSERT_TRUE(autd.send(stm));
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(2u, autd3::link::Audit::stm_cycle(autd, dev.idx()));
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 2048; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));

    std::tie(duties, phases) = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 1);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 2048; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }

  stm.with_mode(autd3::internal::native_methods::GainSTMMode::PhaseHalf);
  ASSERT_THROW(autd.send(stm), autd3::internal::AUTDException);
}

TEST(STMTest, GainSTMAdvancedPhase) {
  auto autd = autd3::internal::Controller::builder()
                  .advanced_phase()
                  .add_device(autd3::internal::AUTD3(autd3::internal::Vector3::Zero(), autd3::internal::Vector3::Zero()))
                  .add_device(autd3::internal::AUTD3(autd3::internal::Vector3::Zero(), autd3::internal::Quaternion::Identity()))
                  .open_with(autd3::link::Audit());

  constexpr double radius = 30.0;
  constexpr int size = 2;
  autd3::internal::Vector3 center = autd.geometry().center() + autd3::internal::Vector3(0, 0, 150);
  auto stm = autd3::internal::GainSTM(1).add_gains_from_iter(std::views::iota(0) | std::views::take(size) | std::views::transform([&](auto i) {
                                                               const double theta = 2 * autd3::internal::pi * i / size;
                                                               return autd3::gain::Focus(
                                                                   center + autd3::internal::Vector3(radius * cos(theta), radius * sin(theta), 0));
                                                             }));
  ASSERT_TRUE(autd.send(stm));
  for (const auto& dev : autd.geometry()) {
    ASSERT_TRUE(autd3::link::Audit::is_stm_gain_mode(autd, dev.idx()));
  }

  ASSERT_EQ(1, stm.frequency());
  ASSERT_EQ(2, stm.sampling_frequency());
  ASSERT_EQ(10240000u, stm.sampling_frequency_division());
  ASSERT_EQ(std::chrono::microseconds(500000), stm.sampling_period());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(81920000u, autd3::link::Audit::stm_frequency_division(autd, dev.idx()));
  }

  ASSERT_EQ(std::nullopt, stm.start_idx());
  ASSERT_EQ(std::nullopt, stm.finish_idx());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(-1, autd3::link::Audit::stm_start_idx(autd, dev.idx()));
    ASSERT_EQ(-1, autd3::link::Audit::stm_finish_idx(autd, dev.idx()));
  }

  stm.with_start_idx(0);
  ASSERT_TRUE(autd.send(stm));
  ASSERT_EQ(0, stm.start_idx());
  ASSERT_EQ(std::nullopt, stm.finish_idx());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(0, autd3::link::Audit::stm_start_idx(autd, dev.idx()));
    ASSERT_EQ(-1, autd3::link::Audit::stm_finish_idx(autd, dev.idx()));
  }

  stm.with_start_idx(std::nullopt).with_finish_idx(0);
  ASSERT_TRUE(autd.send(stm));
  ASSERT_EQ(std::nullopt, stm.start_idx());
  ASSERT_EQ(0, stm.finish_idx());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(-1, autd3::link::Audit::stm_start_idx(autd, dev.idx()));
    ASSERT_EQ(0, autd3::link::Audit::stm_finish_idx(autd, dev.idx()));
  }

  stm = autd3::internal::GainSTM::with_sampling_frequency_division(512).add_gain(autd3::gain::Uniform(1)).add_gain(autd3::gain::Uniform(0.5));
  ASSERT_TRUE(autd.send(stm));
  ASSERT_EQ(20000.0, stm.frequency());
  ASSERT_EQ(2 * 20000.0, stm.sampling_frequency());
  ASSERT_EQ(512u, stm.sampling_frequency_division());
  ASSERT_EQ(std::chrono::microseconds(25), stm.sampling_period());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(4096u, autd3::link::Audit::stm_frequency_division(autd, dev.idx()));
  }

  stm = autd3::internal::GainSTM::with_sampling_frequency(20e3).add_gain(autd3::gain::Uniform(1)).add_gain(autd3::gain::Uniform(0.5));
  ASSERT_TRUE(autd.send(stm));
  ASSERT_EQ(10000, stm.frequency());
  ASSERT_EQ(2 * 10000, stm.sampling_frequency());
  ASSERT_EQ(1024u, stm.sampling_frequency_division());
  ASSERT_EQ(std::chrono::microseconds(50), stm.sampling_period());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(4096u * 2, autd3::link::Audit::stm_frequency_division(autd, dev.idx()));
  }

  stm = autd3::internal::GainSTM::with_sampling_period(std::chrono::microseconds(25))
            .add_gain(autd3::gain::Uniform(1))
            .add_gain(autd3::gain::Uniform(0.5));
  ASSERT_TRUE(autd.send(stm));
  ASSERT_EQ(20000.0, stm.frequency());
  ASSERT_EQ(2 * 20000.0, stm.sampling_frequency());
  ASSERT_EQ(512u, stm.sampling_frequency_division());
  ASSERT_EQ(std::chrono::microseconds(25), stm.sampling_period());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(4096u, autd3::link::Audit::stm_frequency_division(autd, dev.idx()));
  }

  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(2u, autd3::link::Audit::stm_cycle(autd, dev.idx()));
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 2048; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));

    std::tie(duties, phases) = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 1);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 2048; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }

  stm.with_mode(autd3::internal::native_methods::GainSTMMode::PhaseFull);
  ASSERT_TRUE(autd.send(stm));
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(2u, autd3::link::Audit::stm_cycle(autd, dev.idx()));
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 2048; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));

    std::tie(duties, phases) = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 1);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 2048; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }

  stm.with_mode(autd3::internal::native_methods::GainSTMMode::PhaseHalf);
  ASSERT_THROW(autd.send(stm), autd3::internal::AUTDException);
}