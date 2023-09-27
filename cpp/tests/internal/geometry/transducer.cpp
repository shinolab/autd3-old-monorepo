// File: transducer.cpp
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

#include <autd3/internal/geometry/transducer.hpp>
#include <ranges>

#include "utils.hpp"

TEST(Internal_Geometry, TransducerLocalIdx) {
  for (auto autd = create_controller(); auto& dev : autd.geometry()) {
    std::ranges::for_each(std::views::iota(0) | std::views::take(dev.num_transducers()), [&dev](auto i) { ASSERT_EQ(dev[i].local_idx(), i); });
  }
}

TEST(Internal_Geometry, TransducerPosition) {
  auto autd = create_controller();

  ASSERT_EQ(autd.geometry()[0][0].position(), autd3::internal::Vector3(0, 0, 0));
  ASSERT_EQ(autd.geometry()[0][autd3::internal::AUTD3::NUM_TRANS_IN_UNIT - 1].position(), autd3::internal::Vector3(172.72, 132.08, 0));

  ASSERT_EQ(autd.geometry()[1][0].position(), autd3::internal::Vector3(0, 0, 0));
  ASSERT_EQ(autd.geometry()[1][autd3::internal::AUTD3::NUM_TRANS_IN_UNIT - 1].position(), autd3::internal::Vector3(172.72, 132.08, 0));
}

TEST(Internal_Geometry, TransducerRotation) {
  for (auto autd = create_controller(); auto& dev : autd.geometry()) {
    std::ranges::for_each(dev.transducers(), [](auto& tr) { ASSERT_EQ(tr.rotation(), autd3::internal::Quaternion::Identity()); });
  }
}

TEST(Internal_Geometry, TransducerDirectionX) {
  for (auto autd = create_controller(); auto& dev : autd.geometry()) {
    std::ranges::for_each(dev.transducers(), [](auto& tr) { ASSERT_EQ(tr.x_direction(), autd3::internal::Vector3::UnitX()); });
  }
}

TEST(Internal_Geometry, TransducerDirectionY) {
  for (auto autd = create_controller(); auto& dev : autd.geometry()) {
    std::ranges::for_each(dev.transducers(), [](auto& tr) { ASSERT_EQ(tr.y_direction(), autd3::internal::Vector3::UnitY()); });
  }
}

TEST(Internal_Geometry, TransducerDirectionZ) {
  for (auto autd = create_controller(); auto& dev : autd.geometry()) {
    std::ranges::for_each(dev.transducers(), [](auto& tr) { ASSERT_EQ(tr.z_direction(), autd3::internal::Vector3::UnitZ()); });
  }
}

TEST(Internal_Geometry, TransducerFrequency) {
  for (auto autd = create_controller(); auto& dev : autd.geometry()) {
    for (auto& tr : dev) {
      ASSERT_EQ(tr.frequency(), 40e3);
      tr.set_frequency(69.98718496369073e3);
      ASSERT_EQ(tr.frequency(), 69.98718496369073e3);
    }
  }
}

TEST(Internal_Geometry, TransducerCycle) {
  for (auto autd = create_controller(); auto& dev : autd.geometry()) {
    for (auto& tr : dev) {
      ASSERT_EQ(tr.cycle(), 4096);
      tr.set_cycle(3000);
      ASSERT_EQ(tr.cycle(), 3000);
    }
  }
}

TEST(Internal_Geometry, TransducerModDelay) {
  for (auto autd = create_controller(); auto& dev : autd.geometry()) {
    for (auto& tr : dev) {
      ASSERT_EQ(tr.mod_delay(), 0);
      tr.set_mod_delay(1);
      ASSERT_EQ(tr.mod_delay(), 1);
    }
  }
}

TEST(Internal_Geometry, TransducerAmpFilter) {
  for (auto autd = create_controller(); auto& dev : autd.geometry()) {
    for (auto& tr : dev) {
      ASSERT_EQ(tr.amp_filter(), 0);
      tr.set_amp_filter(-1);
      ASSERT_EQ(tr.amp_filter(), -1);
    }
  }
}

TEST(Internal_Geometry, TransducerPhaseFilter) {
  for (auto autd = create_controller(); auto& dev : autd.geometry()) {
    for (auto& tr : dev) {
      ASSERT_EQ(tr.phase_filter(), 0);
      tr.set_phase_filter(-autd3::internal::pi);
      ASSERT_EQ(tr.phase_filter(), -autd3::internal::pi);
    }
  }
}

TEST(Internal_Geometry, TransducerWavelength) {
  for (auto autd = create_controller(); auto& dev : autd.geometry()) {
    for (auto& tr : dev) {
      ASSERT_DOUBLE_EQ(tr.wavelength(340e3), 340e3 / tr.frequency());
    }
  }
}

TEST(Internal_Geometry, TransducerWavenum) {
  for (auto autd = create_controller(); auto& dev : autd.geometry()) {
    for (auto& tr : dev) {
      ASSERT_DOUBLE_EQ(tr.wavenumber(340e3), 2 * autd3::internal::pi * tr.frequency() / 340e3);
    }
  }
}
