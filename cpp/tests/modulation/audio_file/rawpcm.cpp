// File: rawpcm.cpp
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

TEST(Modulation, RawPCM) {
  auto autd = create_controller();

  const std::filesystem::path path = std::filesystem::path(AUTD3_RESOURCE_PATH).append("sin150.dat");
  ASSERT_TRUE(autd.send(autd3::modulation::audio_file::RawPCM(path, 4000)));

  for (auto& dev : autd.geometry()) {
    auto mod = autd.link<autd3::link::Audit>().modulation(dev.idx());
    std::vector<uint8_t> mod_expect{107, 131, 157, 184, 209, 234, 255, 219, 191, 166, 140, 115, 92,  70,  51,  33,  19,  8,   2,   0,
                                    2,   8,   19,  33,  51,  70,  92,  115, 140, 166, 191, 219, 255, 234, 209, 184, 157, 131, 107, 85,
                                    64,  45,  28,  15,  6,   1,   0,   3,   12,  23,  39,  57,  77,  99,  123, 148, 174, 200, 226, 255,
                                    226, 200, 174, 148, 123, 99,  77,  57,  39,  23,  12,  3,   0,   1,   6,   15,  28,  45,  64,  85};
    ASSERT_TRUE(std::ranges::equal(mod, mod_expect));
    ASSERT_EQ(40960, autd.link<autd3::link::Audit>().modulation_frequency_division(dev.idx()));
  }
}
