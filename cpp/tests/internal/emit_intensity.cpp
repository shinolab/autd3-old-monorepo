// File: emit_intensity.cpp
// Project: internal
// Created Date: 25/11/2023
// Author: Shun Suzuki
// -----
// Last Modified: 01/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include <autd3/internal/emit_intensity.hpp>
#include <cmath>
#include <numbers>

TEST(Internal, EmitIntensity) {
  for (int i = 0; i <= 0xFF; i++) {
    const auto intensity = autd3::internal::EmitIntensity(static_cast<uint8_t>(i));
    ASSERT_EQ(intensity.value(), i);
  }
}

TEST(Internal, EmitIntensityWithCorrection) {
  for (int i = 0; i <= 0xFF; i++) {
    const auto intensity = autd3::internal::EmitIntensity::with_correction(static_cast<uint8_t>(i));
    ASSERT_EQ(intensity.value(), static_cast<uint8_t>(std::round(std::asin(std::pow((static_cast<double>(i) / 255.0),
                                                                                    1.0 / autd3::internal::native_methods::DEFAULT_CORRECTED_ALPHA)) /
                                                                 std::numbers::pi * 510.0)));
  }
}
