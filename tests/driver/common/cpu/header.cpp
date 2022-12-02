// File: datagram.cpp
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

#include "autd3/driver/common/cpu/datagram.hpp"

TEST(DriverCommonCPUTest, ModHeaderInitial) { ASSERT_EQ(sizeof(autd3::driver::ModHeaderInitial), 124); }

TEST(DriverCommonCPUTest, ModHeaderSubsequent) { ASSERT_EQ(sizeof(autd3::driver::ModHeaderSubsequent), 124); }

TEST(DriverCommonCPUTest, SilencerHeader) { ASSERT_EQ(sizeof(autd3::driver::SilencerHeader), 124); }

TEST(DriverCommonCPUTest, GlobalHeader) { ASSERT_EQ(sizeof(autd3::driver::GlobalHeader), 128); }
