// File: fpga_flag.cpp
// Project: fpga
// Created Date: 30/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 30/11/2022
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

#include "autd3/driver/common/fpga/fpga_flag.hpp"

using autd3::driver::FPGAControlFlags;

TEST(DriverCommonFPGAControlFlags, FPGAControlFlagsTest) {
  FPGAControlFlags flag(FPGAControlFlags::NONE);
  ASSERT_EQ(flag, FPGAControlFlags::NONE);

  flag.set(FPGAControlFlags::LEGACY_MODE);
  ASSERT_TRUE(flag.contains(FPGAControlFlags::LEGACY_MODE));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::FORCE_FAN));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::READS_FPGA_INFO));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STM_GAIN_MODE));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STM_MODE));

  flag.set(FPGAControlFlags::STM_MODE);
  ASSERT_TRUE(flag.contains(FPGAControlFlags::LEGACY_MODE));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::STM_MODE));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::FORCE_FAN));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::READS_FPGA_INFO));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STM_GAIN_MODE));

  flag.set(FPGAControlFlags::FORCE_FAN);
  ASSERT_TRUE(flag.contains(FPGAControlFlags::LEGACY_MODE));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::STM_MODE));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::FORCE_FAN));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::READS_FPGA_INFO));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STM_GAIN_MODE));

  flag.set(FPGAControlFlags::READS_FPGA_INFO);
  ASSERT_TRUE(flag.contains(FPGAControlFlags::LEGACY_MODE));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::STM_MODE));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::FORCE_FAN));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::READS_FPGA_INFO));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STM_GAIN_MODE));

  flag.set(FPGAControlFlags::STM_GAIN_MODE);
  ASSERT_TRUE(flag.contains(FPGAControlFlags::LEGACY_MODE));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::STM_MODE));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::FORCE_FAN));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::READS_FPGA_INFO));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::STM_GAIN_MODE));

  flag.set(FPGAControlFlags::LEGACY_MODE);
  flag.set(FPGAControlFlags::STM_MODE);
  flag.set(FPGAControlFlags::FORCE_FAN);
  flag.set(FPGAControlFlags::READS_FPGA_INFO);
  flag.set(FPGAControlFlags::STM_GAIN_MODE);
  ASSERT_TRUE(flag.contains(FPGAControlFlags::LEGACY_MODE));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::STM_MODE));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::FORCE_FAN));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::READS_FPGA_INFO));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::STM_GAIN_MODE));

  flag.remove(FPGAControlFlags::LEGACY_MODE);
  ASSERT_FALSE(flag.contains(FPGAControlFlags::LEGACY_MODE));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::STM_MODE));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::FORCE_FAN));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::READS_FPGA_INFO));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::STM_GAIN_MODE));

  flag.remove(FPGAControlFlags::STM_MODE);
  ASSERT_FALSE(flag.contains(FPGAControlFlags::LEGACY_MODE));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STM_MODE));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::FORCE_FAN));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::READS_FPGA_INFO));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::STM_GAIN_MODE));

  flag.remove(FPGAControlFlags::FORCE_FAN);
  ASSERT_FALSE(flag.contains(FPGAControlFlags::LEGACY_MODE));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STM_MODE));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::FORCE_FAN));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::READS_FPGA_INFO));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::STM_GAIN_MODE));

  flag.remove(FPGAControlFlags::READS_FPGA_INFO);
  ASSERT_FALSE(flag.contains(FPGAControlFlags::LEGACY_MODE));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STM_MODE));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::FORCE_FAN));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::READS_FPGA_INFO));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::STM_GAIN_MODE));

  flag.remove(FPGAControlFlags::STM_GAIN_MODE);
  ASSERT_FALSE(flag.contains(FPGAControlFlags::LEGACY_MODE));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STM_MODE));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::FORCE_FAN));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::READS_FPGA_INFO));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STM_GAIN_MODE));
  ASSERT_EQ(flag, FPGAControlFlags::NONE);

  flag.remove(FPGAControlFlags::LEGACY_MODE);
  flag.remove(FPGAControlFlags::STM_MODE);
  flag.remove(FPGAControlFlags::FORCE_FAN);
  flag.remove(FPGAControlFlags::READS_FPGA_INFO);
  flag.remove(FPGAControlFlags::STM_GAIN_MODE);
  ASSERT_FALSE(flag.contains(FPGAControlFlags::LEGACY_MODE));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STM_MODE));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::FORCE_FAN));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::READS_FPGA_INFO));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STM_GAIN_MODE));
  ASSERT_EQ(flag, FPGAControlFlags::NONE);
}
