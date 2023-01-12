// File: datagram.cpp
// Project: cpu
// Created Date: 01/12/2022
// Author: Shun Suzuki
// -----
// Last Modified: 11/01/2023
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

#include "autd3/driver/cpu/datagram.hpp"

TEST(DriverCommonCPUTest, ModHeaderInitial) {
  ASSERT_EQ(sizeof(autd3::driver::ModHeaderInitial), 124);
  ASSERT_EQ(offsetof(autd3::driver::ModHeaderInitial, freq_div), 0);
  ASSERT_EQ(offsetof(autd3::driver::ModHeaderInitial, data), 4);
}

TEST(DriverCommonCPUTest, ModHeaderSubsequent) {
  ASSERT_EQ(sizeof(autd3::driver::ModHeaderSubsequent), 124);
  ASSERT_EQ(offsetof(autd3::driver::ModHeaderSubsequent, data), 0);
}

TEST(DriverCommonCPUTest, SilencerHeader) {
  ASSERT_EQ(sizeof(autd3::driver::SilencerHeader), 124);
  ASSERT_EQ(offsetof(autd3::driver::SilencerHeader, cycle), 0);
  ASSERT_EQ(offsetof(autd3::driver::SilencerHeader, step), 2);
}

TEST(DriverCommonCPUTest, GlobalHeader) {
  ASSERT_EQ(sizeof(autd3::driver::GlobalHeader), 128);
  ASSERT_EQ(offsetof(autd3::driver::GlobalHeader, msg_id), 0);
  ASSERT_EQ(offsetof(autd3::driver::GlobalHeader, fpga_flag), 1);
  ASSERT_EQ(offsetof(autd3::driver::GlobalHeader, cpu_flag), 2);
  ASSERT_EQ(offsetof(autd3::driver::GlobalHeader, size), 3);
  ASSERT_EQ(offsetof(autd3::driver::GlobalHeader, data), 4);
}
