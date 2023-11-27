// File: amp.cpp
// Project: holo
// Created Date: 25/11/2023
// Author: Shun Suzuki
// -----
// Last Modified: 25/11/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include <autd3/gain/holo/amplitude.hpp>
#include <numbers>

TEST(Gain_Holo, AmplitudeDB) {
  const auto amp = 121.5 * autd3::gain::holo::dB;
  ASSERT_EQ(amp.as_spl(), 121.5);
  ASSERT_EQ(amp.as_pascal(), 23.77004454874038);
}

TEST(Gain_Holo, AmplitudeSPL) {
  const auto amp = 23.77004454874038 * autd3::gain::holo::Pascal;
  ASSERT_EQ(amp.as_spl(), 121.5);
  ASSERT_EQ(amp.as_pascal(), 23.77004454874038);
}
