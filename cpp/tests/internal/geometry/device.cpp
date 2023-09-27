// File: device.cpp
// Project: geometry
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 27/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include <autd3/internal/datagram.hpp>
#include <autd3/internal/geometry/device.hpp>
#include <ranges>

#include "utils.hpp"

TEST(Internal_Geometry, DeviceIdx) {
  auto autd = create_controller();
  ASSERT_EQ(autd.geometry()[0].idx(), 0);
  ASSERT_EQ(autd.geometry()[1].idx(), 1);
}

TEST(Internal_Geometry, DeviceSoundSpeed) {
  for (auto autd = create_controller(); auto& dev : autd.geometry()) {
    ASSERT_EQ(dev.sound_speed(), 340e3);
    dev.set_sound_speed(350e3);
    ASSERT_EQ(dev.sound_speed(), 350e3);
  }
}

TEST(Internal_Geometry, DeviceSoundSpeedFromTemp) {
  for (auto autd = create_controller(); auto& dev : autd.geometry()) {
    dev.set_sound_speed_from_temp(15);
    ASSERT_EQ(dev.sound_speed(), 340.2952640537549e3);
  }
}

TEST(Internal_Geometry, DeviceAttenuation) {
  for (auto autd = create_controller(); auto& dev : autd.geometry()) {
    ASSERT_EQ(dev.attenuation(), 0);
    dev.set_attenuation(1);
    ASSERT_EQ(dev.attenuation(), 1);
  }
}

TEST(Internal_Geometry, DeviceNumTransducers) {
  for (auto autd = create_controller(); auto& dev : autd.geometry()) {
    ASSERT_EQ(dev.num_transducers(), 249);
  }
}

TEST(Internal_Geometry, DeviceCenter) {
  for (auto autd = create_controller(); auto& dev : autd.geometry()) {
    ASSERT_EQ(dev.center(), autd3::internal::Vector3(86.62522088353406, 66.71325301204821, 0));
  }
}

TEST(Internal_Geometry, DeviceForceFan) {
  auto autd = create_controller();
  for (auto& dev : autd.geometry()) ASSERT_EQ(autd3::link::Audit::fpga_flags(autd, dev.idx()), 0);

  autd.geometry()[0].force_fan(true);
  autd.geometry()[1].force_fan(false);
  autd.send(autd3::internal::UpdateFlags());
  ASSERT_EQ(autd3::link::Audit::fpga_flags(autd, 0), 1);
  ASSERT_EQ(autd3::link::Audit::fpga_flags(autd, 1), 0);

  autd.geometry()[0].force_fan(false);
  autd.geometry()[1].force_fan(true);
  autd.send(autd3::internal::UpdateFlags());
  ASSERT_EQ(autd3::link::Audit::fpga_flags(autd, 0), 0);
  ASSERT_EQ(autd3::link::Audit::fpga_flags(autd, 1), 1);
}

TEST(Internal_Geometry, DeviceTranslate) {
  for (auto autd = create_controller(); const auto& dev : autd.geometry()) {
    auto original_pos_view = dev.transducers() | std::views::transform([](const auto& tr) { return tr.position(); });
    std::vector original_pos(original_pos_view.begin(), original_pos_view.end());
    autd3::internal::Vector3 t(1, 2, 3);
    dev.translate(t);
    std::ranges::for_each(dev.transducers(), [&t, &original_pos](auto& tr) { ASSERT_EQ(tr.position(), original_pos[tr.local_idx()] + t); });
  }
}

TEST(Internal_Geometry, DeviceRotate) {
  for (auto autd = create_controller(); const auto& dev : autd.geometry()) {
    autd3::internal::Quaternion r(0.7071067811865476, 0, 0, 0.7071067811865476);
    dev.rotate(r);
    std::ranges::for_each(dev.transducers(), [&r](auto& tr) { ASSERT_EQ(tr.rotation(), r); });
  }
}

TEST(Internal_Geometry, DeviceAffine) {
  for (auto autd = create_controller(); const auto& dev : autd.geometry()) {
    auto original_pos_view = dev.transducers() | std::views::transform([](const auto& tr) { return tr.position(); });
    std::vector original_pos(original_pos_view.begin(), original_pos_view.end());
    autd3::internal::Vector3 t(1, 2, 3);
    autd3::internal::Quaternion r(0.7071067811865476, 0, 0, 0.7071067811865476);
    dev.affine(t, r);
    std::ranges::for_each(dev.transducers(), [&r, &t, &original_pos](auto& tr) {
      auto op = original_pos[tr.local_idx()];
      autd3::internal::Vector3 expected = autd3::internal::Vector3(-op.y(), op.x(), op.z()) + t;
      ASSERT_DOUBLE_EQ(tr.position().x(), expected.x());
      ASSERT_DOUBLE_EQ(tr.position().y(), expected.y());
      ASSERT_DOUBLE_EQ(tr.position().z(), expected.z());
      ASSERT_EQ(tr.rotation(), r);
    });
  }
}

TEST(Internal_Geometry, TransducerLocal) {
  for (auto autd = create_controller(); auto& dev : autd.geometry()) {
    std::ranges::for_each(std::views::iota(0) | std::views::take(dev.num_transducers()), [&dev](auto i) { ASSERT_EQ(dev[i].local_idx(), i); });
  }
}
