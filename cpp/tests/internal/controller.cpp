// File: controller.cpp
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

#include <autd3/datagram/force_fan.hpp>
#include <autd3/gain/focus.hpp>
#include <autd3/gain/null.hpp>
#include <autd3/gain/uniform.hpp>
#include <autd3/internal/datagram.hpp>
#include <autd3/internal/special.hpp>
#include <autd3/modulation/sine.hpp>
#include <autd3/modulation/static.hpp>

#include "utils.hpp"

TEST(Internal, ControllerClose) {
  {
    const auto autd = create_controller();
    ASSERT_TRUE(autd.link().is_open());

    autd.close_async().get();
    ASSERT_FALSE(autd.link().is_open());
  }

  {
    const auto autd = create_controller();
    autd.link().break_down();
    ASSERT_THROW(autd.close_async().get(), autd3::internal::AUTDException);
  }
}

TEST(Internal, ControllerSendTimeout) {
  {
    auto autd = autd3::internal::ControllerBuilder()
                    .add_device(autd3::internal::geometry::AUTD3(autd3::internal::Vector3::Zero()))
                    .add_device(autd3::internal::geometry::AUTD3(autd3::internal::Vector3::Zero()))
                    .open_with_async(autd3::link::Audit::builder().with_timeout(std::chrono::microseconds(0)))
                    .get();

    autd.send_async(autd3::gain::Null()).get();
    ASSERT_EQ(autd.link().last_timeout_ns(), 0);

    autd.send_async(autd3::gain::Null(), std::chrono::microseconds(1)).get();
    ASSERT_EQ(autd.link().last_timeout_ns(), 1000);

    autd.send_async(autd3::gain::Null(), autd3::gain::Null(), std::chrono::microseconds(2)).get();
    ASSERT_EQ(autd.link().last_timeout_ns(), 2000);

    autd.send_async(autd3::internal::Stop(), std::chrono::microseconds(1)).get();
    ASSERT_EQ(autd.link().last_timeout_ns(), 1000);
  }

  {
    auto autd = autd3::internal::ControllerBuilder()
                    .add_device(autd3::internal::geometry::AUTD3(autd3::internal::Vector3::Zero()))
                    .add_device(autd3::internal::geometry::AUTD3(autd3::internal::Vector3::Zero()))
                    .open_with_async(autd3::link::Audit::builder().with_timeout(std::chrono::microseconds(10)))
                    .get();

    autd.send_async(autd3::gain::Null()).get();
    ASSERT_EQ(autd.link().last_timeout_ns(), 10000);

    autd.send_async(autd3::gain::Null(), std::chrono::microseconds(1)).get();
    ASSERT_EQ(autd.link().last_timeout_ns(), 1000);

    autd.send_async(autd3::gain::Null(), autd3::gain::Null(), std::chrono::microseconds(2)).get();
    ASSERT_EQ(autd.link().last_timeout_ns(), 2000);

    autd.send_async(autd3::internal::Stop(), std::chrono::microseconds(1)).get();
    ASSERT_EQ(autd.link().last_timeout_ns(), 1000);
  }
}

TEST(Internal, ControllerSendSingle) {
  auto autd = create_controller();

  for (auto& dev : autd.geometry()) {
    auto m = autd.link().modulation(dev.idx());
    ASSERT_TRUE(std::ranges::all_of(m, [](auto d) { return d == 0xFF; }));
  }

  ASSERT_TRUE(autd.send_async(autd3::modulation::Static()).get());
  for (auto& dev : autd.geometry()) {
    auto m = autd.link().modulation(dev.idx());
    ASSERT_TRUE(std::ranges::all_of(m, [](auto d) { return d == 0xFF; }));
  }

  autd.link().down();
  ASSERT_FALSE(autd.send_async(autd3::modulation::Static()).get());

  autd.link().break_down();
  ASSERT_THROW(autd.send_async(autd3::modulation::Static()).get(), autd3::internal::AUTDException);
}

TEST(Internal, ControllerSendDouble) {
  auto autd = create_controller();

  for (auto& dev : autd.geometry()) {
    auto m = autd.link().modulation(dev.idx());
    ASSERT_TRUE(std::ranges::all_of(m, [](auto d) { return d == 0xFF; }));
    auto [intensities, phases] = autd.link().intensities_and_phases(dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }

  ASSERT_TRUE(autd.send_async(autd3::modulation::Static(), autd3::gain::Uniform(0x80)).get());
  for (auto& dev : autd.geometry()) {
    auto m = autd.link().modulation(dev.idx());
    ASSERT_TRUE(std::ranges::all_of(m, [](auto d) { return d == 0xFF; }));
    auto [intensities, phases] = autd.link().intensities_and_phases(dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0x80; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }

  autd.link().down();
  ASSERT_FALSE(autd.send_async(autd3::modulation::Static(), autd3::gain::Uniform(1)).get());

  autd.link().break_down();
  ASSERT_THROW(autd.send_async(autd3::modulation::Static(), autd3::gain::Uniform(1)).get(), autd3::internal::AUTDException);
}

TEST(Internal, ControllerSendSpecial) {
  auto autd = create_controller();

  ASSERT_TRUE(autd.send_async(autd3::gain::Uniform(0x80)).get());
  for (auto& dev : autd.geometry()) {
    auto [intensities, phases] = autd.link().intensities_and_phases(dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0x80; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }

  ASSERT_TRUE(autd.send_async(autd3::internal::Stop()).get());
  for (auto& dev : autd.geometry()) {
    auto [intensities, phases] = autd.link().intensities_and_phases(dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0; }));
  }

  autd.link().down();
  ASSERT_FALSE(autd.send_async(autd3::internal::Stop()).get());

  autd.link().break_down();
  ASSERT_THROW(autd.send_async(autd3::internal::Stop()).get(), autd3::internal::AUTDException);
}

TEST(Internal, ControllerGroup) {
  auto autd = create_controller();

  autd.group([](auto& dev) -> std::optional<size_t> { return dev.idx(); })
      .set(0, autd3::modulation::Static(), autd3::gain::Null())
      .set(1, autd3::modulation::Sine(150), autd3::gain::Uniform(0x80))
      .send_async()
      .get();

  {
    const auto m = autd.link().modulation(0);
    ASSERT_EQ(2, m.size());
    ASSERT_TRUE(std::ranges::all_of(m, [](auto d) { return d == 0xFF; }));
    auto [intensities, phases] = autd.link().intensities_and_phases(0, 0);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }
  {
    const auto m = autd.link().modulation(1);
    ASSERT_EQ(80, m.size());
    auto [intensities, phases] = autd.link().intensities_and_phases(1, 0);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0x80; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }

  autd.group([](auto& dev) -> std::optional<size_t> { return dev.idx(); })
      .set(1, autd3::internal::Stop())
      .set(0, autd3::modulation::Sine(150), autd3::gain::Uniform(0x80))
      .send_async()
      .get();

  {
    const auto m = autd.link().modulation(0);
    ASSERT_EQ(80, m.size());
    auto [intensities, phases] = autd.link().intensities_and_phases(0, 0);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0x80; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }
  {
    auto [intensities, _] = autd.link().intensities_and_phases(1, 0);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0; }));
  }
}

TEST(Internal, ControllerGroupCheckOnlyForEnabled) {
  auto autd = create_controller();
  autd.geometry()[0].set_enable(false);

  std::vector check(autd.geometry().num_devices(), false);
  autd.group([&check](auto& dev) -> std::optional<int> {
        check[dev.idx()] = true;
        return 0;
      })
      .set(0, autd3::modulation::Sine(150), autd3::gain::Uniform(0x80).with_phase(autd3::internal::Phase(0x90)))
      .send_async()
      .get();

  ASSERT_FALSE(check[0]);
  ASSERT_TRUE(check[1]);

  {
    auto [intensities, phases] = autd.link().intensities_and_phases(0, 0);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }
  {
    const auto m = autd.link().modulation(1);
    ASSERT_EQ(80, m.size());
    auto [intensities, phases] = autd.link().intensities_and_phases(1, 0);
    ASSERT_TRUE(std::ranges::all_of(intensities, [](auto d) { return d == 0x80; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0x90; }));
  }
}

TEST(Internal_Geometry, DeviceForceFan) {
  auto autd = create_controller();
  for (auto& dev : autd.geometry()) ASSERT_FALSE(autd.link().is_force_fan(dev.idx()));

  autd.send_async(autd3::datagram::ConfigureForceFan([](const auto& dev) { return dev.idx() == 0; })).get();
  ASSERT_TRUE(autd.link().is_force_fan(0));
  ASSERT_FALSE(autd.link().is_force_fan(1));

  autd.send_async(autd3::datagram::ConfigureForceFan([](const auto& dev) { return dev.idx() == 1; })).get();
  ASSERT_FALSE(autd.link().is_force_fan(0));
  ASSERT_TRUE(autd.link().is_force_fan(1));
}