// File: body.cpp
// Project: cpu
// Created Date: 30/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 20/03/2023
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

#include "autd3/driver/cpu/body.hpp"

using autd3::driver::FocusSTMBodyInitial;
using autd3::driver::FocusSTMBodySubsequent;
using autd3::driver::GainSTMBodyInitial;
using autd3::driver::GainSTMMode;
using autd3::driver::LegacyPhaseFull;
using autd3::driver::LegacyPhaseHalf;
using autd3::driver::pi;
using autd3::driver::STMFocus;

TEST(DriverCommonCPUTest, STMFocus) {
  ASSERT_EQ(sizeof(STMFocus), 8);

  constexpr auto max = static_cast<autd3::driver::autd3_float_t>((1 << 17) - 1) * autd3::driver::FOCUS_STM_FIXED_NUM_UNIT;
  constexpr auto min = static_cast<autd3::driver::autd3_float_t>(-(1 << 17)) * autd3::driver::FOCUS_STM_FIXED_NUM_UNIT;

  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_real_distribution dist(min, max);
  std::uniform_int_distribution dist_u8(0, 0xFF);

  const auto to = [](const uint64_t v) -> autd3::driver::autd3_float_t {
    auto b = static_cast<uint32_t>(v & 0x0003fffful);
    b = (v & 0x20000) == 0 ? b : b | 0xfffc0000u;
    const auto xi = *reinterpret_cast<int32_t*>(&b);
    return static_cast<autd3::driver::autd3_float_t>(xi) * autd3::driver::FOCUS_STM_FIXED_NUM_UNIT;
  };

  for (auto i = 0; i < 10000; i++) {
    const auto x = dist(engine);
    const auto y = dist(engine);
    const auto z = dist(engine);
    const auto shift = static_cast<uint8_t>(dist_u8(engine));

    const STMFocus focus(x, y, z, shift);

    uint64_t v = 0;
    std::memcpy(&v, &focus, sizeof(STMFocus));

    const auto xx = to(v);
    ASSERT_NEAR(xx, x, autd3::driver::FOCUS_STM_FIXED_NUM_UNIT);

    v >>= 18;
    const auto yy = to(v);
    ASSERT_NEAR(yy, y, autd3::driver::FOCUS_STM_FIXED_NUM_UNIT);

    v >>= 18;
    const auto zz = to(v);
    ASSERT_NEAR(zz, z, autd3::driver::FOCUS_STM_FIXED_NUM_UNIT);

    v >>= 18;
    const auto s = static_cast<uint8_t>(v & 0xFF);
    ASSERT_EQ(s, shift);
  }
}

TEST(DriverCommonCPUTest, FocusSTMBodyInitial) {
  ASSERT_EQ(sizeof(FocusSTMBodyInitial), 14);
  ASSERT_EQ(offsetof(FocusSTMBodyInitial, size), 0);
  ASSERT_EQ(offsetof(FocusSTMBodyInitial, freq_div), 2);
  ASSERT_EQ(offsetof(FocusSTMBodyInitial, sound_speed), 6);
  ASSERT_EQ(offsetof(FocusSTMBodyInitial, stm_start_idx), 10);
  ASSERT_EQ(offsetof(FocusSTMBodyInitial, stm_finish_idx), 12);

  constexpr size_t point_size = 1000;
  constexpr size_t size = 5 * sizeof(uint16_t) + point_size * sizeof(STMFocus);

  std::vector<uint8_t> d(size);
  auto* b = reinterpret_cast<FocusSTMBodyInitial*>(d.data());

  b->size = static_cast<uint16_t>(point_size);
  b->freq_div = 0x01234567;
  b->sound_speed = 0x89ABCDEF;

  std::vector<STMFocus> points;
  for (size_t i = 0; i < point_size; i++)
    points.emplace_back(autd3::driver::autd3_float_t{0}, autd3::driver::autd3_float_t{0}, autd3::driver::autd3_float_t{0}, 0);
  {
    auto* p = reinterpret_cast<uint8_t*>(points.data());
    for (size_t i = 0; i < point_size * sizeof(STMFocus); i++) *p++ = static_cast<uint8_t>(i);
  }
  b->set_point(points.data(), points.size());

  ASSERT_EQ(d[0], point_size & 0xFF);
  ASSERT_EQ(d[1], point_size >> 8);
  ASSERT_EQ(d[2], 0x67);
  ASSERT_EQ(d[3], 0x45);
  ASSERT_EQ(d[4], 0x23);
  ASSERT_EQ(d[5], 0x01);
  ASSERT_EQ(d[6], 0xEF);
  ASSERT_EQ(d[7], 0xCD);
  ASSERT_EQ(d[8], 0xAB);
  ASSERT_EQ(d[9], 0x89);
  for (size_t i = 0; i < point_size * sizeof(STMFocus); i++) ASSERT_EQ(d[14 + i], static_cast<uint8_t>(i));
}

TEST(DriverCommonCPUTest, FocusSTMBodySubsequent) {
  ASSERT_EQ(sizeof(FocusSTMBodySubsequent), 2);
  ASSERT_EQ(offsetof(FocusSTMBodySubsequent, size), 0);

  constexpr size_t point_size = 1000;
  constexpr size_t size = 1 * sizeof(uint16_t) + point_size * sizeof(STMFocus);

  std::vector<uint8_t> d(size);
  auto* b = reinterpret_cast<FocusSTMBodySubsequent*>(d.data());

  b->size = static_cast<uint16_t>(point_size);

  std::vector<STMFocus> points;
  for (size_t i = 0; i < point_size; i++)
    points.emplace_back(autd3::driver::autd3_float_t{0}, autd3::driver::autd3_float_t{0}, autd3::driver::autd3_float_t{0}, 0);
  {
    auto* p = reinterpret_cast<uint8_t*>(points.data());
    for (size_t i = 0; i < point_size * sizeof(STMFocus); i++) *p++ = static_cast<uint8_t>(i);
  }
  b->set_point(points.data(), points.size());

  ASSERT_EQ(d[0], point_size & 0xFF);
  ASSERT_EQ(d[1], point_size >> 8);
  for (size_t i = 0; i < point_size * sizeof(STMFocus); i++) ASSERT_EQ(d[2 + i], static_cast<uint8_t>(i));
}

TEST(DriverCommonCPUTest, LegacyPhaseFull) {
  ASSERT_EQ(sizeof(LegacyPhaseFull<0>), 2);
  ASSERT_EQ(sizeof(LegacyPhaseFull<1>), 2);

  uint8_t p[2]{};
  autd3::driver::Drive s{0, 0};

  s.phase = pi;
  reinterpret_cast<LegacyPhaseFull<0>*>(p)->set(s);
  const uint8_t expect_phase_0 = autd3::driver::LegacyDrive::to_phase(s);
  ASSERT_EQ(p[0], expect_phase_0);
  ASSERT_EQ(p[1], 0);

  s.phase = static_cast<autd3::driver::autd3_float_t>(1.5 * pi);
  reinterpret_cast<LegacyPhaseFull<1>*>(p)->set(s);
  const uint8_t expect_phase_1 = autd3::driver::LegacyDrive::to_phase(s);
  ASSERT_EQ(p[0], expect_phase_0);
  ASSERT_EQ(p[1], expect_phase_1);

  s.phase = 0;
  reinterpret_cast<LegacyPhaseFull<0>*>(p)->set(s);
  ASSERT_EQ(p[0], 0);
  ASSERT_EQ(p[1], expect_phase_1);
}

TEST(DriverCommonCPUTest, LegacyPhaseHalf) {
  ASSERT_EQ(sizeof(LegacyPhaseHalf<0>), 2);
  ASSERT_EQ(sizeof(LegacyPhaseHalf<1>), 2);
  ASSERT_EQ(sizeof(LegacyPhaseHalf<2>), 2);
  ASSERT_EQ(sizeof(LegacyPhaseHalf<3>), 2);

  uint8_t p[2]{};
  autd3::driver::Drive s{0, 0};

  s.phase = pi;
  reinterpret_cast<LegacyPhaseHalf<0>*>(p)->set(s);
  const uint8_t expect_phase_0 = autd3::driver::LegacyDrive::to_phase(s) >> 4;
  ASSERT_EQ(p[0] & 0x0F, expect_phase_0);
  ASSERT_EQ(p[0] & 0xF0, 0);
  ASSERT_EQ(p[1] & 0x0F, 0);
  ASSERT_EQ(p[1] & 0xF0, 0);

  s.phase = static_cast<autd3::driver::autd3_float_t>(1.5 * pi);
  reinterpret_cast<LegacyPhaseHalf<1>*>(p)->set(s);
  const uint8_t expect_phase_1 = autd3::driver::LegacyDrive::to_phase(s) >> 4;
  ASSERT_EQ(p[0] & 0x0F, expect_phase_0);
  ASSERT_EQ(p[0] & 0xF0, expect_phase_1 << 4);
  ASSERT_EQ(p[1] & 0x0F, 0);
  ASSERT_EQ(p[1] & 0xF0, 0);

  s.phase = static_cast<autd3::driver::autd3_float_t>(0.8 * pi);
  reinterpret_cast<LegacyPhaseHalf<2>*>(p)->set(s);
  const uint8_t expect_phase_2 = autd3::driver::LegacyDrive::to_phase(s) >> 4;
  ASSERT_EQ(p[0] & 0x0F, expect_phase_0);
  ASSERT_EQ(p[0] & 0xF0, expect_phase_1 << 4);
  ASSERT_EQ(p[1] & 0x0F, expect_phase_2);
  ASSERT_EQ(p[1] & 0xF0, 0);

  s.phase = static_cast<autd3::driver::autd3_float_t>(1.2 * pi);
  reinterpret_cast<LegacyPhaseHalf<3>*>(p)->set(s);
  const uint8_t expect_phase_3 = autd3::driver::LegacyDrive::to_phase(s) >> 4;
  ASSERT_EQ(p[0] & 0x0F, expect_phase_0);
  ASSERT_EQ(p[0] & 0xF0, expect_phase_1 << 4);
  ASSERT_EQ(p[1] & 0x0F, expect_phase_2);
  ASSERT_EQ(p[1] & 0xF0, expect_phase_3 << 4);
}

TEST(DriverCommonCPUTest, GainSTMBodyInitial) {
  ASSERT_EQ(sizeof(GainSTMBodyInitial), 12);
  ASSERT_EQ(offsetof(GainSTMBodyInitial, freq_div), 0);
  ASSERT_EQ(offsetof(GainSTMBodyInitial, mode), 4);
  ASSERT_EQ(offsetof(GainSTMBodyInitial, cycle), 6);
  ASSERT_EQ(offsetof(GainSTMBodyInitial, stm_start_idx), 8);
  ASSERT_EQ(offsetof(GainSTMBodyInitial, stm_finish_idx), 10);

  constexpr size_t size = 4 * sizeof(uint16_t);

  std::vector<uint8_t> d(size);
  auto* b = reinterpret_cast<GainSTMBodyInitial*>(d.data());

  b->freq_div = 0x01234567;
  b->mode = GainSTMMode::PhaseDutyFull;
  b->cycle = 0x89AB;

  ASSERT_EQ(d[0], 0x67);
  ASSERT_EQ(d[1], 0x45);
  ASSERT_EQ(d[2], 0x23);
  ASSERT_EQ(d[3], 0x01);
  ASSERT_EQ(d[4], static_cast<uint16_t>(autd3::driver::GainSTMMode::PhaseDutyFull));
  ASSERT_EQ(d[5], 0x00);
  ASSERT_EQ(d[6], 0xAB);
  ASSERT_EQ(d[7], 0x89);

  b->mode = GainSTMMode::PhaseFull;
  ASSERT_EQ(d[4], static_cast<uint16_t>(autd3::driver::GainSTMMode::PhaseFull));

  b->mode = GainSTMMode::PhaseHalf;
  ASSERT_EQ(d[4], static_cast<uint16_t>(autd3::driver::GainSTMMode::PhaseHalf));
}
