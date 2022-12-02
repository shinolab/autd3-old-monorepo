// File: cpu_flag.cpp
// Project: cpu
// Created Date: 01/12/2022
// Author: Shun Suzuki
// -----
// Last Modified: 01/12/2022
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

#include <random>

#include "autd3/driver/common/cpu/cpu_flag.hpp"

TEST(DriverCommonCPUTest, CPUControlFlags) {
	using autd3::driver::CPUControlFlags;

  CPUControlFlags flag(CPUControlFlags::NONE);

  ASSERT_EQ(flag, CPUControlFlags::NONE);

  flag.set(CPUControlFlags::MOD);

  ASSERT_TRUE(flag != CPUControlFlags::NONE);
  ASSERT_EQ(flag, CPUControlFlags::MOD);

  flag.set(CPUControlFlags::MOD_BEGIN);
  flag.remove(CPUControlFlags::MOD);

  ASSERT_TRUE(flag != CPUControlFlags::MOD);
  ASSERT_EQ(flag, CPUControlFlags::MOD_BEGIN);
}
