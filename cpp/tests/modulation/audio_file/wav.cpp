// File: wav.cpp
// Project: audio_file
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 05/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <gtest/gtest.h>

#include "autd3/modulation/audio_file.hpp"
#include "utils.hpp"

TEST(Modulation, Wav) {
  auto autd = create_controller();

  const std::filesystem::path path = std::filesystem::path(AUTD3_RESOURCE_PATH).append("sin150.wav");
  ASSERT_TRUE(autd.send_async(autd3::modulation::audio_file::Wav(path)).get());

  for (auto& dev : autd.geometry()) {
    auto mod = autd.link().modulation(dev.idx());
    std::vector<uint8_t> mod_expect{128, 157, 185, 210, 230, 245, 253, 254, 248, 236, 217, 194, 167, 137, 109, 80,  54,  32,  15,  5,
                                    1,   5,   15,  32,  54,  80,  109, 137, 167, 194, 217, 236, 248, 254, 253, 245, 230, 210, 185, 157,
                                    128, 99,  71,  46,  26,  11,  3,   2,   8,   20,  39,  62,  89,  119, 147, 176, 202, 224, 241, 251,
                                    255, 251, 241, 224, 202, 176, 147, 119, 89,  62,  39,  20,  8,   2,   3,   11,  26,  46,  71,  99};
    ASSERT_TRUE(std::ranges::equal(mod, mod_expect));
    ASSERT_EQ(5120, autd.link().modulation_frequency_division(dev.idx()));
  }

  ASSERT_TRUE(autd.send_async(autd3::modulation::audio_file::Wav(path).with_sampling_config(
                                  autd3::internal::SamplingConfiguration::from_frequency_division(512)))
                  .get());
  for (auto& dev : autd.geometry()) ASSERT_EQ(512, autd.link().modulation_frequency_division(dev.idx()));
}
