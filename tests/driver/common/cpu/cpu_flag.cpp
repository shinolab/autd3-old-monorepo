// File: cpu_flag.cpp
// Project: cpu
// Created Date: 01/12/2022
// Author: Shun Suzuki
// -----
// Last Modified: 28/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/driver/cpu/cpu_flag.hpp"

#include <gtest/gtest.h>

#include <random>

TEST(DriverCommonCPUTest, CPUControlFlags) {
  using autd3::driver::BitFlags;
  using autd3::driver::CPUControlFlags;

  BitFlags flag(CPUControlFlags::None);

  ASSERT_EQ(flag, CPUControlFlags::None);

  flag.set(CPUControlFlags::Mod);

  ASSERT_TRUE(flag != CPUControlFlags::None);
  ASSERT_EQ(flag, CPUControlFlags::Mod);

  flag.set(CPUControlFlags::ModBegin);
  flag.remove(CPUControlFlags::Mod);

  ASSERT_TRUE(flag != CPUControlFlags::Mod);
  ASSERT_EQ(flag, CPUControlFlags::ModBegin);
}
