// File: stm.cpp
// Project: internal
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 02/12/2023
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
  auto stm = autd3::internal::FocusSTM(1).add_foci_from_iter(std::views::iota(0) | std::views::take(size) | std::views::transform([&](auto i) {
                                                               const double theta = 2 * autd3::internal::pi * i / size;
                                                               return autd3::internal::ControlPoint{
                                                                   center + autd3::internal::Vector3(radius * cos(theta), radius * sin(theta), 0),
                                                                   autd3::internal::EmitIntensity::maximum()};
                                                             }));
  ASSERT_TRUE(autd.send_async(stm).get());
  for (const auto& dev : autd.geometry()) {
    ASSERT_FALSE(autd.link().is_stm_gain_mode(dev.idx()));
  }

  ASSERT_EQ(1, stm.frequency());
  ASSERT_EQ(2, stm.sampling_config().frequency());
  ASSERT_EQ(10240000u, stm.sampling_config().frequency_division());
  ASSERT_EQ(std::chrono::microseconds(500000), stm.sampling_config().period());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(10240000u, autd.link().stm_frequency_division(dev.idx()));
  }

  ASSERT_EQ(std::nullopt, stm.start_idx());
  ASSERT_EQ(std::nullopt, stm.finish_idx());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(-1, autd.link().stm_start_idx(dev.idx()));
    ASSERT_EQ(-1, autd.link().stm_finish_idx(dev.idx()));
  }

  stm.with_start_idx(0);
  ASSERT_TRUE(autd.send_async(stm).get());
  ASSERT_EQ(0, stm.start_idx());
  ASSERT_EQ(std::nullopt, stm.finish_idx());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(0, autd.link().stm_start_idx(dev.idx()));
    ASSERT_EQ(-1, autd.link().stm_finish_idx(dev.idx()));
  }

  stm.with_start_idx(std::nullopt);
  stm.with_finish_idx(0);
  ASSERT_TRUE(autd.send_async(stm).get());
  ASSERT_EQ(std::nullopt, stm.start_idx());
  ASSERT_EQ(0, stm.finish_idx());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(-1, autd.link().stm_start_idx(dev.idx()));
    ASSERT_EQ(0, autd.link().stm_finish_idx(dev.idx()));
  }

  stm = autd3::internal::FocusSTM::from_sampling_config(autd3::internal::SamplingConfiguration::from_frequency_division(512))
            .add_focus(center)
            .add_focus(center);
  ASSERT_TRUE(autd.send_async(stm).get());
  ASSERT_EQ(20000.0, stm.frequency());
  ASSERT_EQ(2 * 20000.0, stm.sampling_config().frequency());
  ASSERT_EQ(512u, stm.sampling_config().frequency_division());
  ASSERT_EQ(std::chrono::microseconds(25), stm.sampling_config().period());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(512u, autd.link().stm_frequency_division(dev.idx()));
  }

  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(2u, autd.link().stm_cycle(dev.idx()));
    auto [intensities, phases] = autd.link().intensities_and_phases(dev.idx(), 0);
    ASSERT_TRUE(std::ranges::any_of(intensities, [](auto d) { return d != 0; }));
    ASSERT_TRUE(std::ranges::any_of(phases, [](auto p) { return p != 0; }));

    std::tie(intensities, phases) = autd.link().intensities_and_phases(dev.idx(), 1);
    ASSERT_TRUE(std::ranges::any_of(intensities, [](auto d) { return d != 0; }));
    ASSERT_TRUE(std::ranges::any_of(phases, [](auto p) { return p != 0; }));
  }
}

TEST(STMTest, GainSTM) {
  auto autd = autd3::internal::ControllerBuilder()
                  .add_device(autd3::internal::geometry::AUTD3(autd3::internal::Vector3::Zero()))
                  .add_device(autd3::internal::geometry::AUTD3(autd3::internal::Vector3::Zero()))
                  .open_with_async(autd3::link::Audit::builder())
                  .get();

  constexpr double radius = 30.0;
  constexpr int size = 2;
  autd3::internal::Vector3 center = autd.geometry().center() + autd3::internal::Vector3(0, 0, 150);
  auto stm = autd3::internal::GainSTM(1).add_gains_from_iter(std::views::iota(0) | std::views::take(size) | std::views::transform([&](auto i) {
                                                               const double theta = 2 * autd3::internal::pi * i / size;
                                                               return autd3::gain::Focus(
                                                                   center + autd3::internal::Vector3(radius * cos(theta), radius * sin(theta), 0));
                                                             }));
  ASSERT_TRUE(autd.send_async(stm).get());
  for (const auto& dev : autd.geometry()) {
    ASSERT_TRUE(autd.link().is_stm_gain_mode(dev.idx()));
  }

  ASSERT_EQ(1, stm.frequency());
  ASSERT_EQ(2, stm.sampling_config().frequency());
  ASSERT_EQ(10240000u, stm.sampling_config().frequency_division());
  ASSERT_EQ(std::chrono::microseconds(500000), stm.sampling_config().period());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(10240000u, autd.link().stm_frequency_division(dev.idx()));
  }

  ASSERT_EQ(std::nullopt, stm.start_idx());
  ASSERT_EQ(std::nullopt, stm.finish_idx());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(-1, autd.link().stm_start_idx(dev.idx()));
    ASSERT_EQ(-1, autd.link().stm_finish_idx(dev.idx()));
  }

  stm.with_start_idx(0);
  ASSERT_TRUE(autd.send_async(stm).get());
  ASSERT_EQ(0, stm.start_idx());
  ASSERT_EQ(std::nullopt, stm.finish_idx());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(0, autd.link().stm_start_idx(dev.idx()));
    ASSERT_EQ(-1, autd.link().stm_finish_idx(dev.idx()));
  }

  stm.with_start_idx(std::nullopt);
  stm.with_finish_idx(0);
  ASSERT_TRUE(autd.send_async(stm).get());
  ASSERT_EQ(std::nullopt, stm.start_idx());
  ASSERT_EQ(0, stm.finish_idx());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(-1, autd.link().stm_start_idx(dev.idx()));
    ASSERT_EQ(0, autd.link().stm_finish_idx(dev.idx()));
  }

  stm = autd3::internal::GainSTM::from_sampling_config(autd3::internal::SamplingConfiguration::from_frequency_division(512))
            .add_gain(autd3::gain::Uniform(0x80))
            .add_gain(autd3::gain::Uniform(0x80));
  ASSERT_TRUE(autd.send_async(stm).get());
  ASSERT_EQ(20000.0, stm.frequency());
  ASSERT_EQ(2 * 20000.0, stm.sampling_config().frequency());
  ASSERT_EQ(512u, stm.sampling_config().frequency_division());
  ASSERT_EQ(std::chrono::microseconds(25), stm.sampling_config().period());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(512u, autd.link().stm_frequency_division(dev.idx()));
  }

  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(2u, autd.link().stm_cycle(dev.idx()));
    auto [intensities, phases] = autd.link().intensities_and_phases(dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0x80; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));

    std::tie(intensities, phases) = autd.link().intensities_and_phases(dev.idx(), 1);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0x80; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }

  stm.with_mode(autd3::internal::native_methods::GainSTMMode::PhaseFull);
  ASSERT_TRUE(autd.send_async(stm).get());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(2u, autd.link().stm_cycle(dev.idx()));
    auto [intensities, phases] = autd.link().intensities_and_phases(dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0xFF; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));

    std::tie(intensities, phases) = autd.link().intensities_and_phases(dev.idx(), 1);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0xFF; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }

  stm.with_mode(autd3::internal::native_methods::GainSTMMode::PhaseHalf);
  ASSERT_TRUE(autd.send_async(stm).get());
  for (const auto& dev : autd.geometry()) {
    ASSERT_EQ(2u, autd.link().stm_cycle(dev.idx()));
    auto [intensities, phases] = autd.link().intensities_and_phases(dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0xFF; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));

    std::tie(intensities, phases) = autd.link().intensities_and_phases(dev.idx(), 1);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0xFF; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }
}
