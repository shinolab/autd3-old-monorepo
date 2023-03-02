// File: primitive.cpp
// Project: modulation
// Created Date: 10/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 02/03/2023
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

#include "autd3/modulation/primitive.hpp"

TEST(Modulation, Static) {
  {
    auto m = autd3::modulation::Static();
    const auto buffer = m.calc();

    ASSERT_EQ(buffer.size(), 2);
    ASSERT_EQ(buffer[0], 1);
    ASSERT_EQ(buffer[1], 1);
  }

  {
    auto m = autd3::modulation::Static(0.5);
    const auto buffer = m.calc();

    ASSERT_EQ(buffer.size(), 2);
    ASSERT_EQ(buffer[0], 0.5);
    ASSERT_EQ(buffer[1], 0.5);
  }

  {
    auto m = autd3::modulation::Static(2.0);
    const auto buffer = m.calc();

    ASSERT_EQ(buffer.size(), 2);
    ASSERT_EQ(buffer[0], 2);
    ASSERT_EQ(buffer[1], 2);
  }

  {
    auto m = autd3::modulation::Static(-1.0);
    const auto buffer = m.calc();

    ASSERT_EQ(buffer.size(), 2);
    ASSERT_EQ(buffer[0], -1);
    ASSERT_EQ(buffer[1], -1);
  }
}

TEST(Modulation, Sine) {
  {
    auto m = autd3::modulation::Sine(150);
    const auto buffer = m.calc();

    const uint8_t expects[80] = {85,  108, 132, 157, 183, 210, 237, 246, 219, 192, 166, 140, 116, 92, 71, 51, 34, 19, 9, 2, 0, 2, 9,  19, 34, 51, 71,
                                 92,  116, 140, 166, 192, 219, 246, 237, 210, 183, 157, 132, 108, 85, 64, 45, 29, 16, 6, 1, 0, 4, 12, 24, 39, 57, 78,
                                 100, 124, 149, 175, 201, 228, 255, 228, 201, 175, 149, 124, 100, 78, 57, 39, 24, 12, 4, 0, 1, 6, 16, 29, 45, 64};

    ASSERT_EQ(buffer.size(), 80);
    for (size_t i = 0; i < 80; i++) ASSERT_EQ(autd3::driver::Modulation::to_duty(buffer[i]), expects[i]);
  }

  {
    auto m = autd3::modulation::Sine(150, 0.4, 0.2);  // from -0.2 to 0.2
    const auto buffer = m.calc();

    const uint8_t expects[80] = {33, 40, 48, 55, 60, 64, 66, 67, 65, 62, 57, 50, 43, 35, 28, 20, 13, 8, 4, 1, 0, 1, 4, 8,  13, 20, 28,
                                 35, 43, 50, 57, 62, 65, 67, 66, 64, 60, 55, 48, 40, 33, 25, 18, 11, 6, 2, 0, 0, 2, 5, 10, 16, 23, 30,
                                 38, 45, 52, 58, 63, 66, 67, 66, 63, 58, 52, 45, 38, 30, 23, 16, 10, 5, 2, 0, 0, 2, 6, 11, 18, 25};

    ASSERT_EQ(buffer.size(), 80);
    for (size_t i = 0; i < 80; i++) ASSERT_EQ(autd3::driver::Modulation::to_duty(buffer[i]), expects[i]);
  }

  {
    auto m = autd3::modulation::Sine(150, 2.0, 0.5);  // clamped from 0.0 to 1.0
    const auto buffer = m.calc();

    const uint8_t expects[80] = {85,  134, 206, 255, 255, 255, 255, 255, 255, 255, 255, 255, 153, 100, 57, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 57,
                                 100, 153, 255, 255, 255, 255, 255, 255, 255, 255, 255, 206, 134, 85,  44, 7,  0, 0, 0, 0, 0, 0, 0, 0, 0, 31, 71,
                                 116, 176, 255, 255, 255, 255, 255, 255, 255, 255, 255, 176, 116, 71,  31, 0,  0, 0, 0, 0, 0, 0, 0, 0, 7, 44};

    ASSERT_EQ(buffer.size(), 80);
    for (size_t i = 0; i < 80; i++) ASSERT_EQ(autd3::driver::Modulation::to_duty(buffer[i]), expects[i]);
  }

  {
    auto m = autd3::modulation::Sine(150);
    m.sampling_frequency_division() = 4096;
    const auto buffer = m.calc();

    const uint8_t expects[800] = {
        85,  87,  89,  92,  94,  96,  99,  101, 103, 106, 108, 110, 113, 115, 117, 120, 122, 125, 127, 130, 132, 135, 137, 140, 142, 145, 147, 150,
        152, 155, 157, 160, 163, 165, 168, 170, 173, 176, 178, 181, 183, 186, 189, 191, 194, 197, 199, 202, 205, 207, 210, 213, 215, 218, 221, 223,
        226, 229, 232, 234, 237, 240, 242, 245, 248, 250, 253, 254, 251, 249, 246, 243, 241, 238, 235, 232, 230, 227, 224, 222, 219, 216, 214, 211,
        208, 206, 203, 200, 198, 195, 192, 190, 187, 184, 182, 179, 176, 174, 171, 169, 166, 163, 161, 158, 156, 153, 151, 148, 146, 143, 140, 138,
        135, 133, 130, 128, 126, 123, 121, 118, 116, 113, 111, 109, 106, 104, 102, 99,  97,  95,  92,  90,  88,  86,  84,  81,  79,  77,  75,  73,
        71,  69,  67,  65,  63,  61,  59,  57,  55,  53,  51,  49,  47,  45,  44,  42,  40,  39,  37,  35,  34,  32,  31,  29,  28,  26,  25,  23,
        22,  21,  19,  18,  17,  16,  15,  14,  13,  12,  11,  10,  9,   8,   7,   6,   6,   5,   4,   4,   3,   3,   2,   2,   1,   1,   1,   1,
        0,   0,   0,   0,   0,   0,   0,   0,   0,   1,   1,   1,   1,   2,   2,   3,   3,   4,   4,   5,   6,   6,   7,   8,   9,   10,  11,  12,
        13,  14,  15,  16,  17,  18,  19,  21,  22,  23,  25,  26,  28,  29,  31,  32,  34,  35,  37,  39,  40,  42,  44,  45,  47,  49,  51,  53,
        55,  57,  59,  61,  63,  65,  67,  69,  71,  73,  75,  77,  79,  81,  84,  86,  88,  90,  92,  95,  97,  99,  102, 104, 106, 109, 111, 113,
        116, 118, 121, 123, 126, 128, 130, 133, 135, 138, 140, 143, 146, 148, 151, 153, 156, 158, 161, 163, 166, 169, 171, 174, 176, 179, 182, 184,
        187, 190, 192, 195, 198, 200, 203, 206, 208, 211, 214, 216, 219, 222, 224, 227, 230, 232, 235, 238, 241, 243, 246, 249, 251, 254, 253, 250,
        248, 245, 242, 240, 237, 234, 232, 229, 226, 223, 221, 218, 215, 213, 210, 207, 205, 202, 199, 197, 194, 191, 189, 186, 183, 181, 178, 176,
        173, 170, 168, 165, 163, 160, 157, 155, 152, 150, 147, 145, 142, 140, 137, 135, 132, 130, 127, 125, 122, 120, 117, 115, 113, 110, 108, 106,
        103, 101, 99,  96,  94,  92,  89,  87,  85,  83,  81,  78,  76,  74,  72,  70,  68,  66,  64,  62,  60,  58,  56,  54,  52,  50,  48,  47,
        45,  43,  41,  40,  38,  36,  35,  33,  32,  30,  29,  27,  26,  24,  23,  22,  20,  19,  18,  17,  16,  14,  13,  12,  11,  10,  9,   9,
        8,   7,   6,   5,   5,   4,   4,   3,   3,   2,   2,   1,   1,   1,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   1,   1,   1,   2,
        2,   2,   3,   3,   4,   5,   5,   6,   7,   7,   8,   9,   10,  11,  12,  13,  14,  15,  16,  17,  19,  20,  21,  23,  24,  25,  27,  28,
        30,  31,  33,  34,  36,  37,  39,  41,  43,  44,  46,  48,  50,  52,  53,  55,  57,  59,  61,  63,  65,  67,  69,  71,  73,  76,  78,  80,
        82,  84,  86,  89,  91,  93,  95,  98,  100, 102, 105, 107, 109, 112, 114, 117, 119, 121, 124, 126, 129, 131, 134, 136, 139, 141, 144, 146,
        149, 151, 154, 157, 159, 162, 164, 167, 170, 172, 175, 177, 180, 183, 185, 188, 191, 193, 196, 198, 201, 204, 206, 209, 212, 215, 217, 220,
        223, 225, 228, 231, 233, 236, 239, 241, 244, 247, 250, 252, 255, 252, 250, 247, 244, 241, 239, 236, 233, 231, 228, 225, 223, 220, 217, 215,
        212, 209, 206, 204, 201, 198, 196, 193, 191, 188, 185, 183, 180, 177, 175, 172, 170, 167, 164, 162, 159, 157, 154, 151, 149, 146, 144, 141,
        139, 136, 134, 131, 129, 126, 124, 121, 119, 117, 114, 112, 109, 107, 105, 102, 100, 98,  95,  93,  91,  89,  86,  84,  82,  80,  78,  76,
        73,  71,  69,  67,  65,  63,  61,  59,  57,  55,  53,  52,  50,  48,  46,  44,  43,  41,  39,  37,  36,  34,  33,  31,  30,  28,  27,  25,
        24,  23,  21,  20,  19,  17,  16,  15,  14,  13,  12,  11,  10,  9,   8,   7,   7,   6,   5,   5,   4,   3,   3,   2,   2,   2,   1,   1,
        1,   0,   0,   0,   0,   0,   0,   0,   0,   0,   0,   1,   1,   1,   2,   2,   3,   3,   4,   4,   5,   5,   6,   7,   8,   9,   9,   10,
        11,  12,  13,  14,  16,  17,  18,  19,  20,  22,  23,  24,  26,  27,  29,  30,  32,  33,  35,  36,  38,  40,  41,  43,  45,  47,  48,  50,
        52,  54,  56,  58,  60,  62,  64,  66,  68,  70,  72,  74,  76,  78,  81,  83,
    };

    ASSERT_EQ(buffer.size(), 800);
    for (size_t i = 0; i < 800; i++) ASSERT_EQ(autd3::driver::Modulation::to_duty(buffer[i]), expects[i]);
  }
}

TEST(Modulation, SineSquared) {
  {
    auto m = autd3::modulation::SineSquared(150);
    const auto buffer = m.calc();

    const uint8_t expects[80] = {128, 147, 166, 185, 204, 223, 242, 249, 230, 210, 191, 172, 153, 134, 115, 96,  77,  57,  38,  19,
                                 0,   19,  38,  57,  76,  96,  115, 134, 153, 172, 191, 210, 230, 249, 242, 223, 204, 185, 166, 147,
                                 128, 108, 89,  70,  51,  32,  13,  6,   25,  45,  64,  83,  102, 121, 140, 159, 179, 198, 217, 236,
                                 255, 236, 217, 198, 179, 159, 140, 121, 102, 83,  64,  45,  26,  6,   13,  32,  51,  70,  89,  108};

    ASSERT_EQ(buffer.size(), 80);
    for (size_t i = 0; i < 80; i++) ASSERT_NEAR(autd3::driver::Modulation::to_duty(buffer[i]), expects[i], 1);
  }

  {
    auto m = autd3::modulation::SineSquared(150, 0.4, 0.2);  // from -0.2 to 0.2
    const auto buffer = m.calc();

    const uint8_t expects[80] = {75, 84, 92, 99,  105, 109, 111, 111, 110, 106, 101, 95, 87, 78, 69, 58, 47, 36, 24, 12, 0, 12, 24, 36, 47, 58, 69,
                                 78, 87, 95, 101, 106, 110, 111, 111, 109, 105, 99,  92, 84, 75, 65, 55, 44, 32, 20, 8,  4, 16, 28, 40, 51, 62, 72,
                                 81, 90, 97, 103, 108, 110, 111, 110, 108, 103, 97,  90, 81, 72, 62, 51, 40, 28, 16, 4,  8, 20, 32, 44, 55, 65};

    ASSERT_EQ(buffer.size(), 80);
    for (size_t i = 0; i < 80; i++) ASSERT_EQ(autd3::driver::Modulation::to_duty(buffer[i]), expects[i]);
  }

  {
    auto m = autd3::modulation::SineSquared(150, 2.0, 0.5);  // clamped from 0.0 to 1.0
    const auto buffer = m.calc();

    const uint8_t expects[80] = {128, 167, 220, 255, 255, 255, 255, 255, 255, 255, 255, 255, 182, 140, 102, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0,  57, 102,
                                 140, 182, 255, 255, 255, 255, 255, 255, 255, 255, 255, 220, 167, 128, 88,  35, 0, 0, 0, 0, 0, 0, 0, 0, 0,  73, 115,
                                 153, 198, 255, 255, 255, 255, 255, 255, 255, 255, 255, 198, 153, 115, 73,  0,  0, 0, 0, 0, 0, 0, 0, 0, 35, 88};

    ASSERT_EQ(buffer.size(), 80);
    for (size_t i = 0; i < 80; i++) ASSERT_EQ(autd3::driver::Modulation::to_duty(buffer[i]), expects[i]);
  }
}

TEST(Modulation, SineLegacy) {
  {
    auto m = autd3::modulation::SineLegacy(150.0);
    const auto buffer = m.calc();

    const uint8_t expects[27] = {255, 228, 202, 176, 150, 125, 102, 80, 59, 41, 26, 13, 5, 1, 1, 5, 13, 26, 41, 59, 80, 102, 125, 150, 176, 202, 228};

    ASSERT_EQ(buffer.size(), 27);
    for (size_t i = 0; i < 27; i++) ASSERT_EQ(autd3::driver::Modulation::to_duty(buffer[i]), expects[i]);
  }

  {
    auto m = autd3::modulation::SineLegacy(150.0, 0.4, 0.2);  // from -0.2 to 0.2
    const auto buffer = m.calc();

    const uint8_t expects[27] = {67, 66, 63, 59, 53, 46, 38, 31, 23, 16, 10, 5, 2, 0, 0, 2, 5, 10, 16, 23, 31, 38, 46, 53, 59, 63, 66};

    ASSERT_EQ(buffer.size(), 27);
    for (size_t i = 0; i < 27; i++) ASSERT_EQ(autd3::driver::Modulation::to_duty(buffer[i]), expects[i]);
  }

  {
    auto m = autd3::modulation::SineLegacy(150.0, 2.0, 0.5);  // clamped from 0.0 to 1.0
    const auto buffer = m.calc();

    const uint8_t expects[27] = {255, 255, 255, 255, 255, 180, 120, 74, 35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 74, 120, 180, 255, 255, 255, 255};

    ASSERT_EQ(buffer.size(), 27);
    for (size_t i = 0; i < 27; i++) ASSERT_EQ(autd3::driver::Modulation::to_duty(buffer[i]), expects[i]);
  }
}

TEST(Modulation, Square) {
  {
    auto m = autd3::modulation::Square(150);
    const auto buffer = m.calc();

    const uint8_t expects[80] = {255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255,
                                 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255,
                                 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};

    ASSERT_EQ(buffer.size(), 80);
    for (size_t i = 0; i < 80; i++) ASSERT_EQ(autd3::driver::Modulation::to_duty(buffer[i]), expects[i]);
  }

  {
    auto m = autd3::modulation::Square(150, 0.2, 0.4);
    const auto buffer = m.calc();

    const uint8_t expects[80] = {67, 67, 67, 67, 67, 67, 67, 67, 67, 67, 67, 67, 67, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 67,
                                 67, 67, 67, 67, 67, 67, 67, 67, 67, 67, 67, 67, 67, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 67,
                                 67, 67, 67, 67, 67, 67, 67, 67, 67, 67, 67, 67, 67, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33, 33};

    ASSERT_EQ(buffer.size(), 80);
    for (size_t i = 0; i < 80; i++) ASSERT_EQ(autd3::driver::Modulation::to_duty(buffer[i]), expects[i]);
  }

  {
    auto m = autd3::modulation::Square(150, -1.0, 2.0);
    const auto buffer = m.calc();

    const uint8_t expects[80] = {255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255,
                                 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255,
                                 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};

    ASSERT_EQ(buffer.size(), 80);
    for (size_t i = 0; i < 80; i++) ASSERT_EQ(autd3::driver::Modulation::to_duty(buffer[i]), expects[i]);
  }

  {
    auto m = autd3::modulation::Square(150, 0.0, 1.0, 0.2);
    const auto buffer = m.calc();

    const uint8_t expects[80] = {255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255,
                                 255, 255, 255, 255, 0,   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255,
                                 255, 255, 255, 255, 0,   0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0};

    ASSERT_EQ(buffer.size(), 80);
    for (size_t i = 0; i < 80; i++) ASSERT_EQ(autd3::driver::Modulation::to_duty(buffer[i]), expects[i]);
  }
}
