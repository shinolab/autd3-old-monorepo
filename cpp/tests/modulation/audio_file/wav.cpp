// File: wav.cpp
// Project: audio_file
// Created Date: 26/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 09/10/2023
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
  ASSERT_TRUE(autd.send(autd3::modulation::audio_file::Wav(path)));

  for (auto& dev : autd.geometry()) {
    auto mod = autd.link<autd3::link::Audit>().modulation(dev.idx());
    std::vector<uint8_t> mod_expect{85,  107, 131, 157, 182, 209, 234, 240, 216, 191, 165, 140, 115, 92,  71,  51,  34,  20,  9,   3,
                                    0,   3,   9,   20,  34,  51,  71,  92,  115, 140, 165, 191, 216, 240, 234, 209, 182, 157, 131, 107,
                                    85,  64,  45,  29,  16,  7,   1,   1,   5,   12,  24,  39,  57,  78,  99,  123, 148, 174, 200, 226,
                                    255, 226, 200, 174, 148, 123, 99,  78,  57,  39,  24,  12,  5,   1,   1,   7,   16,  29,  45,  64};
    ASSERT_TRUE(std::ranges::equal(mod, mod_expect));
    ASSERT_EQ(40960, autd.link<autd3::link::Audit>().modulation_frequency_division(dev.idx()));
  }
}
