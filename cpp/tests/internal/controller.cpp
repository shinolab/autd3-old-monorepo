// File: controller.cpp
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
    ASSERT_TRUE(autd3::link::Audit::is_open(autd));

    autd.close();
    ASSERT_FALSE(autd3::link::Audit::is_open(autd));
  }

  {
    const auto autd = create_controller();
    autd3::link::Audit::break_down(autd);
    ASSERT_THROW(autd.close(), autd3::internal::AUTDException);
  }
}

TEST(Internal, ControllerSendTimeout) {
  {
    auto autd = autd3::internal::Controller::builder()
                    .add_device(autd3::internal::AUTD3(autd3::internal::Vector3::Zero(), autd3::internal::Vector3::Zero()))
                    .add_device(autd3::internal::AUTD3(autd3::internal::Vector3::Zero(), autd3::internal::Quaternion::Identity()))
                    .open_with(autd3::link::Audit().with_timeout(std::chrono::microseconds(0)));

    autd.send(autd3::internal::UpdateFlags());
    ASSERT_EQ(autd3::link::Audit::last_timeout_ns(autd), 0);

    autd.send(autd3::internal::UpdateFlags(), std::chrono::microseconds(1));
    ASSERT_EQ(autd3::link::Audit::last_timeout_ns(autd), 1000);

    autd.send(autd3::internal::UpdateFlags(), autd3::internal::UpdateFlags(), std::chrono::microseconds(2));
    ASSERT_EQ(autd3::link::Audit::last_timeout_ns(autd), 2000);

    autd.send(autd3::internal::Stop(), std::chrono::microseconds(1));
    ASSERT_EQ(autd3::link::Audit::last_timeout_ns(autd), 1000);
  }

  {
    auto autd = autd3::internal::Controller::builder()
                    .add_device(autd3::internal::AUTD3(autd3::internal::Vector3::Zero(), autd3::internal::Vector3::Zero()))
                    .add_device(autd3::internal::AUTD3(autd3::internal::Vector3::Zero(), autd3::internal::Quaternion::Identity()))
                    .open_with(autd3::link::Audit().with_timeout(std::chrono::microseconds(10)));

    autd.send(autd3::internal::UpdateFlags());
    ASSERT_EQ(autd3::link::Audit::last_timeout_ns(autd), 10000);

    autd.send(autd3::internal::UpdateFlags(), std::chrono::microseconds(1));
    ASSERT_EQ(autd3::link::Audit::last_timeout_ns(autd), 1000);

    autd.send(autd3::internal::UpdateFlags(), autd3::internal::UpdateFlags(), std::chrono::microseconds(2));
    ASSERT_EQ(autd3::link::Audit::last_timeout_ns(autd), 2000);

    autd.send(autd3::internal::Stop(), std::chrono::microseconds(1));
    ASSERT_EQ(autd3::link::Audit::last_timeout_ns(autd), 1000);
  }
}

TEST(Internal, ControllerSendSingle) {
  auto autd = create_controller();

  for (auto& dev : autd.geometry()) {
    auto m = autd3::link::Audit::modulation(autd, dev.idx());
    ASSERT_TRUE(std::ranges::all_of(m, [](auto d) { return d == 0; }));
  }

  ASSERT_TRUE(autd.send(autd3::modulation::Static()));
  for (auto& dev : autd.geometry()) {
    auto m = autd3::link::Audit::modulation(autd, dev.idx());
    ASSERT_TRUE(std::ranges::all_of(m, [](auto d) { return d == 0xFF; }));
  }

  autd3::link::Audit::down(autd);
  ASSERT_FALSE(autd.send(autd3::modulation::Static()));

  autd3::link::Audit::break_down(autd);
  ASSERT_THROW(autd.send(autd3::modulation::Static()), autd3::internal::AUTDException);
}

TEST(Internal, ControllerSendDouble) {
  auto autd = create_controller();

  for (auto& dev : autd.geometry()) {
    auto m = autd3::link::Audit::modulation(autd, dev.idx());
    ASSERT_TRUE(std::ranges::all_of(m, [](auto d) { return d == 0; }));
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 0; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }

  ASSERT_TRUE(autd.send(autd3::modulation::Static(), autd3::gain::Uniform(1)));
  for (auto& dev : autd.geometry()) {
    auto m = autd3::link::Audit::modulation(autd, dev.idx());
    ASSERT_TRUE(std::ranges::all_of(m, [](auto d) { return d == 0xFF; }));
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 2048; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }

  autd3::link::Audit::down(autd);
  ASSERT_FALSE(autd.send(autd3::modulation::Static(), autd3::gain::Uniform(1)));

  autd3::link::Audit::break_down(autd);
  ASSERT_THROW(autd.send(autd3::modulation::Static(), autd3::gain::Uniform(1)), autd3::internal::AUTDException);
}

TEST(Internal, ControllerSendSpecial) {
  auto autd = create_controller();

  ASSERT_TRUE(autd.send(autd3::gain::Uniform(1)));
  for (auto& dev : autd.geometry()) {
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 2048; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }

  ASSERT_TRUE(autd.send(autd3::internal::Stop()));
  for (auto& dev : autd.geometry()) {
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, dev.idx(), 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 0; }));
  }

  autd3::link::Audit::down(autd);
  ASSERT_FALSE(autd.send(autd3::internal::Stop()));

  autd3::link::Audit::break_down(autd);
  ASSERT_THROW(autd.send(autd3::internal::Stop()), autd3::internal::AUTDException);
}

TEST(Internal, ControllerSoftwareSTM) {
  {
    auto autd = create_controller();
    auto cnt = 0;
    autd.software_stm([&cnt](auto&, auto, auto) {
          cnt++;
          return false;
        })
        .with_timer_strategy(autd3::internal::native_methods::TimerStrategy::Sleep)
        .start(std::chrono::milliseconds(1));
    ASSERT_EQ(1, cnt);
  }

  {
    auto autd = create_controller();
    auto cnt = 0;
    autd.software_stm([&cnt](auto&, auto, auto) {
          cnt++;
          return false;
        })
        .with_timer_strategy(autd3::internal::native_methods::TimerStrategy::BusyWait)
        .start(std::chrono::milliseconds(1));
    ASSERT_EQ(1, cnt);
  }

  {
    auto autd = create_controller();
    auto cnt = 0;
    autd.software_stm([&cnt](auto&, auto, auto) {
          cnt++;
          return false;
        })
        .with_timer_strategy(autd3::internal::native_methods::TimerStrategy::NativeTimer)
        .start(std::chrono::milliseconds(1));
    ASSERT_EQ(1, cnt);
  }
}

TEST(Internal, ControllerGroup) {
  auto autd = create_controller();

  autd.group([](auto& dev) -> std::optional<size_t> { return dev.idx(); })
      .set(0, autd3::modulation::Static(), autd3::gain::Null())
      .set(1, autd3::modulation::Sine(150), autd3::gain::Uniform(1))
      .send();

  {
    const auto m = autd3::link::Audit::modulation(autd, 0);
    ASSERT_EQ(2, m.size());
    ASSERT_TRUE(std::ranges::all_of(m, [](auto d) { return d == 0xFF; }));
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, 0, 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 8; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }
  {
    const auto m = autd3::link::Audit::modulation(autd, 1);
    ASSERT_EQ(80, m.size());
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, 1, 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 2048; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }

  autd.group([](auto& dev) -> std::optional<size_t> { return dev.idx(); })
      .set(1, autd3::internal::Stop())
      .set(0, autd3::modulation::Sine(150), autd3::gain::Uniform(1))
      .send();

  {
    const auto m = autd3::link::Audit::modulation(autd, 0);
    ASSERT_EQ(80, m.size());
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, 0, 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 2048; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }
  {
    auto [duties, _] = autd3::link::Audit::duties_and_phases(autd, 1, 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 0; }));
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
      .set(0, autd3::modulation::Sine(150), autd3::gain::Uniform(0.5).with_phase(autd3::internal::pi))
      .send();

  ASSERT_FALSE(check[0]);
  ASSERT_TRUE(check[1]);

  {
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, 0, 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 0; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 0; }));
  }
  {
    const auto m = autd3::link::Audit::modulation(autd, 1);
    ASSERT_EQ(80, m.size());
    auto [duties, phases] = autd3::link::Audit::duties_and_phases(autd, 1, 0);
    ASSERT_TRUE(std::ranges::all_of(duties, [](auto d) { return d == 680; }));
    ASSERT_TRUE(std::ranges::all_of(phases, [](auto p) { return p == 2048; }));
  }
}
