// File: audio_file_test.cpp
// Project: audio_file
// Created Date: 30/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 30/05/2022
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

#include <filesystem>

namespace fs = std::filesystem;

#include "autd3/modulation/audio_file.hpp"

TEST(RawPCMTest, default) {
  const fs::path path = fs::path(std::string(AUTD3_RESOURCE_PATH)).append(std::string("sin150.dat"));

  auto m = autd3::modulation::RawPCM(path.string(), 4e3, 40960);
  m.calc();

  const uint8_t expects[80] = {108, 132, 157, 184, 209, 235, 255, 220, 192, 166, 140, 116, 93, 71, 51, 34, 20, 9, 3, 0, 3, 9,  20, 34, 51, 71, 93,
                               116, 140, 166, 192, 220, 255, 235, 209, 184, 157, 132, 108, 85, 64, 45, 29, 15, 6, 1, 0, 4, 12, 24, 39, 57, 77, 100,
                               124, 148, 174, 201, 226, 255, 226, 201, 174, 148, 124, 100, 77, 57, 39, 24, 12, 4, 0, 1, 6, 15, 29, 45, 64, 85};

  ASSERT_EQ(m.buffer().size(), 80);
  for (size_t i = 0; i < 80; i++) ASSERT_EQ(m.buffer()[i], expects[i]);
}

TEST(WavTest, default) {
  const fs::path path = fs::path(std::string(AUTD3_RESOURCE_PATH)).append(std::string("sin150.wav"));

  auto m = autd3::modulation::Wav(path.string(), 40960);
  m.calc();

  const uint8_t expects[80] = {85,  108, 132, 157, 183, 209, 235, 241, 217, 192, 165, 140, 116, 92, 72, 52, 35, 20, 10, 3, 1, 3, 10, 20, 35, 52, 72,
                               92,  116, 140, 165, 192, 217, 241, 235, 209, 183, 157, 132, 108, 85, 65, 46, 29, 17, 7,  2, 1, 5, 13, 25, 40, 58, 79,
                               100, 124, 148, 174, 201, 226, 255, 226, 201, 174, 148, 124, 100, 79, 58, 40, 25, 13, 5,  1, 2, 7, 17, 29, 46, 65};

  ASSERT_EQ(m.buffer().size(), 80);
  for (size_t i = 0; i < 80; i++) ASSERT_EQ(m.buffer()[i], expects[i]);
}
