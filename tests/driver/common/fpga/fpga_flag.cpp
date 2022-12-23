// File: fpga_flag.cpp
// Project: fpga
// Created Date: 30/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 23/12/2022
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
  FPGAControlFlags flag(FPGAControlFlags::None);
  ASSERT_EQ(flag, FPGAControlFlags::None);

  flag.set(FPGAControlFlags::LegacyMode);
  ASSERT_TRUE(flag.contains(FPGAControlFlags::LegacyMode));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::ForceFan));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::ReadsFPGAInfo));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STMGainMode));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STMMode));

  flag.set(FPGAControlFlags::STMMode);
  ASSERT_TRUE(flag.contains(FPGAControlFlags::LegacyMode));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::STMMode));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::ForceFan));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::ReadsFPGAInfo));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STMGainMode));

  flag.set(FPGAControlFlags::ForceFan);
  ASSERT_TRUE(flag.contains(FPGAControlFlags::LegacyMode));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::STMMode));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::ForceFan));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::ReadsFPGAInfo));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STMGainMode));

  flag.set(FPGAControlFlags::ReadsFPGAInfo);
  ASSERT_TRUE(flag.contains(FPGAControlFlags::LegacyMode));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::STMMode));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::ForceFan));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::ReadsFPGAInfo));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STMGainMode));

  flag.set(FPGAControlFlags::STMGainMode);
  ASSERT_TRUE(flag.contains(FPGAControlFlags::LegacyMode));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::STMMode));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::ForceFan));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::ReadsFPGAInfo));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::STMGainMode));

  flag.set(FPGAControlFlags::LegacyMode);
  flag.set(FPGAControlFlags::STMMode);
  flag.set(FPGAControlFlags::ForceFan);
  flag.set(FPGAControlFlags::ReadsFPGAInfo);
  flag.set(FPGAControlFlags::STMGainMode);
  ASSERT_TRUE(flag.contains(FPGAControlFlags::LegacyMode));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::STMMode));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::ForceFan));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::ReadsFPGAInfo));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::STMGainMode));

  flag.remove(FPGAControlFlags::LegacyMode);
  ASSERT_FALSE(flag.contains(FPGAControlFlags::LegacyMode));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::STMMode));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::ForceFan));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::ReadsFPGAInfo));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::STMGainMode));

  flag.remove(FPGAControlFlags::STMMode);
  ASSERT_FALSE(flag.contains(FPGAControlFlags::LegacyMode));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STMMode));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::ForceFan));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::ReadsFPGAInfo));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::STMGainMode));

  flag.remove(FPGAControlFlags::ForceFan);
  ASSERT_FALSE(flag.contains(FPGAControlFlags::LegacyMode));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STMMode));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::ForceFan));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::ReadsFPGAInfo));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::STMGainMode));

  flag.remove(FPGAControlFlags::ReadsFPGAInfo);
  ASSERT_FALSE(flag.contains(FPGAControlFlags::LegacyMode));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STMMode));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::ForceFan));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::ReadsFPGAInfo));
  ASSERT_TRUE(flag.contains(FPGAControlFlags::STMGainMode));

  flag.remove(FPGAControlFlags::STMGainMode);
  ASSERT_FALSE(flag.contains(FPGAControlFlags::LegacyMode));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STMMode));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::ForceFan));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::ReadsFPGAInfo));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STMGainMode));
  ASSERT_EQ(flag, FPGAControlFlags::None);

  flag.remove(FPGAControlFlags::LegacyMode);
  flag.remove(FPGAControlFlags::STMMode);
  flag.remove(FPGAControlFlags::ForceFan);
  flag.remove(FPGAControlFlags::ReadsFPGAInfo);
  flag.remove(FPGAControlFlags::STMGainMode);
  ASSERT_FALSE(flag.contains(FPGAControlFlags::LegacyMode));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STMMode));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::ForceFan));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::ReadsFPGAInfo));
  ASSERT_FALSE(flag.contains(FPGAControlFlags::STMGainMode));
  ASSERT_EQ(flag, FPGAControlFlags::None);
}
