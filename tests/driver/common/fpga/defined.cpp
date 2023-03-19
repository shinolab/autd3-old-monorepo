// File: defined.cpp
// Project: fpga
// Created Date: 30/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 20/03/2023
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

  ASSERT_EQ(sizeof(LegacyDrive), 2);

  Drive s{0, 0};
  uint8_t d[2]{};

  reinterpret_cast<LegacyDrive*>(&d)->set(s);
  ASSERT_EQ(d[0], 0);
  ASSERT_EQ(d[1], 0);

  s.phase = pi;
  s.amp = 0.5;
  reinterpret_cast<LegacyDrive*>(&d)->set(s);
  ASSERT_EQ(d[0], 128);
  ASSERT_EQ(d[1], 85);

  s.phase = 2.0 * pi;
  s.amp = 1.0;
  reinterpret_cast<LegacyDrive*>(&d)->set(s);
  ASSERT_EQ(d[0], 0);
  ASSERT_EQ(d[1], 255);

  s.phase = 3.0 * pi;
  s.amp = 1.5;
  reinterpret_cast<LegacyDrive*>(&d)->set(s);
  ASSERT_EQ(d[0], 128);
  ASSERT_EQ(d[1], 255);

  s.phase = -pi;
  s.amp = -1;
  reinterpret_cast<LegacyDrive*>(&d)->set(s);
  ASSERT_EQ(d[0], 128);
  ASSERT_EQ(d[1], 0);
}

TEST(DriverCommonFPGADefined, Phase) {
  using autd3::driver::AdvancedDrivePhase;

  ASSERT_EQ(sizeof(AdvancedDrivePhase), 2);

  Drive s{0, 0};
  uint16_t d{0};

  {
    constexpr uint16_t cycle = 4096;
    s.phase = 0.0;
    reinterpret_cast<AdvancedDrivePhase*>(&d)->set(s, cycle);
    ASSERT_EQ(d, 0);

    s.phase = pi;
    reinterpret_cast<AdvancedDrivePhase*>(&d)->set(s, cycle);
    ASSERT_EQ(d, 2048);

    s.phase = 2.0 * pi;
    reinterpret_cast<AdvancedDrivePhase*>(&d)->set(s, cycle);
    ASSERT_EQ(d, 0);

    s.phase = 3.0 * pi;
    reinterpret_cast<AdvancedDrivePhase*>(&d)->set(s, cycle);
    ASSERT_EQ(d, 2048);

    s.phase = -pi;
    reinterpret_cast<AdvancedDrivePhase*>(&d)->set(s, cycle);
    ASSERT_EQ(d, 2048);
  }

  {
    constexpr uint16_t cycle = 2000;
    s.phase = 0.0;
    reinterpret_cast<AdvancedDrivePhase*>(&d)->set(s, cycle);
    ASSERT_EQ(d, 0);

    s.phase = pi;
    reinterpret_cast<AdvancedDrivePhase*>(&d)->set(s, cycle);
    ASSERT_EQ(d, 1000);

    s.phase = 2.0 * pi;
    reinterpret_cast<AdvancedDrivePhase*>(&d)->set(s, cycle);
    ASSERT_EQ(d, 0);

    s.phase = 3.0 * pi;
    reinterpret_cast<AdvancedDrivePhase*>(&d)->set(s, cycle);
    ASSERT_EQ(d, 1000);

    s.phase = -pi;
    reinterpret_cast<AdvancedDrivePhase*>(&d)->set(s, cycle);
    ASSERT_EQ(d, 1000);
  }
}

TEST(DriverCommonFPGADefined, Duty) {
  using autd3::driver::AdvancedDriveDuty;

  ASSERT_EQ(sizeof(AdvancedDriveDuty), 2);

  Drive s{0, 0};
  uint16_t d{0};

  {
    constexpr uint16_t cycle = 4096;

    s.amp = 0.0;
    reinterpret_cast<AdvancedDriveDuty*>(&d)->set(s, cycle);
    ASSERT_EQ(d, 0);

    s.amp = 0.5;
    reinterpret_cast<AdvancedDriveDuty*>(&d)->set(s, cycle);
    ASSERT_EQ(d, 683);

    s.amp = 1.0;
    reinterpret_cast<AdvancedDriveDuty*>(&d)->set(s, cycle);
    ASSERT_EQ(d, 2048);

    s.amp = 1.5;
    reinterpret_cast<AdvancedDriveDuty*>(&d)->set(s, cycle);
    ASSERT_EQ(d, 2048);

    s.amp = -1;
    reinterpret_cast<AdvancedDriveDuty*>(&d)->set(s, cycle);
    ASSERT_EQ(d, 0);
  }

  {
    constexpr uint16_t cycle = 2000;

    s.amp = 0.0;
    reinterpret_cast<AdvancedDriveDuty*>(&d)->set(s, cycle);
    ASSERT_EQ(d, 0);

    s.amp = 0.5;
    reinterpret_cast<AdvancedDriveDuty*>(&d)->set(s, cycle);
    ASSERT_EQ(d, 333);

    s.amp = 1.0;
    reinterpret_cast<AdvancedDriveDuty*>(&d)->set(s, cycle);
    ASSERT_EQ(d, 1000);

    s.amp = 1.5;
    reinterpret_cast<AdvancedDriveDuty*>(&d)->set(s, cycle);
    ASSERT_EQ(d, 1000);

    s.amp = -1;
    reinterpret_cast<AdvancedDriveDuty*>(&d)->set(s, cycle);
    ASSERT_EQ(d, 0);
  }
}

TEST(DriverCommonFPGADefined, FPGAInfo) {
  using autd3::driver::FPGAInfo;

  ASSERT_EQ(sizeof(FPGAInfo), 1);

  FPGAInfo info(0);
  ASSERT_FALSE(info.is_thermal_assert());

  info = FPGAInfo(1);
  ASSERT_TRUE(info.is_thermal_assert());

  info = FPGAInfo(2);
  ASSERT_FALSE(info.is_thermal_assert());
}
