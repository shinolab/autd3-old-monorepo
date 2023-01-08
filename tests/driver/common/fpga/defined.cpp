// File: defined.cpp
// Project: fpga
// Created Date: 30/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 08/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 26439 26495 26812)
#endif
#include <gtest/gtest.h>
#if _MSC_VER
#pragma warning(pop)
#endif

#include "autd3/driver/fpga/defined.hpp"

using autd3::driver::Drive;
using autd3::driver::pi;

TEST(DriverCommonFPGADefined, LegacyDrive) {
  using autd3::driver::LegacyDrive;

  Drive s{};  // NOLINT
  LegacyDrive d{};

  s.phase = 0.0;
  s.amp = 0.0;
  d.set(s);
  ASSERT_EQ(d.phase, 0);
  ASSERT_EQ(d.duty, 0);

  s.phase = pi;
  s.amp = 0.5;
  d.set(s);
  ASSERT_EQ(d.phase, 128);
  ASSERT_EQ(d.duty, 85);

  s.phase = 2.0 * pi;
  s.amp = 1.0;
  d.set(s);
  ASSERT_EQ(d.phase, 0);
  ASSERT_EQ(d.duty, 255);

  s.phase = 3.0 * pi;
  s.amp = 1.5;
  d.set(s);
  ASSERT_EQ(d.phase, 128);
  ASSERT_EQ(d.duty, 255);

  s.phase = -pi;
  s.amp = -1;
  d.set(s);
  ASSERT_EQ(d.phase, 128);
  ASSERT_EQ(d.duty, 0);
}

TEST(DriverCommonFPGADefined, Phase) {
  using autd3::driver::Phase;

  Drive s{};
  Phase d{0};

  {
    constexpr uint16_t cycle = 4096;
    s.phase = 0.0;
    d.set(s, cycle);
    ASSERT_EQ(d.phase, 0);

    s.phase = pi;
    d.set(s, cycle);
    ASSERT_EQ(d.phase, 2048);

    s.phase = 2.0 * pi;
    d.set(s, cycle);
    ASSERT_EQ(d.phase, 0);

    s.phase = 3.0 * pi;
    d.set(s, cycle);
    ASSERT_EQ(d.phase, 2048);

    s.phase = -pi;
    d.set(s, cycle);
    ASSERT_EQ(d.phase, 2048);
  }

  {
    constexpr uint16_t cycle = 2000;
    s.phase = 0.0;
    d.set(s, cycle);
    ASSERT_EQ(d.phase, 0);

    s.phase = pi;
    d.set(s, cycle);
    ASSERT_EQ(d.phase, 1000);

    s.phase = 2.0 * pi;
    d.set(s, cycle);
    ASSERT_EQ(d.phase, 0);

    s.phase = 3.0 * pi;
    d.set(s, cycle);
    ASSERT_EQ(d.phase, 1000);

    s.phase = -pi;
    d.set(s, cycle);
    ASSERT_EQ(d.phase, 1000);
  }
}

TEST(DriverCommonFPGADefined, Duty) {
  using autd3::driver::Duty;

  Drive s{};
  Duty d{0};

  {
    constexpr uint16_t cycle = 4096;

    s.amp = 0.0;
    d.set(s, cycle);
    ASSERT_EQ(d.duty, 0);

    s.amp = 0.5;
    d.set(s, cycle);
    ASSERT_EQ(d.duty, 683);

    s.amp = 1.0;
    d.set(s, cycle);
    ASSERT_EQ(d.duty, 2048);

    s.amp = 1.5;
    d.set(s, cycle);
    ASSERT_EQ(d.duty, 2048);

    s.amp = -1;
    d.set(s, cycle);
    ASSERT_EQ(d.duty, 0);
  }

  {
    constexpr uint16_t cycle = 2000;

    s.amp = 0.0;
    d.set(s, cycle);
    ASSERT_EQ(d.duty, 0);

    s.amp = 0.5;
    d.set(s, cycle);
    ASSERT_EQ(d.duty, 333);

    s.amp = 1.0;
    d.set(s, cycle);
    ASSERT_EQ(d.duty, 1000);

    s.amp = 1.5;
    d.set(s, cycle);
    ASSERT_EQ(d.duty, 1000);

    s.amp = -1;
    d.set(s, cycle);
    ASSERT_EQ(d.duty, 0);
  }
}

TEST(DriverCommonFPGADefined, FPGAInfo) {
  using autd3::driver::FPGAInfo;

  FPGAInfo info(0);
  ASSERT_FALSE(info.is_thermal_assert());

  info = FPGAInfo(1);
  ASSERT_TRUE(info.is_thermal_assert());

  info = FPGAInfo(2);
  ASSERT_FALSE(info.is_thermal_assert());
}
