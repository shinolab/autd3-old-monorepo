// File: driver_test.cpp
// Project: driver
// Created Date: 20/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/06/2022
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

#include <autd3/driver/cpu/body.hpp>
#include <autd3/driver/cpu/datagram.hpp>
#include <autd3/driver/cpu/operation.hpp>
#include <autd3/driver/hardware.hpp>
#include <random>

#include "autd3/driver/fpga/defined.hpp"
#include "autd3/driver/utils.hpp"

using autd3::driver::CPUControlFlags;
using autd3::driver::FPGAControlFlags;

TEST(FPGATest, FPGAControlFlagsTest) {
  FPGAControlFlags flag(FPGAControlFlags::NONE);

  ASSERT_EQ(flag, FPGAControlFlags::NONE);

  flag.set(FPGAControlFlags::LEGACY_MODE);

  ASSERT_TRUE(flag != FPGAControlFlags::NONE);
  ASSERT_EQ(flag, FPGAControlFlags::LEGACY_MODE);

  flag.set(FPGAControlFlags::STM_MODE);
  flag.remove(FPGAControlFlags::LEGACY_MODE);

  ASSERT_TRUE(flag != FPGAControlFlags::LEGACY_MODE);
  ASSERT_EQ(flag, FPGAControlFlags::STM_MODE);
}

TEST(FPGATest, FPGAInfo) {
  using autd3::driver::FPGAInfo;

  FPGAInfo info(0);
  ASSERT_FALSE(info.is_thermal_assert());

  info = FPGAInfo(1);
  ASSERT_TRUE(info.is_thermal_assert());

  info = FPGAInfo(2);
  ASSERT_FALSE(info.is_thermal_assert());
}

TEST(HARDTest, is_missing_transducer) {
  using autd3::driver::is_missing_transducer;

  ASSERT_TRUE(is_missing_transducer(1, 1));
  ASSERT_TRUE(is_missing_transducer(2, 1));
  ASSERT_TRUE(is_missing_transducer(16, 1));

  ASSERT_FALSE(is_missing_transducer(0, 0));
  ASSERT_FALSE(is_missing_transducer(17, 0));
  ASSERT_FALSE(is_missing_transducer(17, 13));
}

TEST(CPUTest, STMFocus) {
  ASSERT_EQ(sizeof(autd3::driver::STMFocus), 8);

  constexpr auto max = static_cast<double>((1 << 17) - 1) * autd3::driver::POINT_STM_FIXED_NUM_UNIT;
  constexpr auto min = static_cast<double>(-(1 << 17)) * autd3::driver::POINT_STM_FIXED_NUM_UNIT;

  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_real_distribution dist(min, max);
  std::uniform_int_distribution dist_u8(0, 0xFF);

  const auto to = [](const uint64_t v) -> double {
    auto b = static_cast<uint32_t>(v & 0x0003fffful);
    b = (v & 0x20000) == 0 ? b : b | 0xfffc0000u;
    const auto xi = *reinterpret_cast<int32_t*>(&b);
    return static_cast<double>(xi) * autd3::driver::POINT_STM_FIXED_NUM_UNIT;
  };

  for (auto i = 0; i < 10000; i++) {
    const auto x = dist(engine);
    const auto y = dist(engine);
    const auto z = dist(engine);
    const auto shift = static_cast<uint8_t>(dist_u8(engine));

    const autd3::driver::STMFocus focus(x, y, z, shift);

    uint64_t v = 0;
    std::memcpy(&v, &focus, sizeof(autd3::driver::STMFocus));

    const auto xx = to(v);
    ASSERT_NEAR(xx, x, autd3::driver::POINT_STM_FIXED_NUM_UNIT / 2);

    v >>= 18;
    const auto yy = to(v);
    ASSERT_NEAR(yy, y, autd3::driver::POINT_STM_FIXED_NUM_UNIT / 2);

    v >>= 18;
    const auto zz = to(v);
    ASSERT_NEAR(zz, z, autd3::driver::POINT_STM_FIXED_NUM_UNIT / 2);

    v >>= 18;
    const auto s = static_cast<uint8_t>(v & 0xFF);
    ASSERT_EQ(s, shift);
  }
}

TEST(CPUTest, Body) {
  ASSERT_EQ(sizeof(autd3::driver::PointSTMBodyHead), 498);
  ASSERT_EQ(sizeof(autd3::driver::PointSTMBodyBody), 498);
  ASSERT_EQ(sizeof(autd3::driver::GainSTMBodyHead), 498);
  ASSERT_EQ(sizeof(autd3::driver::GainSTMBodyBody), 498);
  ASSERT_EQ(sizeof(autd3::driver::Body), 498);
}

TEST(CPUTest, TxDatagram) {
  autd3::driver::TxDatagram tx(10);

  ASSERT_EQ(tx.size(), 10);
  ASSERT_EQ(tx.effective_size(), 128 + 498 * 10);

  tx.num_bodies = 5;
  ASSERT_EQ(tx.effective_size(), 128 + 498 * 5);
}

TEST(CPUTest, RxDatagram) {
  autd3::driver::RxDatagram rx(10);

  ASSERT_FALSE(rx.is_msg_processed(1));

  rx[0].msg_id = 1;
  ASSERT_FALSE(rx.is_msg_processed(1));

  for (auto& msg : rx) msg.msg_id = 1;
  ASSERT_TRUE(rx.is_msg_processed(1));
  ASSERT_FALSE(rx.is_msg_processed(2));
}

TEST(CPUTest, CPUControlFlags) {
  CPUControlFlags flag(CPUControlFlags::NONE);

  ASSERT_EQ(flag, CPUControlFlags::NONE);

  flag.set(CPUControlFlags::MOD);

  ASSERT_TRUE(flag != CPUControlFlags::NONE);
  ASSERT_EQ(flag, CPUControlFlags::MOD);

  flag.set(CPUControlFlags::MOD_BEGIN);
  flag.remove(CPUControlFlags::MOD);

  ASSERT_TRUE(flag != CPUControlFlags::MOD);
  ASSERT_EQ(flag, CPUControlFlags::MOD_BEGIN);
}

TEST(CPUTest, Header) {
  ASSERT_EQ(sizeof(autd3::driver::ModHead), 124);
  ASSERT_EQ(sizeof(autd3::driver::ModBody), 124);
  ASSERT_EQ(sizeof(autd3::driver::SilencerHeader), 124);
  ASSERT_EQ(sizeof(autd3::driver::GlobalHeader), 128);
}

TEST(CPUTest, operation_clear) {
  autd3::driver::TxDatagram tx(10);

  clear(tx);

  ASSERT_EQ(tx.header().msg_id, autd3::driver::MSG_CLEAR);
  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(CPUTest, operation_null_header) {
  autd3::driver::TxDatagram tx(10);

  null_header(1, tx);

  ASSERT_EQ(tx.header().msg_id, 1);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::MOD, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::CONFIG_SILENCER, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::CONFIG_SYNC, 0);
  ASSERT_EQ(tx.header().size, 0);
  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(CPUTest, operation_null_body) {
  autd3::driver::TxDatagram tx(10);

  null_body(tx);

  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY, 0);
  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(CPUTest, operation_sync) {
  autd3::driver::TxDatagram tx(10);

  std::vector<uint16_t> cycle;
  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_int_distribution dist(0, 0xFFFF);
  cycle.reserve(autd3::driver::NUM_TRANS_IN_UNIT * 10);
  for (size_t i = 0; i < autd3::driver::NUM_TRANS_IN_UNIT * 10; i++) cycle.emplace_back(dist(engine));

  sync(1, cycle.data(), tx);

  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::MOD, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::CONFIG_SILENCER, 0);
  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::CONFIG_SYNC, 0);

  for (size_t i = 0; i < autd3::driver::NUM_TRANS_IN_UNIT * 10; i++)
    ASSERT_EQ(tx.bodies()[i / autd3::driver::NUM_TRANS_IN_UNIT].data[i % autd3::driver::NUM_TRANS_IN_UNIT], cycle[i]);

  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(CPUTest, operation_modulation) {
  autd3::driver::TxDatagram tx(10);

  uint8_t mod_data[4] = {0x00, 0x01, 0x02, 0x03};

  modulation(1, mod_data, 4, true, 2320, false, tx);

  ASSERT_EQ(tx.header().msg_id, 1);
  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::MOD, 0);
  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::MOD_BEGIN, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::MOD_END, 0);
  ASSERT_EQ(tx.header().size, 4);
  ASSERT_EQ(tx.header().mod_head().freq_div, 2320);
  ASSERT_EQ(tx.header().mod_head().data[0], 0x00);
  ASSERT_EQ(tx.header().mod_head().data[1], 0x01);
  ASSERT_EQ(tx.header().mod_head().data[2], 0x02);
  ASSERT_EQ(tx.header().mod_head().data[3], 0x03);

  mod_data[0] = 0x04;
  mod_data[1] = 0x05;
  mod_data[2] = 0x06;
  mod_data[3] = 0x07;

  modulation(0xFF, mod_data, 4, false, 5, true, tx);

  ASSERT_EQ(tx.header().msg_id, 0xFF);
  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::MOD, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::MOD_BEGIN, 0);
  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::MOD_END, 0);
  ASSERT_EQ(tx.header().size, 4);
  ASSERT_EQ(tx.header().mod_body().data[0], 0x04);
  ASSERT_EQ(tx.header().mod_body().data[1], 0x05);
  ASSERT_EQ(tx.header().mod_body().data[2], 0x06);
  ASSERT_EQ(tx.header().mod_body().data[3], 0x07);

  modulation(0xF0, nullptr, 0, false, 5, true, tx);

  ASSERT_EQ(tx.header().msg_id, 0xF0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::MOD, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::MOD_BEGIN, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::MOD_END, 0);
  ASSERT_EQ(tx.header().size, 0);

  ASSERT_THROW(modulation(1, mod_data, 2319, true, 5, false, tx), std::runtime_error);
}

TEST(CPUTest, operation_config_silencer) {
  autd3::driver::TxDatagram tx(10);

  config_silencer(1, 2088, 4, tx);

  ASSERT_EQ(tx.header().msg_id, 1);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::MOD, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::CONFIG_SYNC, 0);
  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::CONFIG_SILENCER, 0);
  ASSERT_EQ(tx.header().silencer_header().cycle, 2088);
  ASSERT_EQ(tx.header().silencer_header().step, 4);

  ASSERT_THROW(config_silencer(1, 2087, 4, tx), std::runtime_error);
}

TEST(CPUTest, normal_legacy_header) {
  autd3::driver::TxDatagram tx(10);

  normal_legacy_header(tx);

  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY, 0);
  ASSERT_NE(tx.header().fpga_flag.value() & FPGAControlFlags::LEGACY_MODE, 0);
  ASSERT_EQ(tx.header().fpga_flag.value() & FPGAControlFlags::STM_MODE, 0);

  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(CPUTest, operation_normal_legacy_body) {
  autd3::driver::TxDatagram tx(10);

  std::vector<autd3::driver::Drive> drives;
  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_real_distribution dist(0.0, 1.0);
  drives.reserve(autd3::driver::NUM_TRANS_IN_UNIT * 10);
  for (size_t i = 0; i < autd3::driver::NUM_TRANS_IN_UNIT * 10; i++) drives.emplace_back(autd3::driver::Drive{dist(engine), dist(engine), 4096});

  normal_legacy_body(drives, tx);

  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY, 0);

  for (size_t i = 0; i < autd3::driver::NUM_TRANS_IN_UNIT * 10; i++) {
    ASSERT_EQ(tx.bodies()[i / autd3::driver::NUM_TRANS_IN_UNIT].data[i % autd3::driver::NUM_TRANS_IN_UNIT] & 0xFF,
              autd3::driver::LegacyDrive::to_phase(drives[i]));
    ASSERT_EQ(tx.bodies()[i / autd3::driver::NUM_TRANS_IN_UNIT].data[i % autd3::driver::NUM_TRANS_IN_UNIT] >> 8,
              autd3::driver::LegacyDrive::to_duty(drives[i]));
  }

  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(CPUTest, operation_normal_header) {
  autd3::driver::TxDatagram tx(10);

  normal_header(tx);

  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY, 0);
  ASSERT_EQ(tx.header().fpga_flag.value() & FPGAControlFlags::LEGACY_MODE, 0);
  ASSERT_EQ(tx.header().fpga_flag.value() & FPGAControlFlags::STM_MODE, 0);

  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(CPUTest, operation_normal_duty_body) {
  autd3::driver::TxDatagram tx(10);

  std::vector<autd3::driver::Drive> drives;
  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_real_distribution dist(0.0, 1.0);
  drives.reserve(autd3::driver::NUM_TRANS_IN_UNIT * 10);
  for (size_t i = 0; i < autd3::driver::NUM_TRANS_IN_UNIT * 10; i++) drives.emplace_back(autd3::driver::Drive{dist(engine), dist(engine), 4096});

  normal_duty_body(drives, tx);

  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::IS_DUTY, 0);
  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY, 0);

  for (size_t i = 0; i < autd3::driver::NUM_TRANS_IN_UNIT * 10; i++)
    ASSERT_EQ(tx.bodies()[i / autd3::driver::NUM_TRANS_IN_UNIT].data[i % autd3::driver::NUM_TRANS_IN_UNIT], autd3::driver::Duty::to_duty(drives[i]));

  ASSERT_EQ(tx.num_bodies, 10);
}
TEST(CPUTest, operation_normal_phase_body) {
  autd3::driver::TxDatagram tx(10);

  std::vector<autd3::driver::Drive> drives;
  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_real_distribution dist(0.0, 1.0);
  drives.reserve(autd3::driver::NUM_TRANS_IN_UNIT * 10);
  for (size_t i = 0; i < autd3::driver::NUM_TRANS_IN_UNIT * 10; i++) drives.emplace_back(autd3::driver::Drive{dist(engine), dist(engine), 4096});

  normal_phase_body(drives, tx);

  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::IS_DUTY, 0);
  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY, 0);

  for (size_t i = 0; i < autd3::driver::NUM_TRANS_IN_UNIT * 10; i++)
    ASSERT_EQ(tx.bodies()[i / autd3::driver::NUM_TRANS_IN_UNIT].data[i % autd3::driver::NUM_TRANS_IN_UNIT],
              autd3::driver::Phase::to_phase(drives[i]));

  ASSERT_EQ(tx.num_bodies, 10);
}
TEST(CPUTest, operation_point_stm_header) {
  autd3::driver::TxDatagram tx(10);

  point_stm_header(tx);

  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::STM_BEGIN, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::STM_END, 0);
  ASSERT_NE(tx.header().fpga_flag.value() & FPGAControlFlags::STM_MODE, 0);
  ASSERT_EQ(tx.header().fpga_flag.value() & FPGAControlFlags::STM_GAIN_MODE, 0);

  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(CPUTest, operation_point_stm_body) {
  autd3::driver::TxDatagram tx(10);

  std::vector<autd3::driver::STMFocus> points_30;
  std::random_device seed_gen;
  std::mt19937 engine(seed_gen());
  std::uniform_real_distribution dist(-1000.0, 1000.0);
  std::uniform_int_distribution dist_u8(0, 0xFF);
  points_30.reserve(30);
  for (int i = 0; i < 30; i++)
    points_30.emplace_back(autd3::driver::STMFocus(dist(engine), dist(engine), dist(engine), static_cast<uint8_t>(dist_u8(engine))));

  std::vector<std::vector<autd3::driver::STMFocus>> points;
  points.reserve(10);
  for (int i = 0; i < 10; i++) points.emplace_back(points_30);

  constexpr uint32_t sp = 340 * 1024;

  point_stm_header(tx);
  point_stm_body(points, true, 3224, sp / 1024.0, false, tx);

  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY, 0);
  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::STM_BEGIN, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::STM_END, 0);

  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.bodies()[i].point_stm_head().data()[0], 30);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.bodies()[i].point_stm_head().data()[1], 3224);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.bodies()[i].point_stm_head().data()[2], 0);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.bodies()[i].point_stm_head().data()[3], sp & 0xFFFF);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.bodies()[i].point_stm_head().data()[4], sp >> 16);
  ASSERT_EQ(tx.num_bodies, 10);

  point_stm_header(tx);
  point_stm_body(points, false, 3224, sp / 1024.0, true, tx);

  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::STM_BEGIN, 0);
  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::STM_END, 0);

  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.bodies()[i].point_stm_head().data()[0], 30);
  ASSERT_EQ(tx.num_bodies, 10);

  point_stm_header(tx);
  point_stm_body({}, true, 3224, sp / 1024.0, false, tx);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::STM_BEGIN, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::STM_END, 0);
  ASSERT_NE(tx.header().fpga_flag.value() & FPGAControlFlags::STM_MODE, 0);
  ASSERT_EQ(tx.header().fpga_flag.value() & FPGAControlFlags::STM_GAIN_MODE, 0);
  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(CPUTest, operation_gain_stm_legacy_header) {
  autd3::driver::TxDatagram tx(10);

  gain_stm_legacy_header(tx);

  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::STM_BEGIN, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::STM_END, 0);
  ASSERT_NE(tx.header().fpga_flag.value() & FPGAControlFlags::LEGACY_MODE, 0);
  ASSERT_NE(tx.header().fpga_flag.value() & FPGAControlFlags::STM_MODE, 0);
  ASSERT_NE(tx.header().fpga_flag.value() & FPGAControlFlags::STM_GAIN_MODE, 0);

  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(CPUTest, operation_gain_stm_legacy_body) {
  autd3::driver::TxDatagram tx(10);

  std::vector<std::vector<autd3::driver::Drive>> drives_list;
  drives_list.reserve(5);
  for (int i = 0; i < 5; i++) {
    std::vector<autd3::driver::Drive> drives;
    std::random_device seed_gen;
    std::mt19937 engine(seed_gen());
    std::uniform_real_distribution dist(0.0, 1.0);
    drives.reserve(autd3::driver::NUM_TRANS_IN_UNIT * 10);
    for (size_t j = 0; j < autd3::driver::NUM_TRANS_IN_UNIT * 10; j++) drives.emplace_back(autd3::driver::Drive{dist(engine), dist(engine), 4096});
    drives_list.emplace_back(drives);
  }

  gain_stm_legacy_header(tx);
  gain_stm_legacy_body({}, true, 3224, false, autd3::driver::GainSTMMode::PhaseDutyFull, tx);
  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY, 0);
  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::STM_BEGIN, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::STM_END, 0);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.bodies()[i].gain_stm_head().data()[0], 3224);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.bodies()[i].gain_stm_head().data()[1], 0);
  ASSERT_EQ(tx.num_bodies, 10);

  gain_stm_legacy_header(tx);
  gain_stm_legacy_body({&drives_list[0]}, false, 3224, false, autd3::driver::GainSTMMode::PhaseDutyFull, tx);
  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::STM_BEGIN, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::STM_END, 0);
  for (size_t i = 0; i < autd3::driver::NUM_TRANS_IN_UNIT * 10; i++) {
    ASSERT_EQ(tx.bodies()[i / autd3::driver::NUM_TRANS_IN_UNIT].data[i % autd3::driver::NUM_TRANS_IN_UNIT] & 0xFF,
              autd3::driver::LegacyDrive::to_phase(drives_list[0][i]));
    ASSERT_EQ(tx.bodies()[i / autd3::driver::NUM_TRANS_IN_UNIT].data[i % autd3::driver::NUM_TRANS_IN_UNIT] >> 8,
              autd3::driver::LegacyDrive::to_duty(drives_list[0][i]));
  }
  ASSERT_EQ(tx.num_bodies, 10);

  gain_stm_legacy_header(tx);
  gain_stm_legacy_body({&drives_list[4]}, false, 3224, true, autd3::driver::GainSTMMode::PhaseDutyFull, tx);
  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::STM_BEGIN, 0);
  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::STM_END, 0);
  for (size_t i = 0; i < autd3::driver::NUM_TRANS_IN_UNIT * 10; i++) {
    ASSERT_EQ(tx.bodies()[i / autd3::driver::NUM_TRANS_IN_UNIT].data[i % autd3::driver::NUM_TRANS_IN_UNIT] & 0xFF,
              autd3::driver::LegacyDrive::to_phase(drives_list[4][i]));
    ASSERT_EQ(tx.bodies()[i / autd3::driver::NUM_TRANS_IN_UNIT].data[i % autd3::driver::NUM_TRANS_IN_UNIT] >> 8,
              autd3::driver::LegacyDrive::to_duty(drives_list[4][i]));
  }
  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(CPUTest, operation_gain_stm_normal_header) {
  autd3::driver::TxDatagram tx(10);

  gain_stm_normal_header(tx);

  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::STM_BEGIN, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::STM_END, 0);
  ASSERT_EQ(tx.header().fpga_flag.value() & FPGAControlFlags::LEGACY_MODE, 0);
  ASSERT_NE(tx.header().fpga_flag.value() & FPGAControlFlags::STM_MODE, 0);
  ASSERT_NE(tx.header().fpga_flag.value() & FPGAControlFlags::STM_GAIN_MODE, 0);

  ASSERT_EQ(tx.num_bodies, 0);
}

TEST(CPUTest, operation_gain_stm_normal_phase) {
  autd3::driver::TxDatagram tx(10);

  std::vector<std::vector<autd3::driver::Drive>> drives_list;
  drives_list.reserve(5);
  for (int i = 0; i < 5; i++) {
    std::vector<autd3::driver::Drive> drives;
    std::random_device seed_gen;
    std::mt19937 engine(seed_gen());
    std::uniform_real_distribution dist(0.0, 1.0);
    drives.reserve(autd3::driver::NUM_TRANS_IN_UNIT * 10);
    for (size_t j = 0; j < autd3::driver::NUM_TRANS_IN_UNIT * 10; j++) drives.emplace_back(autd3::driver::Drive{dist(engine), dist(engine), 4096});
    drives_list.emplace_back(drives);
  }

  gain_stm_normal_header(tx);
  gain_stm_normal_phase({}, true, 3224, autd3::driver::GainSTMMode::PhaseDutyFull, false, tx);
  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY, 0);
  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::STM_BEGIN, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::STM_END, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::IS_DUTY, 0);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.bodies()[i].gain_stm_head().data()[0], 3224);
  for (int i = 0; i < 10; i++) ASSERT_EQ(tx.bodies()[i].gain_stm_head().data()[1], 0);
  ASSERT_EQ(tx.num_bodies, 10);

  gain_stm_normal_header(tx);
  gain_stm_normal_phase(drives_list[0], false, 3224, autd3::driver::GainSTMMode::PhaseDutyFull, false, tx);
  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::STM_BEGIN, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::STM_END, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::IS_DUTY, 0);
  for (size_t i = 0; i < autd3::driver::NUM_TRANS_IN_UNIT * 10; i++)
    ASSERT_EQ(tx.bodies()[i / autd3::driver::NUM_TRANS_IN_UNIT].data[i % autd3::driver::NUM_TRANS_IN_UNIT],
              autd3::driver::Phase::to_phase(drives_list[0][i]));
  ASSERT_EQ(tx.num_bodies, 10);

  gain_stm_normal_header(tx);
  gain_stm_normal_phase(drives_list[4], false, 3224, autd3::driver::GainSTMMode::PhaseDutyFull, false, tx);
  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::STM_BEGIN, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::STM_END, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::IS_DUTY, 0);
  for (size_t i = 0; i < autd3::driver::NUM_TRANS_IN_UNIT * 10; i++)
    ASSERT_EQ(tx.bodies()[i / autd3::driver::NUM_TRANS_IN_UNIT].data[i % autd3::driver::NUM_TRANS_IN_UNIT],
              autd3::driver::Phase::to_phase(drives_list[4][i]));
  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(CPUTest, operation_gain_stm_normal_duty) {
  autd3::driver::TxDatagram tx(10);

  std::vector<std::vector<autd3::driver::Drive>> drives_list;
  drives_list.reserve(5);
  for (int i = 0; i < 5; i++) {
    std::vector<autd3::driver::Drive> drives;
    std::random_device seed_gen;
    std::mt19937 engine(seed_gen());
    std::uniform_real_distribution dist(0.0, 1.0);
    drives.reserve(autd3::driver::NUM_TRANS_IN_UNIT * 10);
    for (size_t j = 0; j < autd3::driver::NUM_TRANS_IN_UNIT * 10; j++) drives.emplace_back(autd3::driver::Drive{dist(engine), dist(engine), 4096});
    drives_list.emplace_back(drives);
  }

  gain_stm_normal_header(tx);
  gain_stm_normal_duty(drives_list[0], false, tx);
  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::STM_BEGIN, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::STM_END, 0);
  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::IS_DUTY, 0);
  for (size_t i = 0; i < autd3::driver::NUM_TRANS_IN_UNIT * 10; i++)
    ASSERT_EQ(tx.bodies()[i / autd3::driver::NUM_TRANS_IN_UNIT].data[i % autd3::driver::NUM_TRANS_IN_UNIT],
              autd3::driver::Duty::to_duty(drives_list[0][i]));
  ASSERT_EQ(tx.num_bodies, 10);

  gain_stm_normal_header(tx);
  gain_stm_normal_duty(drives_list[4], true, tx);
  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::WRITE_BODY, 0);
  ASSERT_EQ(tx.header().cpu_flag.value() & CPUControlFlags::STM_BEGIN, 0);
  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::STM_END, 0);
  ASSERT_NE(tx.header().cpu_flag.value() & CPUControlFlags::IS_DUTY, 0);
  for (size_t i = 0; i < autd3::driver::NUM_TRANS_IN_UNIT * 10; i++)
    ASSERT_EQ(tx.bodies()[i / autd3::driver::NUM_TRANS_IN_UNIT].data[i % autd3::driver::NUM_TRANS_IN_UNIT],
              autd3::driver::Duty::to_duty(drives_list[4][i]));
  ASSERT_EQ(tx.num_bodies, 10);
}

TEST(CPUTest, operation_force_fan) {
  autd3::driver::TxDatagram tx(10);

  force_fan(tx, true);
  ASSERT_NE(tx.header().fpga_flag.value() & FPGAControlFlags::FORCE_FAN, 0);

  force_fan(tx, false);
  ASSERT_EQ(tx.header().fpga_flag.value() & FPGAControlFlags::FORCE_FAN, 0);
}

TEST(CPUTest, operation_reads_fpga_info) {
  autd3::driver::TxDatagram tx(10);

  reads_fpga_info(tx, true);
  ASSERT_NE(tx.header().fpga_flag.value() & FPGAControlFlags::READS_FPGA_INFO, 0);

  reads_fpga_info(tx, false);
  ASSERT_EQ(tx.header().fpga_flag.value() & FPGAControlFlags::READS_FPGA_INFO, 0);
}

TEST(CPUTest, operation_cpu_version) {
  autd3::driver::TxDatagram tx(10);

  cpu_version(tx);
  ASSERT_EQ(tx.header().msg_id, autd3::driver::MSG_RD_CPU_VERSION);
  ASSERT_EQ(static_cast<uint8_t>(tx.header().cpu_flag.value()), autd3::driver::MSG_RD_CPU_VERSION);
}

TEST(CPUTest, operation_fpga_version) {
  autd3::driver::TxDatagram tx(10);

  fpga_version(tx);
  ASSERT_EQ(tx.header().msg_id, autd3::driver::MSG_RD_FPGA_VERSION);
  ASSERT_EQ(static_cast<uint8_t>(tx.header().cpu_flag.value()), autd3::driver::MSG_RD_FPGA_VERSION);
}

TEST(CPUTest, operation_fpga_functions) {
  autd3::driver::TxDatagram tx(10);

  fpga_functions(tx);
  ASSERT_EQ(tx.header().msg_id, autd3::driver::MSG_RD_FPGA_FUNCTION);
  ASSERT_EQ(static_cast<uint8_t>(tx.header().cpu_flag.value()), autd3::driver::MSG_RD_FPGA_FUNCTION);
}

TEST(UtilitiesTest, rem_euclid) {
  ASSERT_EQ(autd3::driver::rem_euclid(0, 256), 0);
  ASSERT_EQ(autd3::driver::rem_euclid(10, 256), 10);
  ASSERT_EQ(autd3::driver::rem_euclid(255, 256), 255);
  ASSERT_EQ(autd3::driver::rem_euclid(256, 256), 0);
  ASSERT_EQ(autd3::driver::rem_euclid(266, 256), 10);
  ASSERT_EQ(autd3::driver::rem_euclid(-10, 256), 246);
  ASSERT_EQ(autd3::driver::rem_euclid(-266, 256), 246);
}
